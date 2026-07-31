use crate::models::HealthResponse;
use axum::{Json, Router, routing::get};

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route("/health", get(health))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "up",
        mode: runtime_mode(),
    })
}

fn runtime_mode() -> &'static str {
    #[cfg(feature = "kafka")]
    {
        "kafka"
    }

    #[cfg(not(feature = "kafka"))]
    {
        "in_memory"
    }
}
