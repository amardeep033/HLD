# Redis

---

## 1. What Redis Is

Redis is an in-memory data store. Most teams use it as a cache, but it is more than a cache: it can also act as a distributed counter store, rate limiter backend, queue, pub/sub bus, leaderboard, lock coordinator, and session store.

The important sentence:

> Redis is fast because most reads and writes happen in memory, and Redis provides simple data structures with atomic single-threaded command execution.

Redis is usually placed between application servers and the durable database.

```
Client
  ↓
API service
  ↓
Redis cache  ← fast path
  ↓
Database     ← source of truth
```

In HLD interviews, Redis is rarely the source of truth unless the problem is intentionally asking for an in-memory system. Treat the database as durable truth and Redis as a speed/control layer.

---

## 2. Why Redis Is Fast

### 2.1 In-memory storage

Redis keeps data in RAM. RAM access is much faster than disk access. That is the main reason Redis can handle high throughput and low latency.

### 2.2 Simple data structures

Redis operations are built around simple structures:
- strings
- hashes
- lists
- sets
- sorted sets
- streams
- bitmaps
- hyperloglogs

You are not running arbitrary joins or complex query plans. You are usually doing direct key lookups or small bounded operations.

### 2.3 Single-threaded command execution

Classic Redis executes commands on one main thread. This sounds slow, but it avoids locks inside the command execution path.

Because commands are processed one at a time, each command is atomic.

```
Client A: INCR user:123:requests
Client B: INCR user:123:requests

Redis processes them sequentially.
No lost update.
No application-level mutex needed.
```

Redis can still use additional threads for I/O in modern versions, but the mental model for interview answers is: commands are atomic because the core execution path is single-threaded.

---

## 3. Core Data Types

### 3.1 String

A string is the simplest Redis value. It can store text, JSON, bytes, counters, feature flags, tokens, and cached responses.

Common commands:

```
SET user:123 '{"id":123,"name":"Asha"}' EX 300
GET user:123
INCR page:42:views
SETNX lock:job:77 abc123
```

Use strings for:
- cache entries
- counters
- tokens
- simple locks
- serialized objects

### 3.2 Hash

A hash stores fields inside one Redis key.

```
HSET user:123 name Asha city Pune plan pro
HGET user:123 name
HGETALL user:123
HINCRBY user:123 login_count 1
```

Use hashes when you want to update individual fields without rewriting the full object.

Interview tradeoff:
- JSON string is simpler and maps cleanly to application models.
- Hash is better for partial updates.

### 3.3 List

A list is an ordered sequence.

```
LPUSH queue:emails job-1
RPOP queue:emails
```

Use lists for simple queues. But for production-grade queues, prefer Redis Streams, Kafka, SQS, or a real queue depending on reliability needs.

### 3.4 Set

A set stores unique values.

```
SADD post:42:likes user:1 user:2
SISMEMBER post:42:likes user:1
SCARD post:42:likes
```

Use sets for:
- uniqueness
- membership checks
- tags
- followers/following style relationships at moderate scale

### 3.5 Sorted Set

A sorted set stores unique members ordered by score.

```
ZADD leaderboard 980 user:1
ZADD leaderboard 1120 user:2
ZREVRANGE leaderboard 0 9 WITHSCORES
ZRANK leaderboard user:1
```

Use sorted sets for:
- leaderboards
- ranking
- top N items
- sliding-window rate limiters
- priority queues

### 3.6 Stream

Streams are append-only logs with consumer groups.

```
XADD order-events * order_id 123 status paid
XREAD COUNT 10 STREAMS order-events 0
```

Use streams when you need Redis-backed event consumption with replay and consumer groups. Still, for large durable event pipelines, Kafka is usually the stronger HLD answer.

---

## 4. Cache Patterns

### 4.1 Cache-aside

This is the most common interview pattern.

```
read(key):
  value = redis.get(key)
  if value exists:
      return value

  value = database.query(key)
  redis.set(key, value, ttl)
  return value
```

Application code owns the cache behavior. Redis does not automatically know how to load from the database.

Use when:
- reads are much more frequent than writes
- slight staleness is acceptable
- you want simple operational behavior

### 4.2 Write-through

On write, update cache and database together.

```
write(key, value):
  database.update(key, value)
  redis.set(key, value)
```

This keeps cache warmer, but writes are slower and failure handling becomes more careful.

### 4.3 Write-behind

Write to Redis first, then asynchronously flush to DB.

This is fast but risky. If Redis dies before flush, data can be lost unless you use persistence and careful replay.

