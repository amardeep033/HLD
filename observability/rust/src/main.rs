use actix_web::{web, App, HttpServer};

mod handler;
mod metrics;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    metrics::init_tracing();

    HttpServer::new(|| {
        App::new()
            .route("/test", web::post().to(handler::test))
            .route("/health", web::get().to(handler::health))
            .route("/metrics", web::get().to(handler::metrics))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
