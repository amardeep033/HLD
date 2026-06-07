# Retry & Circuit Breaker

## 1. Failure Types in Distributed Systems

| Type | Description | Strategy |
|---|---|---|
| **Transient** | Temporary — resolves on its own (network blip, brief overload) | Retry |
| **Permanent** | Persistent — retrying makes no difference (bad input, auth failure) | Fail fast / Circuit Breaker |

---

## 2. Retries

### 2.1 When to Retry vs. When Not To

**Retry these (transient):**
- Network timeout / connection reset
- `503 Service Unavailable` / temporary overload
- Transient DB or network issues
- Leader election in progress

**Never retry these (permanent):**
- `4xx` client errors (except `408`, `429`)
- Authentication / authorisation failures
- Validation / business logic failures
- Malformed requests

> **Note on 429:** Retry *only* if the response includes a `Retry-After` header indicating a safe backoff window.

---

### 2.2 Retry Strategies

#### 2.2.1 Fixed Delay
Retry after a constant interval regardless of attempt count.
```
Attempt 1 → wait 1s → Attempt 2 → wait 1s → Attempt 3
```
Simple, but can cause **retry storms** under load.

#### 2.2.2 Linear Backoff
Delay grows linearly with each attempt.
```
1s → 2s → 3s → 4s ...
```

#### 2.2.3 Exponential Backoff
Delay doubles with each attempt. Reduces load more aggressively.
```
1s → 2s → 4s → 8s ...
```

#### 2.2.4 Jitter (Randomised Backoff)
Add randomness to backoff to desynchronise retries across multiple clients.
```
delay = random(0, exponential_delay)
```
Prevents **retry storms** — the situation where many clients retry simultaneously after a shared failure, amplifying load on the recovering service.

---

### 2.3 Retry Configuration

| Parameter | Purpose |
|---|---|
| `maxRetryCount` | Max attempts before giving up |
| `retryDelay` / `retryPolicy` | Fixed value or backoff formula |
| `retryOn` | Conditions that trigger a retry (specific exceptions, status codes) |
| `maxRetryDuration` | Total wall-clock time budget across all attempts |

---

### 2.4 Idempotency and Retries

Retrying a non-idempotent operation (e.g. `POST /orders`) can cause duplicate side effects — two orders created, two payments charged.

**Mitigations:**
- Include an **idempotency key** (`Idempotency-Key: <uuid>`) in each request; the server deduplicates on it.
- Design write operations to be inherently idempotent where possible.
- Maintain a processed-request log server-side.

---

### 2.5 Synchronous vs. Asynchronous Retries

| | Synchronous | Asynchronous |
|---|---|---|
| Caller behaviour | Blocks until retry completes | Continues processing; retry runs in background |
| Latency impact | Higher | Lower |
| Throughput | Lower under retry pressure | Better |
| Complexity | Simple | Requires error propagation & state management |

---

### 2.6 Retry Queues

For high-volume or long-duration retries, push failed requests onto a dedicated queue (e.g. RabbitMQ, SQS) rather than retrying inline.

**Benefits:** Prevents retry storms, enables controlled throughput, survives process restarts.  
**Tune:** max queue depth, per-item TTL, processing concurrency, dead-letter queue for exhausted retries.

---

### 2.7 Libraries

| Platform | Library |
|---|---|
| .NET | **Polly** |
| Java / Spring | **Spring Retry** |
| Go | **go-retry** |

---

## 3. Circuit Breaker

### 3.1 Purpose

Retries help with transient failures, but if a downstream service is genuinely unhealthy, retries just add load and increase latency for callers. The circuit breaker **stops requests early** to a known-bad service, freeing resources and giving the downstream time to recover.

---

### 3.2 States

```
          failures >= threshold
Closed ──────────────────────────► Open
  ▲                                  │
  │   test requests succeed          │  timeout expires
  │                                  ▼
  └──────────────────────── Half-Open
         test requests fail
              │
              └──────────────────► Open
```

| State | Behaviour |
|---|---|
| **Closed** | Requests pass through normally; failure count is tracked |
| **Open** | All requests are blocked; fallback is returned immediately |
| **Half-Open** | A limited number of probe requests are allowed through to test recovery |

