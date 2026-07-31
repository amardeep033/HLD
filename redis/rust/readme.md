# Redis in Rust

This is a small Rust service for a 45-minute machine-coding round. It demonstrates the Redis patterns that are easiest to explain in an HLD interview:

- cache-aside reads
- TTL-based cache entries
- Redis counters
- fixed-window rate limiting
- graceful fallback when cache reads fail

The sample app is a tiny URL shortener.

```
POST /shorten
  ↓
fake database insert
  ↓
Redis SET cache:url:{code} long_url EX 300

GET /r/{code}
  ↓
Redis GET cache:url:{code}
  ↓ hit: return from Redis
  ↓ miss: read fake database, refresh Redis, return

GET /limited/{user_id}
  ↓
Redis INCR rate:user:{user_id}:fixed-window
  ↓
Redis EXPIRE key 60 on first request
```

---

## 1. Run Redis

Using Docker:

```bash
docker run --rm --name redis-hld -p 6379:6379 redis:7
```

Or use any local Redis running on:

```bash
redis://127.0.0.1/
```

---

## 2. Run the Rust App

```bash
cargo run
```

Optional:

```bash
REDIS_URL=redis://127.0.0.1/ cargo run
```

Server starts on:

```bash
http://127.0.0.1:8080
```

---

## 3. Files

```
src/main.rs        # app startup, shared AppState, route registration
src/config.rs      # TTL and rate-limit constants
src/handler.rs     # HTTP handlers
src/redis_store.rs # Redis commands hidden behind clean methods
src/db.rs          # fake durable database for the demo
src/model.rs       # request/response structs
curl.md            # commands to test the service
run.sh             # quick local run command
```

This is the structure to use in a machine-coding round when the interviewer expects production-ish organization:

```
HTTP layer     → handler.rs
Redis layer    → redis_store.rs
DB layer       → db.rs
Data contracts → model.rs
Startup        → main.rs
```

The key is that handlers do not build raw Redis commands directly. They call methods like `cache_url`, `get_url_from_cache`, and `allow_request`.

---

## 4. What to Explain in Interview

### Cache-aside

The app first checks Redis:

```rust
state.redis.get_url_from_cache(&code).await
```

If Redis has the value, return immediately.

If Redis misses, read from the database and then refresh Redis:

```rust
let long_url = state.db.find(&code).await;
state.redis.cache_url(&code, &long_url).await;
```

In this demo the database is an in-memory `HashMap`, but in the real system it would be Postgres, MySQL, DynamoDB, or another durable store.

### TTL

Cache entries use a 5-minute TTL:

```rust
SET cache:url:{code} long_url EX 300
```

This prevents stale values from living forever and keeps memory bounded.

### Counter

Redirect count uses:

```rust
INCR counter:redirects:{code}
```

`INCR` is atomic in Redis, so concurrent requests do not lose updates.

### Fixed-window rate limiter

The limiter uses:

```rust
INCR rate:user:{id}:fixed-window
EXPIRE rate:user:{id}:fixed-window 60
```

If the count is above the configured limit, the API returns `429 Too Many Requests`.

Interview caveat: fixed windows are simple but allow boundary bursts. For more accurate limiting, use a sliding-window sorted set or token bucket.

---

## 5. Common Improvements

- Replace `FakeDatabase` with a real DB repository.
- Use Lua for multi-command atomic rate limiting.
- Add TTL jitter to avoid cache avalanche.
- Add negative caching for missing short codes.
- Add metrics for cache hit/miss and Redis latency.
- Use Redis Cluster if one node cannot hold traffic or memory.

---

## 6. Crates Used

```
actix-web   # HTTP server
redis       # async Redis client
tokio       # async runtime
serde       # JSON
uuid        # short code generation for the demo
```
