# System Design / Backend Engineering Plan

---

# THEORY

- CAP theorem
- Consistent hashing
- CDN
- CQRS
- Event sourcing
- Materialized view
- API gateway
- REST vs gRPC
- Polling vs WebHooks
- Bloom filter
- Merkle tree / gossip
- 0 to million users

---

# DATABASE

- SQL vs NoSQL
- Replication + sharding
- Indexing
- Transactions
- Partitioning
- Read replicas
- ACID vs BASE
- Query optimization
- OLTP vs OLAP
- Sharding strategies

---

# RELIABILITY

- Rate limiting + throttling
- Circuit breaker
- Retry + DLQ
- Saga pattern
- Transactional outbox
- Strangler pattern
- Idempotency
- Timeout + backoff
- Health checks
- Sidecar / Ambassador
- Anti-corruption adapter
- Idempotency / consistency

---

# SCALABILITY

- Load balancing algos
- MQ vs Stream
- Caching
- CDN
- Scatter gather
- Pipes & filters
- Reverse proxy
- Backend for frontend
- Auto scaling
- Orch/choreography
- Map reduce

---

# DEPLOY / OPS

- Blue/green deploy
- Canary deploy
- Rolling deploy
- Chaos testing
- Monitoring
- Observability
- Logging
- Alerting
- Feature flags

---

# PRACTICE PROBLEMS

- Rate limiter
- LRU/LFU cache
- URL shortener
- Key-value store
- Message queue
- Notification system
- Web crawler
- Job scheduler
- HTTP server
- Cookies

---

# DESIGN CHECKLIST

## 1. Problem Framing

- [ ] What exactly are we designing?
- [ ] What is the primary goal: scale, latency, reliability, cost, security, or simplicity?
- [ ] Is this for API traffic, DB traffic, internal service traffic, file storage, messaging, or something else?
- [ ] Who are the users: internal systems, external clients, anonymous users, paid tenants?
- [ ] What is in scope and what is explicitly out of scope?

---

## 2. Where It Applies

- [ ] Is the logic applied on client side, server side, or both?
- [ ] Is it enforced at API gateway, load balancer, service layer, database layer, cache layer, or worker layer?
- [ ] Is it global, regional, per service, per node, or per instance?
- [ ] Is there a centralized component or is it embedded inside each service?

---

## 3. Dimension / Keying Strategy

- [ ] On what dimension are we applying the design?
- [ ] Is it keyed by IP, userId, tenantId, API key, deviceId, region, endpoint, objectId, or shard key?
- [ ] Do we need layered enforcement such as IP first, then user, then per API?
- [ ] What happens when identity is missing, anonymous, or spoofed?
- [ ] Are there hot keys or skewed tenants that need special handling?

---

## 4. Functional Requirements

- [ ] What should the system do on the happy path?
- [ ] What are the main APIs, inputs, and outputs?
- [ ] What are the read and write paths?
- [ ] What are the important user actions or workflows?
- [ ] What correctness guarantees are required?

---

## 5. Non-Functional Requirements

- [ ] What latency target do we need?
- [ ] What availability target do we need?
- [ ] What consistency model is acceptable?
- [ ] What throughput or QPS/RPS should it support?
- [ ] What durability is required?
- [ ] What cost constraints exist?

---

## 6. Scale Estimation

- [ ] What is the current scale?
- [ ] What is the expected peak scale?
- [ ] Requests per second?
- [ ] Reads vs writes ratio?
- [ ] Daily active users or tenants?
- [ ] Data size per day, month, or year?
- [ ] Peak traffic pattern: bursty, steady, seasonal, regional?

---

## 7. Traffic Spikes / Sudden Increase

- [ ] What happens during sudden traffic spikes?
- [ ] How do we protect downstream dependencies?
- [ ] Do we need queueing, throttling, buffering, shedding, backpressure, or autoscaling?
- [ ] Do we fail open, fail closed, or degrade gracefully?
- [ ] How do we protect against abuse, bots, retries, or stampedes?

---

## 8. Data Model And Storage

- [ ] What data are we storing?
- [ ] What is configuration data?
- [ ] What is transactional or operational data?
- [ ] What is ephemeral vs persistent data?
- [ ] Which data lives in DB, cache, object store, queue, or local memory?
- [ ] What are the primary keys and secondary indexes?
- [ ] What TTL, retention, or archival policy is needed?

---

## 9. Storage Choices

- [ ] Do we need SQL or NoSQL?
- [ ] Do we need Redis or in-memory cache?
- [ ] Do we need object storage?
- [ ] Do we need a message queue or stream?
- [ ] Do we need search indexing?
- [ ] Why is each storage choice appropriate?

---

## 10. Caching Strategy

