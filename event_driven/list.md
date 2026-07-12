# The Backend Architecture Decision Tree

Each chapter follows the same pattern: **here's the problem → here are your options → why choose one over another.** This isn't "learn Kafka" — it's "how do backend systems evolve as they hit bottlenecks."

---

## 0. Foundation — How code executes

Before distributed systems make sense, you need to know what happens inside a single process.

| Concept | What it is | Why it matters |
|---|---|---|
| Process vs Thread | A process has its own memory space; threads share memory within a process. | Explains isolation/cost tradeoffs — why crashing a thread can take down a process, but processes can't corrupt each other. |
| Sync vs Async | Sync waits for a call to finish before moving on; async kicks off work and continues, picking up the result later. | Determines whether your code blocks the caller or frees it up to do other work while waiting. |
| Blocking vs Non-blocking | Blocking calls halt the thread until an I/O operation completes; non-blocking returns immediately (often with a "not ready yet" signal). | Underpins how servers scale — non-blocking I/O lets one thread handle thousands of connections (e.g., Node.js, Netty). |
| Parallelism vs Concurrency | Concurrency is structuring work as independent tasks; parallelism is literally running them at the same time on multiple cores. | You can have concurrency without parallelism (single core, interleaved) — mixing these up leads to wrong assumptions about speedup. |
| CPU-bound vs IO-bound | CPU-bound work is limited by processor speed (e.g., encoding); IO-bound work is limited by waiting on disk/network. | Dictates your scaling strategy — CPU-bound needs more cores/parallelism, IO-bound needs more concurrency (async, non-blocking). |

---

## 1. Communication — How services talk to each other

**Decision path:** Need another service → Sync? → REST/gRPC. Async? → Message Broker.

| Concept | What it is | Why it matters |
|---|---|---|
| Request-driven vs Event-driven | Request-driven: caller asks and waits for a response. Event-driven: caller emits a fact and moves on, others react later. | Event-driven decouples services in time — the producer doesn't need the consumer to be up or fast. |
| REST vs gRPC | REST is JSON over HTTP, human-readable and universal; gRPC is binary Protobuf over HTTP/2, fast and strongly typed. | REST wins for public/browser-facing APIs; gRPC wins for internal service-to-service calls needing speed and strict contracts. |
| Polling vs Webhooks | Polling repeatedly asks "anything new?"; webhooks let the other side push data when something happens. | Webhooks save resources and cut latency, but require the receiver to expose a reachable endpoint — polling is simpler when that's not possible. |
| WebSockets vs SSE | WebSockets are full-duplex (both sides send anytime); SSE is one-way, server-to-client only, over plain HTTP. | Choose SSE for simple live feeds (notifications, stock tickers); choose WebSockets when the client also needs to send data continuously (chat, gaming). |
| Async vs Sync (distributed) | Sync calls chain services together live; async decouples them via a broker or queue in between. | Sync is simpler to reason about but fragile (one slow service stalls the chain); async trades simplicity for resilience and independent scaling. |

---

## 2. Messaging — Once you've gone async, who receives it and how?