Use this only when the interviewer accepts eventual durability or the system is designed around it.

### 4.4 Refresh-ahead

If a hot key is about to expire, refresh it in the background before users hit a miss.

Useful for very hot keys where DB misses are expensive.

---

## 5. TTL and Eviction

TTL means time to live. After the TTL expires, Redis deletes the key.

```
SET product:123 '{...}' EX 300
TTL product:123
EXPIRE product:123 300
```

Eviction is different. Eviction happens when Redis is out of memory and must remove keys.

Important distinction:
- TTL expiry: key dies because its time ended.
- Eviction: key dies because Redis needs memory.

Common eviction policies:

| Policy | Meaning |
|---|---|
| `noeviction` | reject writes when memory is full |
| `allkeys-lru` | evict least recently used keys from all keys |
| `volatile-lru` | evict least recently used keys only among keys with TTL |
| `allkeys-lfu` | evict least frequently used keys |
| `volatile-ttl` | evict keys with the nearest TTL |

For cache-heavy systems, `allkeys-lru` or `allkeys-lfu` is common. For systems where only cache keys have TTL and important keys do not, `volatile-lru` can make sense.

---

## 6. Cache Problems

### 6.1 Cache penetration

Requests repeatedly ask for data that does not exist.

```
GET user:does-not-exist
Redis miss
DB miss
repeat forever
```

Fixes:
- cache negative results with a short TTL
- use a Bloom filter for known valid IDs
- validate IDs before DB lookup

### 6.2 Cache breakdown

A very hot key expires. Many requests miss at the same time and hit the database.

Fixes:
- add random jitter to TTL
- use a per-key lock so only one request rebuilds the cache
- refresh hot keys in the background

### 6.3 Cache avalanche

Many keys expire at the same time, causing a DB spike.

Fixes:
- add TTL jitter
- stagger warmups
- use circuit breakers and rate limits
- keep critical hot keys refreshed

### 6.4 Stale cache

Redis has old data after the database changed.

Fixes:
- delete cache on write
- update cache on write
- use short TTLs
- publish invalidation events

The common production answer is: update database first, then delete cache.

```
write product:
  update database
  delete redis key product:123
```

The next read reloads fresh data.

---

## 7. Redis for Rate Limiting

Redis is excellent for rate limiting because increments and expiries are atomic enough for common designs.

### 7.1 Fixed window

Allow 100 requests per user per minute.

```
key = rate:user:123:2026-07-26T10:44
count = INCR key
if count == 1:
    EXPIRE key 60
if count > 100:
    reject
```

Pros:
- simple
- fast
- easy to implement

Cons:
- boundary burst problem. A user can make 100 requests at the end of one minute and 100 at the start of the next.

### 7.2 Sliding window log

Store timestamps in a sorted set.

```
ZREMRANGEBYSCORE key 0 now-window
ZADD key now request-id
ZCARD key
EXPIRE key window
```

Pros:
- accurate rolling window

Cons:
- stores one entry per request
- more expensive than fixed window

### 7.3 Token bucket

Tokens refill over time. A request consumes a token.

Use this when you want smooth traffic with controlled bursts. Implementation is more complex but is a strong HLD answer.

---

## 8. Redis for Distributed Locks

Basic lock:

```
SET lock:job:77 random-token NX PX 30000
```

Meaning:
- `NX`: only set if key does not exist
- `PX 30000`: lock expires in 30 seconds
- `random-token`: proves ownership

Release must check ownership before deleting:

```
if GET lock:job:77 == random-token:
    DEL lock:job:77
```

In production, use a Lua script for compare-and-delete so it is atomic.

Important interview caveat:

> Redis locks are useful, but distributed locking is subtle. For financial correctness or strict consistency, prefer database transactions, consensus systems, or a dedicated coordinator.

Mention Redlock only if asked. Do not oversell it as magic.

---

## 9. Persistence

Redis is in-memory, but it can persist data.

### 9.1 RDB snapshot

Redis periodically writes a full snapshot to disk.

Pros:
- compact
- faster restart
- less write overhead

Cons:
- can lose recent writes since the last snapshot

### 9.2 AOF

Append Only File logs every write command.

Pros:
- better durability
- can lose less data depending on fsync policy

Cons:
- larger files
- more write overhead

### 9.3 Interview answer

For cache use cases, persistence is often disabled or not critical. If Redis loses cache data, the database can rebuild it.

For queues, counters, sessions, and rate limit state, persistence may matter. Explain the tradeoff clearly.

