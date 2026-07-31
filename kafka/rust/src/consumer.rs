use crate::{
    models::{OrderCreatedEvent, OrderStatus},
    state::OrderStore,
};

#[cfg(feature = "kafka")]
use {
    anyhow::Context,
    futures::StreamExt,
    rdkafka::{
        ClientConfig,
        consumer::{CommitMode, Consumer, StreamConsumer},
        message::Message,
    },
};

#[cfg(not(feature = "kafka"))]
use tokio::sync::mpsc;

#[cfg(feature = "kafka")]
const ORDERS_TOPIC: &str = "orders";

pub async fn process_event(event: OrderCreatedEvent, orders: &OrderStore) -> anyhow::Result<()> {
    // This in-memory update stands in for a database write in a machine coding round.
    orders
        .write()
        .await
        .insert(event.order_id, OrderStatus::Processed);

    tracing::info!(
        event_id = %event.event_id,
        order_id = %event.order_id,
        "order.created consumed"
    );

    Ok(())
}

#[cfg(not(feature = "kafka"))]
pub async fn consume_memory_orders(
    mut receiver: mpsc::Receiver<OrderCreatedEvent>,
    orders: OrderStore,
) {
    while let Some(event) = receiver.recv().await {
        if let Err(err) = process_event(event, &orders).await {
            tracing::error!(error = %err, "failed to process in-memory event");
        }
    }
}

#[cfg(feature = "kafka")]
pub fn create_consumer(bootstrap_servers: &str) -> anyhow::Result<StreamConsumer> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers)
        .set("group.id", "order-status-service")
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "false")
        .create()
        .context("failed to create Kafka consumer")?;

    consumer
        .subscribe(&[ORDERS_TOPIC])
        .context("failed to subscribe to orders topic")?;

    Ok(consumer)
}

#[cfg(feature = "kafka")]
pub async fn consume_kafka_orders(consumer: StreamConsumer, orders: OrderStore) {
    let mut stream = consumer.stream();

    while let Some(message) = stream.next().await {
        match message {
            Ok(message) => {
                if let Err(err) = handle_kafka_message(&message, &orders).await {
                    tracing::error!(error = %err, "failed to process Kafka message");
                    continue;
                }

                if let Err(err) = consumer.commit_message(&message, CommitMode::Async) {
                    tracing::error!(error = %err, "failed to commit Kafka offset");
                }
            }
            Err(err) => {
                tracing::error!(error = %err, "Kafka consumer error");
            }
        }
    }
}

#[cfg(feature = "kafka")]
async fn handle_kafka_message(
    message: &rdkafka::message::BorrowedMessage<'_>,
    orders: &OrderStore,
) -> anyhow::Result<()> {
    let payload = message.payload_view::<str>().context("missing payload")??;
    let event: OrderCreatedEvent = serde_json::from_str(payload).context("invalid event JSON")?;

    tracing::info!(
        partition = message.partition(),
        offset = message.offset(),
        "Kafka message received"
    );

    process_event(event, orders).await
}
