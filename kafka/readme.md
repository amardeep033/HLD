# Kafka

---

## 1. What Kafka Is

Kafka is a distributed event log.

That sentence matters. Kafka is not just a queue. It stores events in ordered append-only logs, and many consumers can read those logs at their own pace.

Classic request-response:

```
Order Service  --HTTP-->  Payment Service
```

Kafka-based communication:

```
Order Service  --produces-->  order.created topic  --consumed by-->  Payment Service
                                                         `---------->  Email Service
                                                         `---------->  Analytics Service
```

The producer does not need to know who consumes the event. This is the main architectural win: services are decoupled in time and ownership.

---

## 2. Core Terms

### 2.1 Event / Message

An event is a fact that already happened.

Examples:

```json
{
  "order_id": "ord_123",
  "user_id": "user_7",
  "amount": 1299,
  "event_type": "order.created"
}
```

Good event names are past tense:

- `order.created`
- `payment.authorized`
- `shipment.dispatched`

Avoid command-like names such as `create_order`. Kafka carries facts, not function calls.

### 2.2 Topic

A topic is a named stream of related events.

Examples:

- `orders`
- `payments`
- `user-events`
- `inventory-updates`

Think of a topic as a table-like logical name, but physically it is split into partitions.

### 2.3 Partition

A partition is one ordered log inside a topic.

```
orders topic
|-- partition 0:  [e1][e4][e8]
|-- partition 1:  [e2][e5][e9]
`-- partition 2:  [e3][e6][e7]
```

Kafka only guarantees ordering inside one partition. It does not guarantee global ordering across all partitions.

This is the most important interview point.

### 2.4 Offset

An offset is the position of a message inside a partition.

```
partition 0:
offset 0 -> order A created
offset 1 -> order B created
offset 2 -> order A paid
```

Consumers track offsets to know how far they have read.

### 2.5 Broker

A broker is one Kafka server. A Kafka cluster has multiple brokers.

Each broker stores some partitions. Partitions are replicated across brokers for fault tolerance.

### 2.6 Producer

A producer writes events to a topic.

The producer chooses the partition. Usually it does that using the message key.

```text
key = order_id
value = order event JSON
```

If all events for the same `order_id` use the same key, they go to the same partition. That preserves order for that order.

### 2.7 Consumer

A consumer reads events from a topic.

Consumers usually run in a consumer group.

### 2.8 Consumer Group

A consumer group is a set of consumer instances sharing work.

If a topic has 3 partitions and a group has 3 consumers:

```
partition 0 -> consumer A
partition 1 -> consumer B
partition 2 -> consumer C
```

If the group has 6 consumers but only 3 partitions, 3 consumers sit idle. Kafka parallelism is bounded by partition count.

Different consumer groups each get their own copy of the stream:

```
orders topic
|-- payment-service group    reads all orders
|-- email-service group      reads all orders
`-- analytics-service group  reads all orders
```

---

## 3. The Mental Model

Kafka is like a durable commit log:

```
Producer appends events
        v
Kafka stores them by topic-partition-offset
        v
Consumers pull events and commit offsets
```

Kafka does not push messages to consumers. Consumers poll.

That matters because consumers control their own pace. If a consumer is slow, Kafka keeps the events until retention expires. The producer does not block on every downstream service.

---

## 4. Ordering

Kafka ordering is simple but easy to say wrong.

Kafka guarantees:

- order within one partition
- order for messages with the same key, assuming the key maps to the same partition

Kafka does not guarantee:

- order across partitions
- order across different keys

Example:

```
key = order_123

order.created  -> partition 1 offset 10
order.paid     -> partition 1 offset 11
order.shipped  -> partition 1 offset 12
```

This is ordered.

If you do not set a key, events may be spread across partitions and ordering for one entity can break.

> Interview sentence: "I choose the partition key based on the entity whose order I need to preserve, for example `order_id` for order lifecycle events."

---

## 5. Delivery Semantics

### 5.1 At-most-once

