# 05 · Event Driven — async between services

> organs. services that don't wait on each other.
> prev ← [04](04_communication.md) · next → [06 cross-cutting](06_cross_cutting.md)
> deep dive → [../kafka](../kafka/readme.md) · [../redis](../redis/readme.md) · old long-form notes → [_old/](_old/)

## map

```
event driven
├── vocabulary
│   ├── message ── any envelope on the wire (umbrella)
│   │   ├── command ── "do X" (1 receiver, can be rejected) → PlaceOrder
│   │   ├── event ─── "X happened" (past tense, 0..N receivers, fact) → OrderPlaced
│   │   └── query ─── "tell me X" (expects reply)
│   ├── domain event (inside a bounded context) vs integration event (published to other services)
│   └── producer / publisher ── broker ── consumer / subscriber
├── what's inside the event?
│   ├── event notification ── thin: "OrderPlaced id=42", consumer calls back for details
│   ├── event-carried state transfer (ECST) ── fat: full order in event, no callback
│   └── event sourcing ── events ARE the source of truth (state = replay)
├── who receives it? (messaging models)
│   ├── point-to-point / message queue ── 1 message → 1 consumer, deleted after ack
│   ├── publish/subscribe ── 1 message → every current subscriber (fan-out)
│   └── log / stream ── append-only, retained, each consumer group reads at own offset, replayable
├── processing mode (siblings)
│   └── batch (chunks on schedule) | micro-batch (small chunks, seconds) | stream (each event as it comes)
├── infra type
│   ├── message broker ── smart broker, dumb consumer: routing, per-msg ack, delete on consume (RabbitMQ, ActiveMQ, SQS)
│   ├── log / event streaming platform ── dumb broker, smart consumer: offsets, retention, replay (Kafka, Pulsar, Kinesis)
│   ├── event bus / router ── rules route events to targets (EventBridge, Event Grid)
│   └── lightweight ── Redis Pub/Sub (no persistence) | Redis Streams | NATS
├── mechanics
│   ├── ack / nack | visibility timeout | redelivery
│   ├── retry (immediate → retry topic w/ delay) → DLQ (dead letter queue) after N
│   ├── poison message ── always fails, blocks head of queue/partition → DLQ it
│   ├── competing consumers ── N workers on 1 queue, scale out
│   ├── consumer group ── kafka: partitions split among group members
│   ├── partition + key ── ordering only within partition (key = orderId)
│   ├── offset / commit ── consumer's bookmark in the log
│   ├── retention (time/size) | log compaction (keep latest per key)
│   ├── backpressure / lag ── consumer slower than producer
│   └── schema registry ── versioned event contracts (avro/protobuf)
├── delivery guarantees
│   ├── at-most-once ── may lose, never dup (ack before process)
│   ├── at-least-once ── never lose, may dup (ack after process) ← default
│   ├── exactly-once ── really "effectively once" = at-least-once + idempotent/dedupe
│   └── idempotent consumer | inbox table (dedupe by message id)
├── consistency patterns (db + messaging)
│   ├── dual-write problem ── write db AND publish, crash between = lost event
│   ├── transactional outbox ── event row in same db tx → relay/CDC publishes
│   ├── inbox ── consumer stores processed ids → dedupe
│   ├── CDC ── read db's commit log (WAL/binlog) → events (Debezium)
│   ├── listen to yourself ── publish first, then consume your own event to update db
│   ├── CQRS ── separate write model & read model (read model built from events)
│   ├── materialized view / projection ── precomputed read shape from events
│   └── event sourcing ── store events not state, snapshots for speed
├── multi-service transaction (siblings)
│   ├── 2PC ── coordinator: prepare all → commit all (locks, blocking, rare)
│   ├── 3PC ── 2PC + extra phase to reduce blocking (theory mostly)
│   ├── saga ── local tx per service + compensating action on failure
│   │   ├── orchestration ── central coordinator tells each step (sync REST or async commands)
│   │   └── choreography ── each service reacts to previous event, no center
│   ├── TCC (try-confirm-cancel) ── reserve → confirm or cancel (booking style)
│   └── workflow engine / process manager ── durable orchestrator (Temporal, Camunda, Step Functions)
└── interaction shapes across services
    ├── fire & forget ── publish, done
    ├── async request/reply over broker ── reply-to queue + correlation id
    ├── async over sync ── POST → 202 + status url → client polls
    ├── callback / webhook ── POST → 202 → callee calls your url when done
    ├── scatter-gather ── ask many, aggregate replies
    └── claim check ── big payload in blob store, only reference in message
```

