use actix_web::{App, HttpServer, web};

mod config;
mod db;
mod handler;
mod model;
mod redis_store;

use crate::db::FakeDatabase;
use crate::redis_store::RedisStore;

#[derive(Clone)]
pub struct AppState {
    pub redis: RedisStore,
    pub db: FakeDatabase,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());

    let redis = RedisStore::new(&redis_url).await.map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            format!("failed to connect to Redis at {redis_url}: {err}"),
        )
    })?;

    let state = AppState {
        redis,
        db: FakeDatabase::default(),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(handler::routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