---

### 3.3 Configuration

| Parameter | Purpose |
|---|---|
| `failureThreshold` | Consecutive (or percentage) failures required to trip to Open |
| `openTimeout` | How long to stay Open before moving to Half-Open |
| `probeCount` | Number of test requests allowed in Half-Open |
| `successThreshold` | Successes in Half-Open required to return to Closed |

---

### 3.4 Cascading Failures — Why Circuit Breakers Matter

**Scenario:** An e-commerce app has three services — Payment, Inventory, Order Processing. Payment depends on a database; the other two depend on Payment.

Without circuit breakers:
1. DB outage → Payment service starts timing out
2. Inventory and Order services keep calling Payment → their thread pools fill up with blocked threads
3. All three services become unavailable — a cascade

With circuit breakers on each service:
1. DB outage → Payment breaker trips Open after threshold
2. Inventory and Order services receive immediate fallback responses
3. Rest of system stays responsive while Payment recovers

---

### 3.5 Per-Service vs. Per-Endpoint Breakers

**Per-service:** One breaker covers all endpoints of a service.
- Any endpoint failure trips the breaker for the entire service.
- Simple config, but can block healthy endpoints unnecessarily.

**Per-endpoint:** One breaker per route.
- `/products` failing does not trip the breaker for `/products/{id}`.
- More granular and accurate; more configuration overhead.

**Rule of thumb:** Start per-service; move to per-endpoint if you observe unnecessary blocking of healthy routes.

---

### 3.6 Bulkhead Isolation

Bulkhead isolation limits the **blast radius** of a failure by giving each downstream dependency its own resource pool (thread pool, connection pool, semaphore).

Without bulkheads, a slow downstream can exhaust the shared thread pool, starving all other calls — even to healthy services.

**Example (.NET / Polly):**
```csharp
Policy.BulkheadAsync(
    maxParallelization: 10,   // max concurrent calls to this dependency
    maxQueuingActions: 20     // queue size before rejecting
);
```

Combine with circuit breakers: bulkheads limit resource consumption *while* the breaker is closed; the breaker halts traffic once failure rate crosses the threshold.

---

### 3.7 Libraries

| Platform | Library |
|---|---|
| .NET | **Polly** (v8: `ResiliencePipeline`) |
| Java | **Resilience4j** (preferred), **Hystrix** (Netflix, now in maintenance) |
| Java | **Sentinel** (Alibaba, better suited to high-throughput scenarios) |

---

## 4. Request Resilience Pipeline

A well-ordered pipeline for outbound calls:

```
Request
  → Validate (fail fast on bad input — no retry, no breaker)
  → Bulkhead (enforce concurrency limit)
  → Circuit Breaker check (if Open → fallback immediately)
  → Retry (with backoff + jitter)
  → Timeout (per-attempt and total)
  → Downstream call
  → Circuit Breaker records result
  → Fallback (if all retries exhausted or breaker Open)
```

> **Order matters:** Put the circuit breaker *outside* the retry loop so that a tripped breaker short-circuits all retry attempts, not just one.

---

## 5. Retry vs. Circuit Breaker

| | Retry | Circuit Breaker |
|---|---|---|
| Targets | Transient failures | Persistent / systemic failures |
| Action | Re-attempts the operation | Stops requests to the failing service |
| Effect on traffic | Increases traffic during failure | Reduces traffic during failure |
| Recovery role | Recovers from short blips | Protects while a service heals |
| Risk without care | Retry storms, duplicate side-effects | Over-tripping, blocking healthy services |

They are **complementary** — use both together.

---

## 6. Observability

Track these metrics to tune and diagnose your resilience layer:

| Metric | Why it matters |
|---|---|
| Retry count | High count = underlying instability |
| Retry success rate | Low rate = condition is not actually transient |
| Circuit breaker open rate | High rate = downstream is chronically unhealthy |
| Fallback invocation count | Signals degraded-mode frequency |
| P99 latency | Retries add tail latency — track, don't just average |
| Timeout count | Tune timeout values; source of retry pressure |
| Downstream failure % | Direct health indicator of dependencies |

