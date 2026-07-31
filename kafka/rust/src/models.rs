use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Created,
    Processed,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateOrderRequest {
    pub user_id: String,
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreatedEvent {
    pub event_id: Uuid,
    pub order_id: Uuid,
    pub user_id: String,
    pub amount: u64,
    pub event_type: String,
}

#[derive(Debug, Serialize)]
pub struct CreateOrderResponse {
    pub order_id: Uuid,
    pub status: OrderStatus,
}

#[derive(Debug, Serialize)]
pub struct OrderStatusResponse {
    pub order_id: Uuid,
    pub status: OrderStatus,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub mode: &'static str,
}