The consumer commits the offset before processing.

If it crashes after commit but before processing, the message is lost.

Use this only when losing an event is acceptable.

### 5.2 At-least-once

The consumer processes the message, then commits the offset.

If it crashes after processing but before commit, Kafka will deliver the same message again.

This is the common default.

Your handler must be idempotent.

### 5.3 Exactly-once

Kafka supports exactly-once semantics with idempotent producers and transactions, but it is not magic.

It mostly means exactly-once between Kafka topics when using Kafka transactions. External side effects like database writes, emails, and payment calls still need idempotency.

> Interview sentence: "I usually design for at-least-once delivery and make consumers idempotent. Exactly-once is useful for Kafka-to-Kafka pipelines, but external systems still need dedupe."

---

## 6. Idempotency

Idempotency means processing the same event twice has the same result as processing it once.

Kafka consumers need this because retries and rebalances can cause duplicate delivery.

Common techniques:

- Put a unique `event_id` in every event
- Store processed `event_id`s in a database table
- Use upserts instead of inserts where possible
- Make state transitions monotonic, for example `CREATED -> PAID -> SHIPPED`
- Use idempotency keys when calling external APIs

Example:

```
event_id = "evt_123"

Consumer receives evt_123
  -> checks processed_events table
  -> not found
  -> applies business change
  -> inserts evt_123
  -> commits Kafka offset

Consumer receives evt_123 again
  -> already processed
  -> skip
  -> commit offset
```

---

## 7. Consumer Lag

Consumer lag is:

```
latest offset in Kafka - last committed offset by consumer group
```

If Kafka has offset 10,000 and your consumer has committed 9,100, lag is 900.

Lag means the consumer is behind.

Causes:

- consumer processing is slow
- downstream database/API is slow
- not enough partitions for parallelism
- consumer crashed
- large traffic spike

How to fix:

- scale consumers up, if partitions allow it
- increase partition count for future scalability
- optimize processing logic
- batch database writes
- reduce slow external calls
- pause/retry poison messages instead of blocking the whole partition forever

---

## 8. Rebalancing

Rebalancing happens when Kafka changes partition ownership inside a consumer group.

Triggers:

- a consumer joins
- a consumer leaves
- a consumer crashes
- partitions are added

During a rebalance, consumers may briefly stop processing. If your consumer has not committed offsets carefully, messages may be reprocessed.

This is another reason consumers should be idempotent.

---

## 9. Retention

Kafka stores events for a configured retention period or size.

Example:

```text
retention.ms = 7 days
```

After retention expires, old messages are deleted even if nobody consumed them.

This is different from a traditional queue, where a message is usually removed once consumed. In Kafka, consumption does not delete the message.

This allows replay:

```
New analytics service starts today
  -> seek to earliest offset
  -> replay last 7 days of orders
```

---

## 10. Replication and Durability

Each partition has one leader and multiple followers.

```
partition 0
|-- broker 1: leader
|-- broker 2: follower
`-- broker 3: follower
```

Producers write to the leader. Followers replicate from the leader.

Important producer setting:

```text
acks=all
```

This means the producer waits until the message is acknowledged by all in-sync replicas, not just the leader.

Use:

- `acks=all`
- `enable.idempotence=true`
- `min.insync.replicas >= 2` in production

This prevents data loss when a broker dies right after accepting a write.

---

## 11. Kafka vs Queue

| Area | Kafka | Traditional Queue |
|---|---|---|
| Storage | Durable append-only log | Message queue |
| Consumption | Many groups can read same event | Usually one consumer removes message |
| Replay | Natural, via offsets | Usually hard or unavailable |
| Ordering | Per partition | Often queue-level or limited |
| Scaling | Partitions | Competing consumers |
| Best for | Event streaming, fanout, replay | Task dispatch, simple async jobs |

Use Kafka when multiple systems need the same stream, event history matters, or replay is valuable.

Use a queue when you just need background job execution.

---

## 12. HLD Example: Order Pipeline

### 12.1 Requirements

Build an order processing system:

- API receives orders
- Payment service charges user
- Inventory service reserves stock
- Email service sends confirmation
- Analytics service tracks events
- System should tolerate downstream failures

### 12.2 Architecture

```
Client
  v