---

## 7. When to Use What — and How They Fit Together

### 7.1 The Four Mechanisms

| Mechanism | What it does | Who it protects |
|---|---|---|
| **Rate Limiter** | Caps the number of requests a *caller* can make in a time window | The downstream service — from being overwhelmed |
| **Throttling** | Slows or sheds excess requests when *you* are overloaded | Your own service — preserves capacity under pressure |
| **Retry** | Re-attempts a failed request after a backoff delay | The caller — recovers from transient failures transparently |
| **Circuit Breaker** | Stops all requests to a known-bad downstream for a cooldown period | Both — reduces caller latency and gives downstream breathing room |

---

### 7.2 Decision Guide: Which Mechanism for Which Problem?

| Symptom / Situation | Use |
|---|---|
| Occasional network blip, 503, timeout | **Retry** with exponential backoff + jitter |
| Downstream is consistently failing or slow | **Circuit Breaker** — stop hammering it |
| A single client is sending too many requests | **Rate Limiter** on the server side (per-client quota) |
| Your own service is receiving more load than it can handle | **Throttling** — shed or queue excess traffic |
| Slow downstream is tying up your thread pool | **Bulkhead** — isolate it to its own resource pool |
| All of the above on a production critical path | **Full resilience pipeline** (see below) |

---

### 7.3 Rate Limiter vs. Throttling — the Distinction

These terms are often used interchangeably but they target different directions:

**Rate Limiter** — enforced *at the server*, per caller identity (API key, IP, tenant).  
Goal: prevent any single client from consuming a disproportionate share of capacity.  
Algorithms: Token Bucket, Leaky Bucket, Fixed/Sliding Window counter.

**Throttling** — enforced *at the server* on total inbound load, regardless of caller.  
Goal: shed excess requests gracefully when total traffic exceeds service capacity.  
Common response: `429 Too Many Requests` with a `Retry-After` header.

From the caller's perspective, both produce a `429` — handle it the same way: respect `Retry-After`, back off, and retry.

---

### 7.4 How They All Integrate — the Full Picture

```
Incoming request (to your service)
  → Rate Limiter      — reject if caller quota exceeded (429)
  → Throttle check    — shed if total capacity exceeded (429)
  → Handle request
      → Outbound call to downstream
            → Bulkhead          — enforce concurrency limit per dependency
            → Circuit Breaker   — short-circuit if downstream is known-bad (fail fast)
            → Retry             — re-attempt on transient failure (backoff + jitter)
            → Timeout           — per-attempt hard deadline
            → Downstream call
            → Circuit Breaker records result (success / failure)
            → Fallback          — degraded response if all attempts exhausted
```

**Key ordering rules:**
1. **Rate limiter and throttle first** — reject bad or excess traffic before spending any resources.
2. **Bulkhead before circuit breaker** — cap concurrency before deciding whether to even try.
3. **Circuit breaker outside the retry loop** — a tripped breaker short-circuits *all* retry attempts, not just the first.
4. **Timeout on every attempt** — retries without per-attempt timeouts can stall indefinitely.
5. **Fallback last** — only reached when all other mechanisms have been exhausted.

---

### 7.5 Layered Ownership

Different mechanisms live in different layers of the stack:

| Layer | Mechanism | Typical owner |
|---|---|---|
| API Gateway / Proxy | Rate limiting, global throttling | Platform / infra team |
| Service mesh (Envoy, Istio) | Circuit breaker, retry, timeout | Platform / infra team |
| Application code | Retry logic, bulkhead, custom fallback | Service team |
| Client SDK / library | Retry, timeout, circuit breaker | SDK / client team |

Avoid duplicating the same mechanism at multiple layers without coordination — double retries (gateway + application) can multiply traffic dramatically.

---

### 7.6 Quick Reference

> **Transient error, healthy downstream** → Retry  
> **Persistent failure, unhealthy downstream** → Circuit Breaker + Fallback  
> **Client abusing your API** → Rate Limiter  
> **You are overwhelmed with total traffic** → Throttle (shed load)  
> **Slow dependency starving your thread pool** → Bulkhead  
> **Production system under real load** → All of the above, layered in order