## messaging models (siblings)

| | queue (p2p) | pub/sub | log / stream |
|---|---|---|---|
| 1 message goes to | 1 consumer | all current subscribers | every consumer group (each reads all) |
| after read | deleted | gone (delivered) | stays till retention |
| late joiner sees old msgs? | only unconsumed | ✗ | ✓ replay from offset |
| ordering | FIFO-ish (breaks with competing consumers) | none | per partition |
| mental model | "do this job once" | "shout it" | "write it in the ledger" |
| use | background jobs (email, invoice) | notifications fan-out | event backbone, analytics, CQRS feeds |
| examples | SQS, RabbitMQ queue | SNS, Redis Pub/Sub, rabbit fanout | Kafka, Kinesis, Pulsar, Redis Streams |

> tell: new consumer tomorrow needs history? → stream. only new ones? → pub/sub. exactly one worker once? → queue.

## broker vs pub/sub vs stream (the confusing trio)

| term | category | one-liner |
|---|---|---|
| message broker | infrastructure | the server in the middle (rabbit, activemq, sqs) |
| queue / pub-sub | messaging pattern | how a broker delivers (1 vs many) — rabbit does both |
| stream / log | storage model | messages retained & replayable, consumer tracks offset |
| event bus | routing layer | rules-based routing of events to targets |
| kafka | log (distributed commit log / WAL) | can act as pub/sub (many groups) AND queue (1 group, many members) |

## batch vs stream (siblings)

| | batch | micro-batch | stream |
|---|---|---|---|
| unit | big chunk on schedule | small chunk, seconds | each event |
| latency | minutes–hours | seconds | ms |
| tools | Spark, Hadoop, cron + SQL | Spark Structured Streaming | Flink, Kafka Streams |
| good for | reports, billing, ETL | near-realtime dashboards | fraud, alerts, live tracking |

## event payload (siblings)

| | notification | ECST | event sourcing |
|---|---|---|---|
| carries | id + type | full state | the change itself |
| consumer needs callback? | yes | no | no |
| source of truth | db | db | event log |
| cost | sync coupling back | fat events, schema = public contract | complex reads, one-way door |

## saga: orchestration vs choreography

| | orchestration | choreography |
|---|---|---|
| control | central coordinator | none, services react to events |
| transport | sync REST or async commands | events on broker |
| coupling | coordinator knows all | services know only events |
| visibility | flow in one place | flow smeared across services |
| fits | many steps, branching | few steps, simple |

## ways to not lose an event

| pattern | one-liner |
|---|---|
| outbox | write event in same tx as data, relay publishes later |
| CDC | tail db log, emit changes (can implement outbox) |
| inbox | consumer dedupes by message id |
| idempotent consumer | processing twice = same result (upsert, keys) |
| DLQ | park failures for inspection instead of losing/looping |

## rough Q&A (from old notes)

- queue reads in order? ── single consumer FIFO yes · competing consumers no strict order · SQS standard best-effort, SQS FIFO per group · kafka per partition only.
- one msg stuck blocks queue? ── poison msg → retry N → DLQ · kafka partition head-of-line blocks → retry topics + DLQ.
- kafka = queue or stream or WAL? ── WAL/log at heart; stream by usage; queue-like via consumer group.
- stream vs queue? ── many independent readers + replay + order → stream; job distribution → queue.
- event driven vs request driven? ── need answer now → request; inform others / decouple → event.

## example: food delivery (one-liners)

| need | pattern |
|---|---|
| 3 client types want diff shapes | BFF (→ 09) |
| email/sms/invoice off request path | message queue + workers |
| analytics, loyalty, fraud all watch orders | stream (kafka topic `orders`) |
| tracking reads ≫ order writes | CQRS read model from stream |
| food + payment + driver across 3 dbs | saga (compensate: refund, release) |
| order saved but event lost on crash | outbox |
| redelivered msg → double sms | idempotent consumer |