| Concept | What it is | Why it matters |
|---|---|---|
| Message Queue | One message, consumed by exactly one worker from the pool. | Used for distributing work/tasks — great for load leveling (e.g., job processing). |
| Pub/Sub | One message, broadcast to every subscriber of a topic. | Used for fan-out notifications — many services need to know the same thing happened. |
| Stream | An ordered, replayable, durable log of events (not deleted after consumption). | Enables replay and multiple independent consumers reading at their own pace (e.g., Kafka topics). |
| Batch Processing | Data is collected and processed in chunks on a schedule, not immediately. | Trades latency for throughput/efficiency — cheaper for large volumes where real-time isn't required. |
| Kafka = WAL | Kafka is fundamentally a distributed, append-only Write-Ahead Log. | Explains why Kafka can replay history and support many consumer groups — it's storage first, messaging second. |
| DLQ (Dead Letter Queue) | A holding queue for messages that repeatedly fail processing. | Prevents one poison message from blocking the whole queue or looping retries forever. |
| Retry | Automatically re-attempting a failed operation, often with backoff. | Absorbs transient failures (network blips) without involving a human or failing the whole request. |
| Visibility Timeout | Time a message is "invisible" to other consumers while one is processing it. | Prevents duplicate processing — if the timeout expires before an ack, the message becomes visible again for retry. |
| Consumer Groups | A set of consumers that split a topic's partitions among themselves. | The mechanism for horizontal scaling of consumption — add consumers to process more partitions in parallel. |
| Ordering | Guarantee that messages are processed in the order they were produced. | Usually only guaranteed within a partition — critical for things like "update before delete" on the same entity. |
| Delivery Guarantees | At-most-once, at-least-once, or exactly-once semantics. | Defines what failure modes you must design for — most systems default to at-least-once, forcing you to handle duplicates. |
| Idempotency | Processing the same message twice has the same effect as processing it once. | The practical fix for at-least-once delivery — without it, retries cause double charges, duplicate emails, etc. |

---

## 3. Data Architecture — When one database isn't enough

**Decision path:** One DB enough? → No → these patterns.

| Concept | What it is | Why it matters |
|---|---|---|
| CQRS | Splits the read model from the write model into separate paths. | Lets you optimize/scale reads and writes independently (e.g., denormalized read store vs normalized write store). |
| Event Sourcing | Store the sequence of state-changing events, not just the current state. | Gives you a full audit trail and the ability to rebuild state at any point in time — but adds complexity to querying "current" state. |
| Outbox Pattern | Write the DB change and the "event to publish" in the same local transaction, then relay it to the broker. | Solves the dual-write problem — guarantees you never update the DB without also emitting the event (or vice versa). |
| CDC (Change Data Capture) | Streams row-level database changes out as events, usually by reading the DB's transaction log. | Lets you sync data or trigger downstream systems without touching application code — often used to implement the outbox pattern automatically. |
| Read Replica | A copy of the database that serves read traffic, kept in sync with the primary. | Scales read throughput horizontally and isolates reporting/analytics load from the write path. |
| Eventual Consistency | Replicas/consumers converge to the correct state eventually, not instantly. | The tradeoff you accept for availability and scale in distributed systems — you must design UX/logic to tolerate a lag window. |
| Transactions | A group of operations that succeed or fail together, atomically. | The baseline guarantee for correctness within a single database — but doesn't naturally extend across services. |
| Saga | A sequence of local transactions across services, each with a compensating action if a later step fails. | The way to get transaction-like consistency across microservices without a distributed lock — trades atomicity for a defined rollback path. |
| 2PC (Two-Phase Commit) | A protocol where a coordinator asks all participants to "prepare," then "commit" only if everyone agrees. | Gives strong cross-service atomicity, but blocks and doesn't scale well — mostly why Sagas are preferred in practice. |

---

## 4. API Layer — How clients reach us

| Concept | What it is | Why it matters |
|---|---|---|
| API Gateway | A single entry point that routes, authenticates, and rate-limits requests to backend services. | Centralizes cross-cutting concerns (auth, throttling, logging) so individual services don't reimplement them. |
| BFF (Backend For Frontend) | A dedicated backend layer tailored to one specific client (web, mobile, etc.). | Avoids one generic API trying to please every client — each frontend gets exactly the shape of data it needs. |
| GraphQL | A query language letting clients specify exactly which fields they need in one request. | Solves over-fetching/under-fetching from REST — especially valuable when clients have very different data needs. |
| Reverse Proxy | A server that sits in front of your services and forwards client requests to them. | Adds a layer for TLS termination, caching, and hiding internal topology from the outside world. |
| Load Balancer | Distributes incoming traffic across multiple service instances. | Enables horizontal scaling and high availability — no single instance is a bottleneck or single point of failure. |
| CDN | A geographically distributed network of servers caching content close to users. | Cuts latency and origin load by serving static (and sometimes dynamic) content from the nearest edge location. |

