# Rate Limiter — Complete Request Trace

> discussion answer for: _"Trace a request through a scaled, layered rate limiter."_

---

## Prerequisites

Before any request arrives, the system has two sources of truth set up.

**Config file** — loaded into gateway memory at startup:

```
ip_limit:               100 req/min
user_limit:             1000 req/min
api_limit GET:/search:  100 req/min
api_limit POST:/upload: 10 req/min
tier premium GET:/search: 500 req/min
```

**DB / Redis config store** — queried on cache miss:

```
user_id  → tier (free / premium)
user_id  → account status
api_id   → cost category
tenant_id → quota override
```

Policy is cached in gateway memory at startup. On the hot path, no DB or network call is needed to resolve limits.

---

## Step 0 — Client sends request

Request hits the nearest **API gateway** (load balanced, multi-region).

Client-side limiting is optional and cannot be trusted. All enforcement is server-side.
Client-side may have rate limiter in their browser or app to provide early feedback and reduce unnecessary network calls, but it’s not a substitute for server-side checks. Also, on restart, client-side counters reset — so it can’t be the source of truth for enforcement.

---

## Step 1 — API gateway extracts identity

**Where:** API gateway (in-process)

The gateway reads from the incoming request:

- `ip` — always available
- `userId` — from auth token, if present
- `apiId` — normalized route, e.g. `GET:/v1/search?q=test` → `GET:/v1/search`
- `tier` — resolved from local cache using `userId`

---

## Step 2 — Load policy from local memory cache

**Where:** Gateway in-memory cache

Resolve which limits apply to this request — IP limit, user limit, per-API limit — from the config cached at startup.

No network call. This keeps the limiter off the hot path for policy reads.

Cache miss → fetch from Redis config store → repopulate local cache.

---

## Step 3 — Layer 1: IP check

**Where:** Redis cluster, IP shard

Cheapest check. Runs first. No auth required — protects against bots and unauthenticated abuse.

In the below key: rl and ip are static prefixes; window is the time bucket (e.g. 1m for 1 minute); ip_hash is a hash of the IP address to anonymize it and distribute keys evenly.
**Key:** `rl:ip:{window}:{ip_hash}`   => e.g. `rl:ip:1m:ab12cd34` or `rl:ip:1d:ab12cd34` : different windows (1 minute, 1 day) for different use cases.
**Stores:** integer counter with TTL = window size  
**Shard:** `hash(ip) % N` → routes to one specific Redis node

| Result      | Action                 |
| ----------- | ---------------------- |
| Under limit | Continue to Layer 2    |
| Over limit  | Return 429 immediately |

---

## Step 4 — Layer 2: User global check

**Where:** Redis cluster, user shard

Only for authenticated requests. Enforces total account-level usage across all endpoints. Prevents a user from spreading traffic across many routes to bypass limits.

**Key:** `rl:user:{window}:{user_id}`  
**Stores:** integer counter with TTL = window size  
**Shard:** `hash(userId) % N` → same shard strategy as Layer 3 (intentional — see note below)

| Result      | Action                 |
| ----------- | ---------------------- |
| Under limit | Continue to Layer 3    |
| Over limit  | Return 429 immediately |

---

## Step 5 — Layer 3: Per-API per-user check

**Where:** Redis cluster, user shard

Only runs if the endpoint has a dedicated rule in config. Most precise business-level control — treats cheap and expensive APIs differently.

**Key:** `rl:user_api:{window}:{user_id}:{api_id}`  
**Stores:** integer counter with TTL = window size  
**Shard:** `hash(userId) % N`

> **Why shard by `userId` and not `apiId`?**  
> Sharding by `apiId` would create hot nodes on popular routes like `/search`.  
> Sharding by `userId` spreads traffic evenly. It also means all three counters for the same user (Layers 2 and 3) land on the same Redis node — so a multi-layer check is a single-node operation, not a cross-shard scatter-gather.

| Result      | Action                 |
| ----------- | ---------------------- |
| Under limit | Allow request          |
| Over limit  | Return 429 immediately |

---

## Step 6 — Counter increment

**Where:** Redis cluster

When a layer passes, its counter is incremented atomically. TTL is set to the window size — Redis auto-expires old counters, no manual cleanup needed.

---

## Step 7 — All layers pass → forward to backend

Request is forwarded to the backend service.

Response includes rate limit headers, taken from the most restrictive layer that applied:

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 43
X-RateLimit-Reset: 1748082660
```

---

## Step 8 — Any layer fails → 429

Returned immediately. Remaining layers are skipped.

```
HTTP 429 Too Many Requests
Retry-After: 30
X-RateLimit-Limit: 10
X-RateLimit-Remaining: 0
```

---

## Bonus: Senior-level follow-ups

**What if Redis goes down?**  
Depends on the endpoint — configured per API.

| Endpoint       | Strategy                      |
| -------------- | ----------------------------- |
| Payments       | Fail closed — reject requests |
| Login          | Fail closed                   |
| Public content | Fail open — allow requests    |

**Hot key problem**  
If one user generates extreme traffic, their shard becomes a bottleneck. Mitigation: stripe the key into N sub-keys (e.g. `rl:user:{window}:{user_id}:0..3`) and sum the stripes during the check. Only needed at very high scale or during abuse events.

**Multi-region**  
Each region enforces its own limits locally (low latency). Global quota sync happens asynchronously. This gives approximate global enforcement — acceptable for most APIs, not for strict financial quotas.

Each layer is ordered cheapest-first. IP check needs no auth and no user lookup — it blocks bots before you do any real work. User check enforces fairness. Per-API check enforces cost control. Failing fast at the cheapest layer saves Redis reads on the expensive ones.