---

## 10. Replication, Sentinel, and Cluster

### 10.1 Replication

Redis supports primary-replica replication.

```
Primary Redis
   ↓
Replica Redis
```

Writes go to primary. Replicas can serve reads, but replicas can lag.

### 10.2 Sentinel

Sentinel monitors Redis nodes and performs failover.

If primary dies:
1. Sentinel detects failure
2. Sentinel promotes a replica
3. Clients reconnect to the new primary

Use Sentinel when one Redis primary's memory is enough but you need high availability.

### 10.3 Cluster

Redis Cluster shards data across multiple primaries.

```
key → hash slot → Redis node
```

Use Cluster when:
- one Redis node cannot hold all data
- one Redis node cannot handle all traffic
- you need horizontal scaling

Tradeoffs:
- multi-key operations only work easily when keys are in the same hash slot
- client must understand redirects
- operational complexity increases

---

## 11. Consistency and Failure Modes

Redis is fast, but it does not remove distributed systems problems.

Common failure modes:
- Redis unavailable
- Redis slow due to memory pressure or hot keys
- stale cache after DB update
- replica lag
- cache stampede
- eviction of important keys
- network split between app and Redis

Design rules:
- have DB fallback for cache reads
- use timeouts
- set TTLs
- avoid unbounded keys
- do not put critical correctness only in Redis unless the system is designed for it
- protect hot keys

---

## 12. Key Design

Good Redis key names are predictable and namespaced.

```
cache:user:123
session:token:abc
rate:user:123:login
lock:payment:order-987
leaderboard:daily:2026-07-26
```

Rules:
- include the domain
- include the identifier
- include the purpose
- include version if schema changes
- avoid very long keys
- avoid unbounded high-cardinality metrics around Redis keys

For Redis Cluster, use hash tags when related keys must live together:

```
cart:{user-123}:items
cart:{user-123}:metadata
```

The part inside `{}` decides the hash slot.

---

## 13. HLD Examples

### 13.1 URL shortener

Redis use:
- cache short code → long URL
- rate limit URL creation per user/IP
- optionally count redirects with `INCR`

Database remains source of truth.

Read path:

```
GET short:abc123 from Redis
if hit:
    redirect
else:
    read DB
    SET short:abc123 long_url EX 86400
    redirect
```

Write path:

```
generate short code
insert into DB with unique constraint
delete/set Redis key
```

### 13.2 News feed

Redis use:
- cache user feed pages
- store recent post IDs in sorted sets
- fanout-on-write for celebrity/non-celebrity split

Watch out for:
- hot celebrity keys
- memory blowup if every user's feed is cached
- stale feeds

### 13.3 Chat

Redis use:
- presence: `presence:user:123` with TTL
- pub/sub for online fanout
- streams for lightweight event log

But durable messages should go to a database or log system. Redis Pub/Sub does not persist messages for offline users.

### 13.4 Leaderboard

Redis sorted set is the natural fit.

```
ZADD leaderboard:weekly score user_id
ZREVRANGE leaderboard:weekly 0 99 WITHSCORES
ZREVRANK leaderboard:weekly user_id
```

For very large systems, shard leaderboards by game/region/time period.

---

## 14. Rust Crates to Know

```
redis       # official-ish Redis client crate
tokio       # async runtime
actix-web   # HTTP server
serde       # JSON request/response
uuid        # request IDs / lock tokens
```

For a 45-minute machine-coding round, keep the design simple:
- one `RedisStore` wrapper
- clear methods like `get_cached_url`, `put_cached_url`, `allow_request`
- HTTP handlers with small request/response structs
- TTLs as constants
- graceful error responses

---

## 15. Interview Questions

1. Why is Redis fast?
2. Is Redis single-threaded?
3. What is cache-aside?
4. What is the difference between TTL expiry and eviction?
5. What are cache penetration, breakdown, and avalanche?
6. How do you design a rate limiter with Redis?
7. When would you use a sorted set?
8. How do Redis locks work?
9. What is the danger of using Redis as the source of truth?
10. When do you need Redis Cluster?
11. What happens if Redis is down?
12. How do you invalidate cache after DB writes?

---

## 16. One-Minute Summary

Redis is a fast in-memory data structure store. In HLD, use it for caching, rate limiting, counters, sessions, leaderboards, lightweight queues, pub/sub, and locks. Keep the database as source of truth unless the problem explicitly allows Redis-backed durability. Always discuss TTLs, eviction, stale data, cache stampede, Redis failure, and whether you need replication or cluster.