Order API
  v writes order row
  v produces order.created
Kafka topic: orders
  |-- Payment Service    consumer group: payment-service
  |-- Inventory Service  consumer group: inventory-service
  |-- Email Service      consumer group: email-service
  `-- Analytics Service  consumer group: analytics-service
```

### 12.3 Partition Key

Use `order_id` as the Kafka key.

Why:

- all events for one order stay ordered
- different orders can be processed in parallel
- consumer can reason about order state transitions safely

### 12.4 Failure Handling

For transient failures:

```
consumer fails processing
  -> do not commit offset
  -> retry with backoff
```

For poison messages:

```
message fails repeatedly
  -> publish to orders.dlq
  -> commit original offset
  -> alert for manual/debug workflow
```

### 12.5 Database Consistency

The common hard problem:

```
write order to database
produce order.created to Kafka
```

If the DB write succeeds but Kafka publish fails, the system is inconsistent.

Production answer: transactional outbox.

```
Same DB transaction:
  1. insert order
  2. insert outbox row: order.created

Background relay:
  1. reads unsent outbox rows
  2. publishes to Kafka
  3. marks outbox row sent
```

This makes database write and event creation atomic from the application point of view.

---

## 13. Common Interview Questions

### 13.1 Why Kafka instead of HTTP between services?

Use Kafka when producers and consumers should be decoupled, multiple consumers need the same event, and replay/backpressure matter.

HTTP is fine for synchronous queries or commands where the caller needs an immediate answer.

### 13.2 How do you scale consumers?

Add more consumer instances up to the number of partitions. If topic has 12 partitions, a group can actively use at most 12 consumers.

### 13.3 How do you preserve ordering?

Put all events for the same entity on the same partition by using a stable key like `order_id` or `user_id`.

### 13.4 Can Kafka lose messages?

Yes, if configured poorly. Safer producer settings are `acks=all`, idempotence enabled, retries enabled, and enough in-sync replicas.

### 13.5 Can Kafka duplicate messages?

Yes. At-least-once consumers can see duplicates after crashes or rebalances. Design idempotent consumers.

### 13.6 What is consumer lag?

The difference between the latest Kafka offset and the last offset committed by a consumer group. It shows how far behind consumers are.

### 13.7 What is a DLQ?

A dead-letter queue/topic stores messages that repeatedly fail processing, so one bad message does not block a partition forever.

### 13.8 What is the outbox pattern?

A pattern for reliably publishing events after database writes. Store the event in an outbox table in the same DB transaction, then a relay publishes it to Kafka.

---

## 14. Rust Crates to Know

```
rdkafka       # Kafka producer and consumer client
tokio         # async runtime
axum          # small HTTP service
serde         # JSON serialization
uuid          # event_id and order_id generation
tracing       # structured logs
```

For a machine coding round, keep the scope small:

1. HTTP endpoint accepts an order
2. Producer publishes `order.created`
3. Consumer reads the event in the same binary
4. Consumer updates in-memory status
5. Health/status endpoints prove it works

That is enough to show real integration without building a full distributed system.

---

## 15. One-Page Summary

```
Kafka = durable distributed event log

Topic      = logical event stream
Partition  = ordered append-only log inside a topic
Offset     = message position inside one partition
Producer   = writes events
Consumer   = reads events
Group      = consumers sharing partitions

Ordering:
  guaranteed only within one partition
  choose key = entity whose order matters

Delivery:
  common default = at-least-once
  duplicates are possible
  consumers must be idempotent

Scaling:
  parallelism is bounded by partition count
  consumer lag tells you if consumers are behind

Reliability:
  producer: acks=all + idempotence
  consumer: process then commit
  poison message: retry, then DLQ
  DB + Kafka consistency: transactional outbox
```
