use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

use crate::AppState;
use crate::model::{
    CreateShortUrlRequest, CreateShortUrlResponse, ErrorResponse, HealthResponse,
    ResolveUrlResponse,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health))
        .route("/shorten", web::post().to(create_short_url))
        .route("/r/{code}", web::get().to(resolve_short_url))
        .route("/limited/{user_id}", web::get().to(limited_endpoint));
}

async fn create_short_url(
    state: web::Data<AppState>,
    body: web::Json<CreateShortUrlRequest>,
) -> impl Responder {
    if !body.long_url.starts_with("http://") && !body.long_url.starts_with("https://") {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "long_url must start with http:// or https://",
        });
    }

    let code = Uuid::new_v4().simple().to_string()[..8].to_string();

    state.db.insert(code.clone(), body.long_url.clone()).await;

    if let Err(err) = state.redis.cache_url(&code, &body.long_url).await {
        eprintln!("redis cache write failed: {err}");
    }

    HttpResponse::Ok().json(CreateShortUrlResponse {
        short_url: format!("http://localhost:8080/r/{code}"),
        code,
    })
}

async fn resolve_short_url(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let code = path.into_inner();

    match state.redis.get_url_from_cache(&code).await {
        Ok(Some(long_url)) => {
            let _ = state.redis.increment_redirect_count(&code).await;
            return HttpResponse::Ok().json(ResolveUrlResponse {
                code,
                long_url,
                source: "redis-cache",
            });
        }
        Ok(None) => {}
        Err(err) => {
            eprintln!("redis cache read failed: {err}");
        }
    }

    let Some(long_url) = state.db.find(&code).await else {
        return HttpResponse::NotFound().json(ErrorResponse {
            error: "short code not found",
        });
    };

    if let Err(err) = state.redis.cache_url(&code, &long_url).await {
        eprintln!("redis cache refresh failed: {err}");
    }
    let _ = state.redis.increment_redirect_count(&code).await;

    HttpResponse::Ok().json(ResolveUrlResponse {
        code,
        long_url,
        source: "database",
    })
}

async fn limited_endpoint(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();

    match state.redis.allow_request(&user_id).await {
        Ok(decision) if decision.allowed => HttpResponse::Ok()
            .insert_header(("X-RateLimit-Remaining", decision.remaining.to_string()))
            .json(HealthResponse { status: "up" }),
        Ok(_) => HttpResponse::TooManyRequests().json(ErrorResponse {
            error: "rate limit exceeded",
        }),
        Err(err) => {
            eprintln!("redis rate limit failed: {err}");
            HttpResponse::ServiceUnavailable().json(ErrorResponse {
                error: "rate limiter unavailable",
            })
        }
    }
}

async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse { status: "up" })
}
