# Kafka in Rust - 45 Minute Machine Coding Round

---

## 1. What This Builds

This is a small Kafka-backed order service.

```
POST /orders
  v
Rust API validates request
  v
produces order.created event to Kafka topic: orders
  v
background consumer reads same topic
  v
updates in-memory order status
  v
GET /orders/{order_id}
```

This is intentionally small. In an interview, the goal is not to build an entire production order system. The goal is to show that you know:

- how to create a producer
- how to create a consumer
- why the key matters
- when to commit offsets
- how to explain idempotency and retries

---

## 2. Run Kafka Locally

Use the included Docker Compose file:

```bash
docker compose up -d
```

It runs Redpanda, which is Kafka API compatible and easier for local demos.

Create the topic:

```bash
docker exec -it kafka-redpanda rpk topic create orders
```

Check topics:

```bash
docker exec -it kafka-redpanda rpk topic list
```

---

## 3. Run the Rust Service

Without Kafka, for quick compile/run practice:

```bash
cd rust
cargo run
```

This uses the same producer/consumer flow with an in-memory channel. It is useful when the interview environment has no Docker or Kafka.

With real Kafka:

```bash
cd rust
cargo run --features kafka
```

The app listens on:

```text
http://127.0.0.1:8080
```

By default it connects to:

```text
127.0.0.1:9092
```

Override with:

```bash
KAFKA_BOOTSTRAP_SERVERS=127.0.0.1:9092 cargo run
```

With real Kafka:

```bash
KAFKA_BOOTSTRAP_SERVERS=127.0.0.1:9092 cargo run --features kafka
```

---

## 4. Test With Curl

Health:

```bash
curl -s localhost:8080/health
```

Create order:

```bash
curl -s -X POST localhost:8080/orders \
  -H "Content-Type: application/json" \
  -d '{"user_id":"user_1","amount":1299}'
```

Response:

```json
{
  "order_id": "copy-this-id-from-your-response",
  "status": "created"
}
```

Fetch status:

```bash
curl -s localhost:8080/orders/<order_id>
```

After the background consumer reads the event, status becomes:

```json
{
  "order_id": "...",
  "status": "processed"
}
```

---

## 5. Code Walkthrough

### 5.1 Producer

```rust
let producer: FutureProducer = ClientConfig::new()
    .set("bootstrap.servers", bootstrap_servers)
    .set("acks", "all")
    .set("enable.idempotence", "true")
    .create()?;
```

Interview explanation:

> I use `acks=all` and idempotence so producer retries are safer. In production I would also configure replication and `min.insync.replicas` on the broker side.

In this repo, the real Kafka producer compiles when you use:

```bash
cargo run --features kafka
```

The default mode avoids `rdkafka` so the code still compiles on machines without `cmake` or `librdkafka`.

### 5.2 Message Key

```rust
let key = event.order_id.to_string();

FutureRecord::to("orders")
    .key(&key)
    .payload(&payload)
```

Interview explanation:

> The key is `order_id` because ordering matters per order. Kafka only guarantees ordering inside a partition, so all events for one order should go to the same partition.

### 5.3 Consumer

```rust
let consumer: StreamConsumer = ClientConfig::new()
    .set("group.id", "order-status-service")
    .set("auto.offset.reset", "earliest")
    .set("enable.auto.commit", "false")
    .create()?;
```

Interview explanation:

> I disable auto commit because I want to commit only after processing succeeds. This gives at-least-once delivery. Duplicates are possible, so the handler should be idempotent.

### 5.4 Offset Commit

```rust
handle_message(&message, &orders).await?;
consumer.commit_message(&message, CommitMode::Async)?;
```

Interview explanation:

> Process first, commit after. If the service crashes before commit, Kafka can redeliver the message. That is safer than losing the event.

---

## 6. What To Say About Production

This demo uses an in-memory map. In production:

- store orders in a database
- use transactional outbox when creating orders
- make the consumer idempotent using `event_id`
- add retries with backoff
- send poison messages to a DLQ topic
- track consumer lag
- add tracing and metrics
- use schema management, for example Protobuf or Avro with a schema registry

---

## 7. Common Extensions

If the interviewer asks for more:

1. Add `GET /orders` to list all orders.
2. Add validation failure responses.
3. Add a `processed_event_ids` set for idempotency.
4. Add a retry count and `orders.dlq` topic.
5. Split producer and consumer into separate binaries.

For 45 minutes, do not overbuild. Get one event flowing end to end, then explain the production hardening.