---

## 5. Performance — When it's too slow

| Concept | What it is | Why it matters |
|---|---|---|
| Caching | Storing computed/fetched results temporarily to avoid redoing the work. | The single highest-leverage fix for latency and DB load — but introduces staleness risk you must manage. |
| Redis | An in-memory key-value store commonly used as a cache, session store, or lightweight broker. | Its in-memory nature gives sub-millisecond access, making it the default choice for caching layers. |
| Cache Aside | App checks the cache first; on a miss, reads from DB and populates the cache itself. | Simple and widely used — cache only holds what's actually been requested, but the first request always pays the DB cost. |
| Read Through | The cache itself is responsible for loading data from the DB on a miss (app just talks to the cache). | Simplifies application code vs cache-aside, at the cost of coupling the cache layer to your data-loading logic. |
| Write Through | Writes go to the cache first, which synchronously writes to the DB. | Keeps cache and DB always consistent, but adds write latency since both stores must be updated before returning. |
| Write Behind | Writes go to the cache, which asynchronously flushes to the DB later. | Fast writes, but risks data loss if the cache fails before flushing — needs durability tradeoffs to be explicit. |
| Cache Invalidation | Removing or updating stale cache entries when underlying data changes. | Famously "one of the two hard problems in CS" — get it wrong and users see outdated data indefinitely. |
| CDN | (see API Layer) Edge caching for static/dynamic content. | Also a performance lever, not just a networking one — reduces origin load and round-trip time. |

---

## 6. Reliability — When dependencies fail

| Concept | What it is | Why it matters |
|---|---|---|
| Retry | Re-attempting a failed call, ideally with backoff and jitter. | Handles transient failures automatically — but naive retries can amplify load on an already-struggling service. |
| Timeout | A hard limit on how long you'll wait for a response before giving up. | Prevents one slow dependency from hanging your entire request chain indefinitely. |
| Circuit Breaker | Stops calling a failing dependency for a cooldown period after repeated failures. | Protects the caller (and the failing service) from wasting resources on calls that are very likely to fail. |
| Bulkhead | Isolates resources (thread pools, connections) per dependency so one failure can't exhaust shared resources. | Prevents cascading failure — a slow dependency starves only its own pool, not the whole service. |
| Rate Limiter | Caps how many requests a client or service can make in a time window. | Protects services from being overwhelmed by traffic spikes or abusive clients. |
| Backpressure | A mechanism for a system to signal "slow down" to whoever is sending it work. | Prevents producers from overwhelming consumers — without it, queues grow unbounded and systems fall over under load. |

---

## 7. Security

| Concept | What it is | Why it matters |
|---|---|---|
| Cookies | Small key-value data stored by the browser and sent with every request to a domain. | The traditional mechanism for maintaining state (like sessions) over stateless HTTP. |
| Sessions | Server-side stored state, referenced by a session ID (usually held in a cookie). | Lets the server keep authoritative control of user state — can be revoked instantly, unlike stateless tokens. |
| JWT | A signed (optionally encrypted) token carrying claims, verifiable without a DB lookup. | Enables stateless auth that scales horizontally — but revoking a single JWT before expiry is hard. |
| OAuth | A protocol for delegated authorization — letting an app act on a user's behalf without their password. | The standard for "Login with Google/GitHub" and third-party API access without credential sharing. |
| CSRF | An attack tricking a logged-in user's browser into making unwanted requests to a site they're authenticated on. | Explains why state-changing requests need CSRF tokens or SameSite cookies as defense. |
| CORS | A browser security mechanism controlling which origins can call your API from client-side JS. | Misconfiguring it either blocks legitimate frontends or opens your API to unintended origins. |
| API Keys | A static secret identifying the calling application (not a specific user). | Simple to implement for service-to-service or third-party API access, but weaker than OAuth for user-level permissions. |

---

## 8. Observability

