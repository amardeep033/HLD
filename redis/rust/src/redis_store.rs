use redis::{AsyncCommands, RedisError, aio::ConnectionManager};

use crate::config::{RATE_LIMIT_MAX_REQUESTS, RATE_LIMIT_SECONDS, URL_CACHE_TTL_SECONDS};
use crate::model::RateLimitDecision;

#[derive(Clone)]
pub struct RedisStore {
    manager: ConnectionManager,
}

impl RedisStore {
    pub async fn new(redis_url: &str) -> Result<Self, RedisError> {
        let client = redis::Client::open(redis_url)?;
        let manager = client.get_connection_manager().await?;
        Ok(Self { manager })
    }

    pub async fn get_url_from_cache(&self, code: &str) -> Result<Option<String>, RedisError> {
        let mut conn = self.manager.clone();
        conn.get(cache_key(code)).await
    }

    pub async fn cache_url(&self, code: &str, long_url: &str) -> Result<(), RedisError> {
        let mut conn = self.manager.clone();
        let _: () = conn
            .set_ex(cache_key(code), long_url, URL_CACHE_TTL_SECONDS)
            .await?;
        Ok(())
    }

    pub async fn increment_redirect_count(&self, code: &str) -> Result<i64, RedisError> {
        let mut conn = self.manager.clone();
        conn.incr(counter_key(code), 1).await
    }

    pub async fn allow_request(&self, identity: &str) -> Result<RateLimitDecision, RedisError> {
        let mut conn = self.manager.clone();
        let key = rate_key(identity);

        let count: i64 = conn.incr(&key, 1).await?;
        if count == 1 {
            let _: bool = conn.expire(&key, RATE_LIMIT_SECONDS).await?;
        }

        Ok(RateLimitDecision {
            allowed: count <= RATE_LIMIT_MAX_REQUESTS,
            remaining: (RATE_LIMIT_MAX_REQUESTS - count).max(0),
        })
    }
}

fn cache_key(code: &str) -> String {
    format!("cache:url:{code}")
}

fn counter_key(code: &str) -> String {
    format!("counter:redirects:{code}")
}

fn rate_key(identity: &str) -> String {
    format!("rate:user:{identity}:fixed-window")
}
