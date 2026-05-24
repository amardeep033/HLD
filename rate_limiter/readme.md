# 1. Initial Clarification Questions

## 1.1 Is this rate limiter for API calls, DB calls, or internal service calls?

### 1.1.1 Public API Rate Limiter

- Usually server-side
- Protects backend services
- Most common interview scenario
- Flow: Client -> API Gateway / Middleware -> Services

### 1.1.2 Internal Microservice Rate Limiter

- Service-to-service protection
- Prevent noisy neighbors
- Often handled via sidecars/service mesh

### 1.1.3 DB Query Limiter

- Usually closer to client/service layer
- Protects database connections
- Often connection-pool based

---

## 1.2 Should rate limiting happen on client side or server side?

### 1.2.1 Client Side

Pros:

- reduces unnecessary traffic
- better UX
- avoids server overload

Cons:

- cannot be trusted
- easy to bypass
- malicious clients ignore it

### 1.2.2 Server Side

Pros:

- authoritative
- secure
- centralized control

Cons:

- extra infrastructure
- additional latency

### 1.2.3 Best Real-World Answer

Usually BOTH:

- client-side for optimization
- server-side for enforcement

---

## 1.3 What should be the limiting dimension?

### 1.3.1 Possible Dimensions

| Dimension    | Use Case             | Problem (explained)                                                                                                   |
| ------------ | -------------------- | --------------------------------------------------------------------------------------------------------------------- |
| IP           | Anonymous traffic    | Multiple users can share the same IP (NAT/proxy), so one abusive user may accidentally block everyone behind that IP. |
| User ID      | Authenticated APIs   | Rate limiting only works after login/authentication, so unauthenticated abuse cannot be controlled.                   |
| API Key      | SaaS/public APIs     | If the API key gets leaked or shared, attackers can consume another customer’s quota.                                 |
| Tenant ID    | Multi-tenant systems | One noisy tenant can exhaust shared resources and impact all users under that tenant.                                 |
| Device ID    | Mobile apps          | Device IDs can often be reset, cloned, or spoofed by attackers to bypass limits.                                      |
| Region       | Geo protection       | Region-level limiting is too broad and may throttle legitimate users from the same geography.                         |
| Endpoint/API | Expensive APIs       | Requires maintaining separate limits and metadata for every endpoint, increasing system complexity.                   |

### 1.3.2 Strong Answer

Use layered limiting:
Example:

1. IP-based fallback
2. User-based limiting
3. API-specific limits
4. Tenant-tier quotas

---

# 2. Architecture Questions

## 2.1 Should this be embedded in services or centralized?

### 2.1.1 Option 1 — Embedded In Every Service

Client -> Service A (Limiter)
Client -> Service B (Limiter)

Pros:

- simple for small systems
- no extra network hop

Cons:

- duplicated logic
- inconsistent policies
- hard to manage globally

### 2.1.2 Option 2 — Centralized Gateway/Middleware

Client -> API Gateway -> Rate Limiter -> Services

Pros:

- centralized control
- reusable
- consistent policies
- easier monitoring

Cons:

- possible bottleneck
- additional latency
- single point of failure if badly designed

### 2.1.3 Strong Production Answer

For microservices:

- centralized enforcement is usually preferred

Examples:

- Envoy
- NGINX
- Kong
- AWS API Gateway

---

# 3. Requirement Deep Dive Questions

## 3.1 Same limit for all APIs?

### 3.1.1 Same Global Limit

Example:

1000 req/min for every endpoint

Pros:

- simple

Cons:

- expensive APIs can still overload system

---

### 3.1.2 Per API Limits

Example:
/search -> 100 req/sec
/upload -> 10 req/sec
/status -> 1000 req/sec

Pros:

- fine-grained control

Cons:

- more metadata/configuration

---

### 3.1.3 Per Tier + Per API

Example:

Free user:
/search -> 50/min

Premium user:
/search -> 500/min

This is usually the strongest answer.

---

## 3.2 What scale are we designing for?

This question is VERY important.

Because algorithm + storage changes completely.

### 3.2.1 Small Scale

100 req/sec
single server

Possible solution:

- in-memory dictionary

Request → app server checks local hashmap counter → allow/block immediately.

---

### 3.2.2 Medium Scale

10k req/sec
multiple services

Possible solution:

- Redis
- distributed cache

Request → service queries shared Redis counter → atomic increment/check → allow/block consistently across instances.

---

### 3.2.3 Large Scale

1M+ req/sec
multi-region

Need:

- sharding
- distributed Redis
- local caching
- approximate counting
- async replication

Request → nearest regional limiter/local cache → sharded distributed Redis/global quota sync → async replication across regions → allow/block.

---

# 4. Choosing Rate Limiting Algorithms

| Algorithm              | Detail                                                  | Pros / When to use                                             | Cons / When NOT to use                                          |
| ---------------------- | ------------------------------------------------------- | -------------------------------------------------------------- | --------------------------------------------------------------- |
| Fixed Window           | Single counter per fixed time window                    | Very simple, fast, cheap; good for low-scale/simple APIs       | Burst problem at window boundary; not ideal for strict fairness |
| Sliding Window Log     | Store timestamp of every request                        | Very accurate and fair; good for security-sensitive APIs       | High memory/storage usage at scale                              |
| Sliding Window Counter | Combine previous + current window proportionally        | Better smoothing than fixed window with lower memory than logs | Slight approximation/inaccuracy                                 |
| Token Bucket           | Tokens refill at constant rate; requests consume tokens | Allows controlled bursts; widely used for APIs/networking      | Slightly more complex refill logic                              |
| Leaky Bucket           | Requests leave queue at fixed rate                      | Smooth outgoing traffic; protects downstream systems           | Can increase latency/drop bursts                                |

