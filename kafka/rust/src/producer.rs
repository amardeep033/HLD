use crate::models::OrderCreatedEvent;
use axum::http::StatusCode;

#[cfg(feature = "kafka")]
use {
    anyhow::Context,
    rdkafka::{
        ClientConfig,
        producer::{FutureProducer, FutureRecord},
    },
    std::time::Duration,
};

#[cfg(not(feature = "kafka"))]
use tokio::sync::mpsc;

#[cfg(feature = "kafka")]
const ORDERS_TOPIC: &str = "orders";

#[derive(Clone)]
pub struct OrderProducer {
    #[cfg(feature = "kafka")]
    inner: FutureProducer,

    #[cfg(not(feature = "kafka"))]
    sender: mpsc::Sender<OrderCreatedEvent>,
}

impl OrderProducer {
    #[cfg(feature = "kafka")]
    pub fn new(bootstrap_servers: &str) -> anyhow::Result<Self> {
        let inner = ClientConfig::new()
            .set("bootstrap.servers", bootstrap_servers)
            .set("message.timeout.ms", "5000")
            .set("acks", "all")
            .set("enable.idempotence", "true")
            .create()
            .context("failed to create Kafka producer")?;

        Ok(Self { inner })
    }

    #[cfg(not(feature = "kafka"))]
    pub fn new(sender: mpsc::Sender<OrderCreatedEvent>) -> Self {
        Self { sender }
    }

    pub async fn publish_order_created(
        &self,
        event: &OrderCreatedEvent,
    ) -> Result<(), (StatusCode, String)> {
        self.publish(event).await
    }

    #[cfg(feature = "kafka")]
    async fn publish(&self, event: &OrderCreatedEvent) -> Result<(), (StatusCode, String)> {
        let payload = serde_json::to_string(event)
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
        let key = event.order_id.to_string();

        self.inner
            .send(
                FutureRecord::to(ORDERS_TOPIC).key(&key).payload(&payload),
                Duration::from_secs(5),
            )
            .await
            .map_err(|(err, _)| {
                (
                    StatusCode::BAD_GATEWAY,
                    format!("Kafka publish failed: {err}"),
                )
            })?;

        Ok(())
    }

    #[cfg(not(feature = "kafka"))]
    async fn publish(&self, event: &OrderCreatedEvent) -> Result<(), (StatusCode, String)> {
        self.sender
            .send(event.clone())
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    }
}
