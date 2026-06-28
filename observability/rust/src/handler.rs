use actix_web::{web, HttpResponse};
use prometheus::{Encoder, TextEncoder};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use uuid::Uuid;

use crate::metrics::{http_duration, http_requests};

// ─── POST /test ──────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct TestRequest {
    value: i32,
}

#[derive(Serialize)]
struct TestResponse {
    result: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<&'static str>,
}

pub async fn test(body: web::Json<TestRequest>) -> HttpResponse {
    let request_id = Uuid::new_v4().to_string();
    let start = Instant::now();

    let root = tracing::info_span!(
        "http_request",
        request_id = %request_id,
        endpoint   = "/test",
    );
    let _enter = root.enter();

    tracing::info!("request received");

    let value: Option<i32> = {
        let _s = tracing::info_span!("validate", value = body.value).entered();
        if body.value < 1 || body.value > 10 { None } else { Some(body.value) }
    };

    if value.is_none() {
        let elapsed = start.elapsed().as_secs_f64();
        http_requests().with_label_values(&["/test", "400"]).inc();
        http_duration().with_label_values(&["/test"]).observe(elapsed);
        tracing::warn!(value = body.value, "validation failed");
        tracing::info!(status = 400, "request completed");
        return HttpResponse::BadRequest()
            .insert_header(("X-Request-Id", request_id.as_str()))
            .json(TestResponse { result: "error", reason: Some("value must be between 1 and 10") });
    }

    let value = value.unwrap();

    let accepted = {
        let _s = tracing::info_span!("business_logic", value).entered();
        value % 2 != 0
    };

    if accepted {
        tracing::info!(value, accepted, "business logic completed");
    } else {
        tracing::warn!(value, accepted, "business logic completed");
    }

    let elapsed = start.elapsed().as_secs_f64();
    http_duration().with_label_values(&["/test"]).observe(elapsed);

    if accepted {
        http_requests().with_label_values(&["/test", "200"]).inc();
        tracing::info!(status = 200, "request completed");
        HttpResponse::Ok()
            .insert_header(("X-Request-Id", request_id.as_str()))
            .json(TestResponse { result: "ok", reason: None })
    } else {
        http_requests().with_label_values(&["/test", "422"]).inc();
        tracing::info!(status = 422, "request completed");
        HttpResponse::UnprocessableEntity()
            .insert_header(("X-Request-Id", request_id.as_str()))
            .json(TestResponse { result: "error", reason: Some("even numbers are rejected") })
    }
}

// ─── GET /health ─────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse { status: "up" })
}

// ─── GET /metrics ─────────────────────────────────────────────────────────────

pub async fn metrics() -> HttpResponse {
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();
    encoder.encode(&prometheus::gather(), &mut buffer).unwrap();
    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(buffer)
}