---

## 4.1 Strong Interview-Level Answer

### 4.1.1 If accuracy matters

Use:

- sliding window log

### 4.1.2 If scale matters

Use:

- rolling buckets
- token bucket

### 4.1.3 If burst handling matters

Use:

- token bucket

### 4.1.4 If smoothing matters

Use:

- leaky bucket

### 4.1.5 If simplicity matters

Use:

- fixed window

---

# 5. Storage Discussion

## 5.1 Why Redis?

- in-memory
- low latency
- atomic operations
- distributed
- TTL support
- Lua scripts
- sorted sets/hashes

Redis is one of the most common answers.

---

## 5.2 Redis Data Structure Choices

### 5.2.1 Option 1 — String Counter

INCR key

Simple fixed window.

---

### 5.2.2 Option 2 — Sorted Set

Useful for exact sliding windows.

ZADD
ZRANGEBYSCORE
ZREMRANGEBYSCORE

Pros:

- precise

Cons:

- memory expensive

---

### 5.2.3 Option 3 — Hashes / Rolling Buckets

Example:

user:123:api:/search

Value:

{
bucket_1: 20,
bucket_2: 10,
bucket_3: 5
}

Good scalable choice.

---

# 6. Request Lifecycle

Example flow:

1. Request arrives
2. Extract userId/IP/API
3. Build rate limit key
4. Find current bucket
5. Increment bucket count atomically
6. Sum valid buckets
7. Compare against limit
8. Allow or reject

---

# 7. Important HTTP Response Discussion

## 7.1 Wrong Answer

304 Not Modified

304 is for caching.

---

## 7.2 Correct Status Code

429 Too Many Requests

Usually includes:

Retry-After: 30
X-RateLimit-Limit
X-RateLimit-Remaining

---

# 8. Concurrency & Race Conditions

Very important topic.

## 8.1 Problem

1000 requests arrive simultaneously.

Need atomic updates.

---

## 8.2 Solutions

### 8.2.1 Redis Atomic Commands

INCR

---

### 8.2.2 Lua Scripts

Do:

- read
- update
- validate

atomically.

Strong answer.

---

### 8.2.3 Distributed Locks

Usually avoid unless necessary.

Too expensive at high RPS.

---

# 9. Scaling Problems

## 9.1 What happens if every request hits Redis?

Excellent interviewer follow-up.

At 1M+ RPS:

- Redis can become bottleneck
- network latency matters
- hot keys become issue

---

## 9.2 Possible Optimizations

### 9.2.1 Local In-Memory Cache

Each gateway keeps temporary counters.

Pros:

- reduces Redis calls

Cons:

- eventual consistency

---

### 9.2.2 Batch Synchronization

Push aggregated counters periodically.

---

### 9.2.3 Sharding

Distribute users across Redis cluster.

Example:

hash(userId) % N

---

### 9.2.4 Consistent Hashing

Better redistribution during scaling.

---

### 9.2.5 Approximate Counting

Trade perfect accuracy for scalability.

Often acceptable.

---

# 10. Failure Handling

Very important senior-level discussion.

## 10.1 What if Redis goes down?

### 10.1.1 Option 1 — Fail Open

Allow requests.

Pros:

- availability

Cons:

- abuse possible

---

### 10.1.2 Option 2 — Fail Closed

Reject requests.

Pros:

- protects backend

Cons:

- hurts availability

---

### 10.1.3 Real Production Approach

Depends on endpoint.

Example:

| Endpoint       | Strategy     |
| -------------- | ------------ |
| Payments       | Fail closed  |
| Login          | Maybe closed |
| Public content | Fail open    |

Very strong answer.

---

# 11. Multi-Region Challenges

## 11.1 Problem

Global consistency is hard.

Example:

US region count != India region count

---

## 11.2 Solutions

### 11.2.1 Global Redis

Pros:

- centralized

Cons:

- high latency

---

### 11.2.2 Regional Rate Limiting

Each region has own limits.

Pros:

- low latency

Cons:

- approximate global enforcement

---

### 11.2.3 Hybrid

Local enforcement + async global sync.

Usually strongest practical answer.

---

# 12. Hot Key Problem

## 12.1 Problem

Celebrity/API key receives massive traffic.

Single Redis node overloaded.

---

## 12.2 Solutions

### 12.2.1 Key Partitioning

Example:

user123:0
user123:1
user123:2

---

### 12.2.2 Randomized Buckets

Spread load.

---

### 12.2.3 Local Buffers

Reduce direct hits.

---

# 13. Observability

Strong candidates discuss this.

## 13.1 Metrics

Track:

allowed_requests
rejected_requests
redis_latency
hot_keys
bucket_utilization

---

## 13.2 Logging

Important for abuse detection.

---

## 13.3 Dashboards

- Grafana
- Prometheus
- Datadog

---

# 14. Example Strong Final Architecture

Client
|
API Gateway / Middleware
|
Local In-Memory Cache
|
Redis Cluster (Sharded)
|
Backend Services

Features:

- centralized
- distributed
- scalable
- low latency
- configurable policies
- rolling bucket counters
- Redis TTL cleanup
- Lua atomic updates
- 429 responses

---

refer 'https://bytebytego.com/courses/system-design-interview/design-a-rate-limiter'