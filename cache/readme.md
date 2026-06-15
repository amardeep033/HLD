# Cache — HLD Interview Notes (SDE2)

---

## 1. Memory Hierarchy

```
CPU Registers → L1/L2/L3 Cache → RAM → Disk Cache → Disk (SSD/HDD)
```

Each level is slower but larger. Caching exploits **temporal locality** — recently accessed data is likely to be accessed again.

---

## 2. What Is a Cache?

A cache is a small, fast storage layer holding recently or frequently accessed data to reduce latency and load on slower backing stores (DB, disk, external API).

**Cache miss flow:** request → cache miss → fetch from source → store in cache → return data

---

## 3. Cache Locations

| Location | Examples | Notes |
|---|---|---|
| **External cache server** | Redis, Memcached | Shared across app servers; most common in distributed systems |
| **In-process / in-memory** | `IMemoryCache` (.NET), Guava Cache (Java) | Fast, zero network hop; not shared across instances |
| **CDN** | Cloudflare, Akamai | Caches static/semi-static assets at edge nodes |
| **Client-side** | Browser cache, mobile app cache | HTTP cache headers control behavior; limited server-side control |
| **DB query cache** | MySQL query cache (deprecated), ORM caches | Sits between app and DB |

---

## 4. Cache Architectures (Write/Read Strategies)

### Cache-Aside (Lazy Loading)
Most common pattern for application-managed caches (e.g., Redis + app server).

```
READ:  App → check cache → miss → App fetches DB → App writes to cache → return
WRITE: App writes to DB → invalidate/delete cache entry
```

- **Pro:** Only caches what's actually read; resilient to cache failure.
- **Con:** First read after a miss is slow (cache cold start); risk of stale data if invalidation is missed.

---

### Read-Through
Cache sits in front of DB and fetches data itself on a miss.

```
READ:  App → check cache → miss → Cache fetches DB → Cache stores → return
```

- **Pro:** App logic is simpler; CDNs work this way.
- **Con:** First read is still slow; cache layer needs DB access.

---

### Write-Through
Every write goes to cache **and** DB synchronously.

```
WRITE: App → write cache → write DB (sync) → return
```

- **Pro:** Cache always fresh; no stale read risk.
- **Con:** Write latency doubles; cache gets polluted with data that may never be read.

---

### Write-Back (Write-Behind)
Write goes to cache immediately; DB write is **async/deferred**.

```
WRITE: App → write cache → return (DB write happens later in background)
```

- **Pro:** Very fast writes.
- **Con:** Risk of data loss if cache crashes before DB is flushed. Not suitable for financial or critical data.

---

## 5. Cache Eviction Policies

| Policy | Evicts | When to Use |
|---|---|---|
| **LRU** (Least Recently Used) | Least recently accessed item | General purpose; default choice |
| **LFU** (Least Frequently Used) | Least accessed over time | When access frequency matters more than recency |
| **FIFO** | Oldest inserted item | Simple queues; rarely used in practice |
| **TTL** (Time To Live) | Item after a time threshold | Stale tolerance acceptable; session data, tokens |
| **Random** | Random item | Rarely useful; approximated in some distributed caches |

> Redis supports LRU, LFU, TTL, and random eviction via `maxmemory-policy`.

---

## 6. Cache Consistency

The core problem: cache and DB can diverge.

### Strategies

**Cache Invalidation on Write (most common)**
Delete the cache entry when the DB is updated. Next read repopulates with fresh data.
```
write to DB → DELETE cache key
```
- Simple and correct; slight latency on first read post-write.
- Risk: race condition between write + delete and a concurrent read (see: *double delete* or *versioned keys* pattern for mitigation).

**TTL-based Expiry**
Let stale data live for a bounded time window. Suitable when eventual consistency is acceptable (feeds, metrics, dashboards).

**Write-Through (always fresh)**
Cache is updated on every write — no staleness, but write performance cost.

**Versioned Cache Keys**
On schema/data change, write to a new key version (e.g., `user:42:v2`). Old keys naturally expire. Useful for deployments.

---

### Race Condition: Read-Then-Write (Stale Set)

```
T1: cache miss → fetch from DB (gets value = 10)
T2: DB updated to 20 → cache deleted
T1: writes stale value 10 back to cache  ← problem
```

Mitigation: compare-and-set, short TTLs, or the *delete-on-write* pattern with an extra delayed delete.

---

## 7. Cache Stampede (Thundering Herd)

**Problem:** A popular cache key expires. Dozens/hundreds of concurrent requests all miss and simultaneously hit the DB.

### Mitigations

| Technique | How It Works |
|---|---|
| **Request coalescing / mutex lock** | First miss acquires a lock and fetches; others wait. Only one DB query per miss. |
| **Probabilistic early recomputation** | Start refreshing the cache slightly before TTL expires (cache warming), so it's never truly cold. |
| **Jitter / random TTL offset** | Add randomness to TTL so keys don't all expire at the same time (`TTL = base + rand(0, 30s)`). |
| **Background refresh** | Serve stale data immediately; refresh asynchronously in the background. |

---

## 8. Hot Keys

**Problem:** A small number of keys receive disproportionate traffic (e.g., a trending post, a celebrity profile). This overwhelms the specific cache node holding that key.

### Mitigations

| Technique | How It Works |
|---|---|
| **Key replication / sharding** | Store the same value under multiple keys (`hot_key_1`, `hot_key_2`, ...) and load-balance reads across them. |
| **Local in-process cache fallback** | Cache extremely hot values in-process (L1 cache) to avoid hitting Redis at all. Short TTL (1–5s). |
| **Read replicas** | Route reads to Redis replicas; writes still go to master. |
| **Rate limiting** | Throttle requests to specific keys to prevent abuse. |

