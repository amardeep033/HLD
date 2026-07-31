use crate::{
    models::{
        CreateOrderRequest, CreateOrderResponse, OrderCreatedEvent, OrderStatus,
        OrderStatusResponse,
    },
    state::AppState,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/orders", post(create_order))
        .route("/orders/:order_id", get(get_order))
}

async fn create_order(
    State(state): State<AppState>,
    Json(request): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<CreateOrderResponse>), (StatusCode, String)> {
    if request.user_id.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "user_id is required".to_string()));
    }

    if request.amount == 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "amount must be greater than 0".to_string(),
        ));
    }

    let event = OrderCreatedEvent {
        event_id: Uuid::new_v4(),
        order_id: Uuid::new_v4(),
        user_id: request.user_id,
        amount: request.amount,
        event_type: "order.created".to_string(),
    };

    state.producer.publish_order_created(&event).await?;

    state
        .orders
        .write()
        .await
        .insert(event.order_id, OrderStatus::Created);

    tracing::info!(
        event_id = %event.event_id,
        order_id = %event.order_id,
        user_id = %event.user_id,
        amount = event.amount,
        "order.created published"
    );

    Ok((
        StatusCode::ACCEPTED,
        Json(CreateOrderResponse {
            order_id: event.order_id,
            status: OrderStatus::Created,
        }),
    ))
}

async fn get_order(
    State(state): State<AppState>,
    Path(order_id): Path<Uuid>,
) -> Result<Json<OrderStatusResponse>, StatusCode> {
    let orders = state.orders.read().await;
    let status = orders
        .get(&order_id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(OrderStatusResponse { order_id, status }))
}
