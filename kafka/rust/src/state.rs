use crate::{
    config::AppConfig,
    consumer,
    models::{OrderCreatedEvent, OrderStatus},
    producer::OrderProducer,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

#[cfg(not(feature = "kafka"))]
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct AppState {
    pub orders: OrderStore,
    pub producer: OrderProducer,
}

pub type OrderStore = Arc<RwLock<HashMap<Uuid, OrderStatus>>>;

pub async fn build_app_state(config: &AppConfig) -> anyhow::Result<AppState> {
    #[cfg(not(feature = "kafka"))]
    let _ = config;

    let orders = Arc::new(RwLock::new(HashMap::new()));

    #[cfg(feature = "kafka")]
    {
        let producer = OrderProducer::new(&config.kafka_bootstrap_servers)?;
        let consumer = consumer::create_consumer(&config.kafka_bootstrap_servers)?;

        tokio::spawn(consumer::consume_kafka_orders(consumer, orders.clone()));

        Ok(AppState { orders, producer })
    }

    #[cfg(not(feature = "kafka"))]
    {
        let (sender, receiver) = mpsc::channel::<OrderCreatedEvent>(1024);
        tokio::spawn(consumer::consume_memory_orders(receiver, orders.clone()));

        tracing::warn!("running without Kafka; use `cargo run --features kafka` for real Kafka");

        Ok(AppState {
            orders,
            producer: OrderProducer::new(sender),
        })
    }
}