---

## 9. Cache Penetration

**Problem:** Requests for keys that **don't exist** in either cache or DB (e.g., invalid IDs). Every request is a miss → DB hit. Can be used as a DoS vector.

### Mitigations

- **Cache null/empty results** with a short TTL so repeated lookups don't reach DB.
- **Bloom filter** in front of the cache — if the key definitely doesn't exist, reject early without any cache/DB lookup.

---

## 10. Cache Avalanche

**Problem:** A large batch of cache keys expire at the same time → massive simultaneous DB load.

**Distinct from stampede:** stampede is one hot key; avalanche is many keys expiring together.

### Mitigations

- **Staggered TTLs:** Add random jitter to expiration times on write.
- **Persistent cache with warm-up:** Pre-populate cache after restarts.
- **Circuit breaker:** If DB load spikes, return stale data or a degraded response rather than cascading failure.

---

## 11. Quick Reference: Common Interview Scenarios

| Scenario | Recommended Pattern |
|---|---|
| User profile / session data | Cache-Aside + TTL |
| Product catalog (read-heavy) | Read-Through + long TTL |
| Shopping cart / order (write-critical) | Write-Through or skip cache |
| News feed / recommendations | Cache-Aside + short TTL + accept eventual consistency |
| Rate limiting counters | Redis with atomic INCR + TTL |
| Distributed lock | Redis `SET NX PX` (Redlock for HA) |
| Global config / feature flags | In-process cache + background refresh |

---

## 13. Cache Warming (Preloading)

**Problem:** After a Redis restart or cold deployment, the cache is empty. Every request misses → DB gets hammered → high latency / potential outage.

**Solution:** Proactively populate the cache with hot data *before* traffic arrives.

```
Deployment
    ↓
Identify hot keys (query DB / analytics)
    ↓
Bulk-write to Redis
    ↓
Open to traffic
```

### Warming Strategies

| Strategy | How | When |
|---|---|---|
| **Eager / startup warm-up** | A background job loads hot keys on app start or after Redis restart | Deployments, cache restarts |
| **Offline precomputation** | Batch job runs periodically (e.g., every hour) and refreshes top-N keys | ML outputs, leaderboards, trending feeds |
| **Traffic replay** | Replay recent prod request logs against the new cache | Blue/green deploys |
| **Gradual rollout** | Route a small % of traffic to the new instance first; it warms up naturally | Large-scale cache cluster replacements |

### What to Warm

- Top 1000 most-read products / videos
- User configs and feature flags
- ML model inference outputs (expensive to compute)
- Auth tokens / session data for active users

### Interview Q&A

> **"What happens when Redis restarts?"**
> Cache goes cold. All requests are cache misses → DB load spikes → latency increases. To mitigate: enable Redis persistence (RDB/AOF) so data survives restarts, AND implement a warm-up job as a safety net for cases where persistence isn't sufficient (e.g., new cluster, data migration).

> **"How do you decide what to warm?"**
> Use access logs or analytics to identify the top-N keys by request frequency. Only warm what's actually hot — warming too much wastes memory and delays startup.

---

## 14. Multi-Level Caching (L1 + L2)

Standard pattern at scale (>100k RPS). Avoids the Redis network hop for the hottest data by adding an in-process (L1) cache in front of Redis (L2).

```
Request
   ↓
L1: In-process cache (in-memory, per instance)
   ↓ miss
L2: Redis (shared, distributed)
   ↓ miss
Database
```

### Characteristics

| | L1 (In-Process) | L2 (Redis) |
|---|---|---|
| Latency | ~microseconds (no network) | ~1–5ms (network hop) |
| Shared across instances | ❌ No | ✅ Yes |
| Size | Small (MBs, bounded by heap) | Large (GBs) |
| Eviction | LRU / TTL (short, e.g. 1–5s) | LRU / LFU / TTL |
| Consistency challenge | High (each instance has its own copy) | Lower (single source of truth) |

### Consistency Problem with L1

Because each app instance has its own L1 cache, a write on instance A doesn't automatically invalidate instance B's L1 copy.

**Mitigations:**
- Use **very short TTLs** on L1 (1–5 seconds) — stale window is small and bounded.
- Use **pub/sub invalidation**: on write, publish a cache invalidation event to Redis pub/sub; all instances subscribe and evict their local copy.
- Only cache **immutable or slowly-changing data** in L1 (e.g., feature flags, config, product metadata).

### When to Use

- Traffic > ~100k RPS on a hot path
- Redis latency is measurable bottleneck (usually after profiling)
- Data is read-heavy with low write frequency

### Real-World Examples

- **Netflix:** EVCache (L2, Memcached) + local in-process cache (L1) for device/session data
- **Facebook:** Memcached (L2) + per-machine local cache for social graph edges
- **Google:** Multiple cache tiers in front of Bigtable / Spanner

---

## 12. Redis vs Memcached (Quick Comparison)

| | Redis | Memcached |
|---|---|---|
| Data structures | Strings, Lists, Sets, Hashes, Sorted Sets, Streams | Strings only |
| Persistence | RDB snapshots + AOF | None |
| Replication | Yes (primary-replica) | No |
| Cluster support | Redis Cluster | Third-party sharding only |
| Lua scripting | Yes | No |
| Use when | Complex data, persistence needed, pub/sub | Simple KV, max throughput, multi-threaded |