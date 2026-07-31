# 06 · Cross-Cutting — survive, be fast, be big, be correct, be seen

> immune system. applies to every call at every layer.
> prev ← [05](05_event_driven.md) · next → [07 frontend](07_frontend.md)
> deep dive → [../cache](../cache/readme.md) · [../rate_limiter](../rate_limiter/readme.md) · [../retry_and_circuit_breaker](../retry_and_circuit_breaker/) · [../consistent_hashing_and_kv_design](../consistent_hashing_and_kv_design/readme.md) · [../DB](../DB/readme.md) · [../observability](../observability/readme.md)

## map

```
cross-cutting
├── resilience (survive failure)
│   ├── detect ─────── timeout | health check (liveness / readiness / startup) | heartbeat
│   ├── retry ──────── immediate | fixed delay | exponential backoff | + jitter | retry budget
│   ├── stop ───────── circuit breaker (closed → open → half-open)
│   ├── isolate ────── bulkhead (pool per dependency) | cell-based architecture
│   ├── degrade ────── fallback | cached/default response | kill switch (feature flag)
│   ├── protect self ─ rate limit | throttling | load shedding | backpressure | admission control
│   ├── redundancy ─── replication | failover (active-passive / active-active) | multi-AZ / multi-region
│   └── safe repeat ── idempotency (key + dedupe)
├── performance (be fast)
│   ├── cache ── where | pattern | eviction | invalidation | failure modes
│   ├── CDN ── cache at edge near user
│   ├── connection pooling | keep-alive | http/2 multiplexing
│   ├── batching | compression (gzip, brotli) | pagination
│   ├── async offload → 05
│   └── db ── index | denormalize | read replica | materialized view | query tuning
├── scalability (be big)
│   ├── compute ── horizontal + stateless + LB | autoscaling (HPA, VMSS)
│   └── data
│       ├── replication ── leader-follower | multi-leader | leaderless (quorum)
│       ├── partitioning ── split a table (same or diff node)
│       ├── sharding ── partition across nodes: range | hash | directory | geo
│       └── consistent hashing ── add/remove node moves ~1/N keys
├── consistency (be correct)
│   ├── models ── strong/linearizable | sequential | causal | read-your-writes | monotonic reads | eventual
│   ├── CAP (partition → choose C or A) | PACELC (else → latency vs consistency)
│   ├── ACID (sql) vs BASE (nosql)
│   ├── isolation ── read uncommitted | read committed | repeatable read | snapshot | serializable
│   │   └── anomalies ── dirty read | non-repeatable read | phantom | lost update | write skew
│   ├── concurrency control ── pessimistic (locks) | optimistic (version / etag) | MVCC
│   └── consensus ── Raft | Paxos | ZAB · leader election · quorum (R + W > N) · distributed lock
└── observability (be seen) → 12 tools
    ├── 3 pillars ── logs | metrics | traces  (+ profiles, events)
    ├── glue ── correlation id / trace id / span id, context propagation (W3C traceparent)
    ├── methods ── RED (rate, errors, duration) | USE (utilization, saturation, errors) | 4 golden signals (latency, traffic, errors, saturation)
    └── act ── alerting | dashboards | SLO burn rate | on-call runbooks
```

## resilience (siblings — what each protects)

| pattern | protects | one-liner |
|---|---|---|
| timeout | caller | don't wait forever |
| retry | caller from blips | try again (only idempotent ops!) |
| backoff + jitter | callee | spread retries, avoid thundering herd |
| circuit breaker | both | stop hammering a dead dependency, fail fast |
| bulkhead | caller | one slow dep can't eat all threads |
| fallback | user | degraded answer > error |
| rate limit | callee | cap requests per client/window |
| load shedding | callee | drop low-priority work when overloaded |
| backpressure | consumer | tell producer to slow down |

> order of wrap (typical): timeout → retry → circuit breaker → bulkhead → call

## rate limit algorithms

| algo | one-liner | burst |
|---|---|---|
| fixed window | count per minute bucket | 2x at edges |
| sliding window log | store every timestamp | exact, memory heavy |
| sliding window counter | weighted prev + current window | good approx |
| token bucket | tokens refill at rate, req spends one | allows burst up to bucket |
| leaky bucket | queue drains at fixed rate | smooths, no burst |

## cache — where (near → far from user)

browser → CDN / edge → reverse proxy (nginx, varnish) → app in-memory (caffeine) → distributed (redis) → db buffer pool / page cache

## cache — patterns (siblings)

| pattern | read/write | one-liner |
|---|---|---|
| cache-aside (lazy) | read | app checks cache, miss → db → fill cache |
| read-through | read | cache itself loads from db on miss |
| write-through | write | write cache + db synchronously |
| write-behind / write-back | write | write cache, flush to db async (risk loss) |
| write-around | write | write db only, cache fills on read |
| refresh-ahead | read | refresh hot keys before expiry |

## cache — eviction

LRU (least recently used) · LFU (least frequently) · FIFO · TTL expiry · random · ARC / W-TinyLFU (smart hybrids)

## cache — failure modes

| problem | one-liner | fix |
|---|---|---|
| stampede / thundering herd / dogpile | hot key expires, 1000 reqs hit db | lock/single-flight, early refresh |
| penetration | queries for keys that don't exist | cache nulls, bloom filter |
| avalanche | many keys expire together | jitter TTLs |
| hot key | one key overloads one node | replicate key, local cache |
| stale data | db changed, cache didn't | TTL, invalidate on write, CDC |

## replication (siblings)

| type | writes go to | one-liner |
|---|---|---|
| single leader | 1 leader | followers replicate, simple, failover needed |
| multi-leader | several leaders | multi-region writes, conflict resolution |
| leaderless | any node | quorum R+W>N (dynamo, cassandra) |
| sync vs async | — | sync = durable+slow, async = fast+lag |

## sharding strategies

| strategy | one-liner | pain |
|---|---|---|
| range | a–m / n–z | hotspots |
| hash | hash(key) % N | resharding (→ consistent hashing) |
| directory | lookup table key→shard | lookup is SPOF |
| geo | by region | uneven load |

## consistency ladder (strong → weak)

linearizable → sequential → causal → read-your-writes / monotonic → eventual
