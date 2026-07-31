use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateShortUrlRequest {
    pub long_url: String,
}

#[derive(Serialize)]
pub struct CreateShortUrlResponse {
    pub code: String,
    pub short_url: String,
}

#[derive(Serialize)]
pub struct ResolveUrlResponse {
    pub code: String,
    pub long_url: String,
    pub source: &'static str,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: &'static str,
}

pub struct RateLimitDecision {
    pub allowed: bool,
    pub remaining: i64,
}