| Concept | What it is | Why it matters |
|---|---|---|
| OpenTelemetry (OTel) | A vendor-neutral standard/SDK for collecting logs, metrics, and traces. | Lets you switch observability backends (Datadog, Grafana, etc.) without re-instrumenting your code. |
| Logging | Recording discrete events as they happen in the system. | The most basic debugging tool — answers "what happened," but doesn't scale well for cross-service investigation alone. |
| Metrics | Numeric, aggregatable measurements over time (latency, error rate, throughput). | Powers dashboards and alerting — tells you *that* something is wrong and how bad, cheaply, at scale. |
| Tracing | Following a single request's path across multiple services, as one connected trace. | Answers "where in the chain did it slow down or fail," which logs/metrics alone can't localize. |
| Correlation IDs | A unique ID attached to a request and propagated through every service it touches. | The glue that lets you stitch together logs from different services into one coherent story. |
| Health Checks | Endpoints reporting whether a service instance is alive/ready to take traffic. | Lets load balancers and orchestrators (e.g., Kubernetes) automatically route around or restart unhealthy instances. |

---

## 9. Missing topics worth adding

These come up surprisingly often in interviews and real systems, and didn't fit neatly above.

| Concept | What it is | Why it matters |
|---|---|---|
| Service Discovery | The mechanism by which services find the network location of other services. | Needed because instances come and go dynamically (scaling, deploys) — hardcoded IPs don't survive that. |
| DNS | Translates human-readable names into IP addresses. | The oldest form of service discovery — still underlies most routing even in modern systems. |
| Consul | A tool for service discovery, health checking, and configuration. | A concrete example of dynamic service discovery in non-Kubernetes environments. |
| Kubernetes Services | A stable network identity/IP for a set of pods that may be rescheduled at any time. | K8s' built-in answer to service discovery — abstracts away individual pod churn. |
| Database Scaling | The general problem of handling more data/traffic than one DB instance can serve. | The umbrella problem that replication, sharding, and partitioning each solve differently. |
| Replication | Copying data across multiple DB nodes for redundancy and read scaling. | Improves availability (failover) and read throughput, at the cost of consistency lag. |
| Sharding | Splitting a dataset across multiple DB instances by some key (e.g., user ID range). | Scales writes horizontally when a single node can't hold or handle all the data. |
| Partitioning | Dividing a large table/dataset into smaller pieces, often within the same DB instance. | Improves query performance and manageability without necessarily distributing across servers. |
| Indexes | Data structures that let the DB find rows without scanning the whole table. | The default first fix for slow queries — but every index adds write overhead and storage cost. |
| Scheduling | Running work at specific times or intervals rather than on-demand. | Needed for recurring maintenance/business tasks (cleanup, reports) that don't map to a user request. |
| Cron Jobs | Time-based scheduled tasks (e.g., "run every night at 2am"). | The simplest, most common scheduling primitive — but doesn't handle overlapping runs or distributed coordination well. |
| Batch Processing | (see Messaging) Processing accumulated data in scheduled chunks. | Relevant here too — often the workload that cron jobs trigger. |
| Workers | Background processes that pull tasks off a queue and execute them, separate from the request path. | Keeps slow/heavy work off the user-facing request-response cycle. |
| Distributed Locking | Coordinating exclusive access to a resource across multiple machines (e.g., via Redis or Zookeeper). | Prevents race conditions when multiple instances might otherwise act on the same resource simultaneously. |
| CAP Theorem | A distributed system can only guarantee two of Consistency, Availability, and Partition tolerance at once. | The theoretical backbone tying together CQRS, Saga, Kafka, and eventual consistency — every one of those is a CAP tradeoff in practice. |

---

## Notes on structure

- **OpenTelemetry belongs under Observability**, not next to CQRS/Saga — messaging and OTel aren't conceptually linked; OTel is about *observing* any of these systems, not a data-architecture pattern.
- **CAP Theorem is the unifying thread** — CQRS, Saga, Kafka, and eventual consistency are all different answers to the same CAP tradeoff.
- Every chapter above should be read as: *"Here's the problem → here are your options → why choose one over the other."*