- [ ] What should be cached?
- [ ] Where is caching applied: client, CDN, gateway, service, DB, or query cache?
- [ ] What are cache keys?
- [ ] What is cache TTL?
- [ ] How do we handle invalidation?
- [ ] What happens during cache miss, cache penetration, or cache stampede?

---

## 11. Sharding / Partitioning

- [ ] Do we need sharding or partitioning?
- [ ] What is the partition key?
- [ ] Why is that key chosen?
- [ ] What happens with uneven distribution or hot partitions?
- [ ] Do we need consistent hashing?
- [ ] How does rebalancing work when capacity changes?

---

## 12. API / Interface Design

- [ ] What are the core APIs or endpoints?
- [ ] What request and response shape is expected?
- [ ] What status codes or error codes are returned?
- [ ] What metadata or headers should be returned?
- [ ] How do clients discover limits, pagination, retries, or failures?

---

## 13. Architecture Placement

- [ ] Separate service or same service?
- [ ] If separate, why is centralization useful?
- [ ] If embedded, why is local simplicity acceptable?
- [ ] Is there a gateway, middleware, sidecar, or library approach?
- [ ] What are the tradeoffs in latency, consistency, and operational complexity?

---

## 14. Request / Data Lifecycle

- [ ] What is the full lifecycle of a request?
- [ ] What happens from request arrival to response?
- [ ] What components are touched in order?
- [ ] Where do we read config?
- [ ] Where do we read or write state?
- [ ] Where can the request be rejected, delayed, retried, or queued?

---

## 15. Algorithm / Strategy Choice

- [ ] Which algorithm or design strategy fits best?
- [ ] Why is it a good fit for this scale and access pattern?
- [ ] What are the tradeoffs?
- [ ] What simpler option exists?
- [ ] What more accurate or more scalable option exists?

---

## 16. Per-Algorithm Thinking

- [ ] When should this algorithm be used?
- [ ] When should it not be used?
- [ ] Is it optimized for simplicity, fairness, burst handling, smoothing, consistency, memory usage, or throughput?
- [ ] Does it require exact counting or approximate counting?
- [ ] Does it work well at small scale and large scale?

---

## 17. Consistency / Concurrency

- [ ] What race conditions can happen?
- [ ] What happens under simultaneous requests or writes?
- [ ] Do we need atomic operations, transactions, locks, CAS, or idempotency?
- [ ] Is eventual consistency acceptable?
- [ ] What inconsistencies are tolerable vs unacceptable?

---

## 18. Failure Handling

- [ ] What happens if cache is down?
- [ ] What happens if DB is down?
- [ ] What happens if one region is down?
- [ ] What happens if a dependency is slow?
- [ ] What is fail-open vs fail-closed behavior?
- [ ] What is the degraded mode?

---

## 19. Security / Abuse / Isolation

- [ ] What are the abuse vectors?
- [ ] How do we isolate one noisy user or tenant from others?
- [ ] Are auth and authz part of the flow?
- [ ] Are there privacy or PII concerns in keys or logs?
- [ ] How do we prevent spoofing, replay, scraping, or brute force?

---

## 20. Multi-Region / Geo Considerations

- [ ] Is the system single-region or multi-region?
- [ ] Is data local, global, or replicated?
- [ ] What is the latency tradeoff across regions?
- [ ] Is global consistency required?
- [ ] Can we do local enforcement with async global sync?

---

## 21. Observability

- [ ] What metrics should be tracked?
- [ ] What logs are needed?
- [ ] What traces are useful?
- [ ] What dashboards are needed?
- [ ] What alerts should fire?
- [ ] How do we detect hot keys, bottlenecks, errors, or retries?

---

## 22. Bottlenecks And Optimizations

- [ ] What is the likely bottleneck at 10x scale?
- [ ] What happens if every request hits the same dependency?
- [ ] What can be batched?
- [ ] What can be cached?
- [ ] What can be approximated?
- [ ] What can be precomputed or asynchronously processed?

---

## 23. Tradeoffs / Alternatives

- [ ] What is the simplest workable design?
- [ ] What is the production-grade design?
- [ ] What tradeoff are we making between accuracy, complexity, latency, and cost?
- [ ] What alternatives were considered and why rejected?

---

## 24. Final Answer Checklist

- [ ] Did I clearly state assumptions?
- [ ] Did I explain where the logic is applied?
- [ ] Did I explain on what dimension it is applied?
- [ ] Did I explain scale and peak load?
- [ ] Did I explain storage and keys?
- [ ] Did I explain lifecycle and architecture flow?
- [ ] Did I explain failure handling?
- [ ] Did I explain algorithm choice and tradeoffs?
- [ ] Did I explain cache and sharding where relevant?
- [ ] Did I mention observability and bottlenecks?
