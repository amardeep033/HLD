RabbitMQ is a message broker — software that helps different services/applications communicate with each other asynchronously, reliably, and at scale. - Decouple microservices
Producer sends message → Exchange
Exchange decides based on routing rules → Queue
Consumer pulls the message from the queue
Consumer acknowledges after processing - Messages aren’t lost if a consumer crashes.
RabbitMQ removes message from queue
Common use cases: Payment processing queues, Notification systems (SMS, email), Fraud detection pipelines, Order/event workflows, Logging pipelines

Kafka (Event Streaming Platform)- store events immutably and replay anytime.
Behaves like a distributed commit log: High throughput, partitioning, consumer groups.
Consumers track their own offsets: Ideal when you need streams, analytics, audits, event sourcing, high-scale pipelines.

If your system is about tasks/jobs → choose RabbitMQ.
If your system is about events/data streams → choose Kafka.
If you need high throughput + replay + durability → Kafka.
If you need routing + flexible exchange patterns → RabbitMQ.

“Do this task once and remove it.” vs “Record all events so multiple systems can read them anytime, even replay history.”

DLQ: Orch issues a command → Exec executes it, but exec may panic / lose connectivity and fail to report completion to orch → DB stays stale and the system thinks the command didn’t finish.

Make actions idempotent (no re-executions).
Persist intent and results durably (so a crash doesn’t lose them).
Use explicit delivery & acknowledgement (don’t rely on best-effort RPC(Http req)).
Reconcile periodically (a sweeper to fix missed updates) ---- Re-checks evidence of completion (logs, executor heartbeat, idempotency store).
Observability & correlation (trace every command with a unique id).

When Exec can’t call Orch update API, Exec should retry (with exponential backoff) and then push the update into a durable fallback (local disk, persistent queue, or DLQ). If retries fail permanently, put into a Dead Letter Queue and alert the ops team.
For status updates, publish to a dedicated status topic or queue rather than directly calling Orch API—Orch subscribes to this topic and updates DB.

event driven architecture: Services communicate by producing and consuming events, rather than direct API calls. This decouples services, improves scalability, and allows for more flexible communication patterns.

Integration / Communication Patterns

1. Request-Reply
   A service sends a request and waits for a response.
   Examples: REST, gRPC, GraphQL

2. Event-Driven
   Components communicate by producing and reacting to events.
   No direct coupling between sender and receiver.
   Examples:
   - Pub/Sub: publisher emits to a topic, multiple subscribers react independently
   - Message Queue: point-to-point async delivery (RabbitMQ, Azure Service Bus)
   - Event Streaming: ordered, replayable log of events (Kafka)

3. Event Sourcing  ← separate pattern, often paired with EDA but distinct
   State is stored as an immutable sequence of events, not as a current snapshot.
   State is reconstructed by replaying the event log.
   Often combined with CQRS.