# The Architecture That Kept Forking: A Mental Map of Event-Driven Systems

Every "event-driven architecture" diagram you've seen — queues, topics, streams, sagas, CQRS — looks like a random pile of tools. It isn't. It's a series of **forks**: at each point, the simple option ran into a specific limitation, and picking a fix meant picking a side.

This isn't about one system (no food app, no specific example holding it together). It's the decision tree itself — so that when *any* HLD question puts a wall in front of you, you already know which fork you're standing at, and what your options are.

So let's start from nothing and fork our way to the whole map.

---

## 0. The Starting Point: One Synchronous Call

Service A needs something from Service B. The obvious move: A calls B directly and waits.

```
Service A ──── call ────▶ Service B
Service A ◀─── result ──── Service B
```

Simple, easy to trace, easy to debug — a stack trace spans both services. This is **request-driven architecture**, and it's the correct default until it isn't.

It stops being correct the moment any one of these becomes true:
- B is slow, and A doesn't actually need the result *right now*.
- B is temporarily down, and A shouldn't fail just because B did.
- More than one service needs to know what A just did.
- A and B are owned by different teams who don't want to be coupled to each other's uptime.

Every fork below exists because one of those four became true.

---

## 1. Fork — Does the Caller Need to Wait?

> 🔑 **The first fork.** This is the one decision every architecture makes before anything else: **sync or async**.

### 1.1 Sync — Request-Driven

A calls B, blocks, gets a result. REST, gRPC, GraphQL queries — all the same shape underneath.

> 📦 **Request-Driven Architecture** — A calls B and waits for a direct response. A's success is coupled to B's availability and latency, right now.

**Reach for it when:** A genuinely needs B's answer before it can proceed (checking if a payment is authorized). **Don't** when A just needs to *inform* B something happened and doesn't care about B's response.

### 1.2 Async — Event-Driven

A does its own work, announces what happened, and moves on. Whoever needs to react, reacts later, on their own schedule.

```
Service A ──── publish event ────▶  [ broker ]  ──── deliver ────▶ Service B
Service A already moved on. B reacts whenever it can.
```

> 📦 **Event-Driven Architecture** — services communicate by producing and consuming events through an intermediary, rather than calling each other directly. Nobody blocks on anybody.

**Reach for it when:** the caller doesn't need an immediate answer, or more than one consumer might care. **Don't** when you need a synchronous answer to continue — async here just adds a polling loop to fake what a direct call already gives you for free.

**What people confuse this with:** "async" meaning `async/await` in code. That's just non-blocking *syntax* on a single machine. Event-driven architecture is about decoupling across *services* — through a broker, with its own delivery and ordering guarantees, independent of either service being up.

Once you're in the async branch, a second fork immediately appears — because "announce it to a broker" still leaves open the question of who's listening.

---

## 2. Fork — Who Receives the Message?

Three shapes exist, and mixing them up is the single most common design mistake in event-driven systems.

```
                     Async Message
                          │
        ┌─────────────────┼─────────────────┐
        ▼                 ▼                 ▼
  Point-to-Point      Publish/Subscribe    Log / Stream
   (Message Queue)      (Pub/Sub)           (Kafka-style)
```

### 2.1 Point-to-Point — Message Queue

One message, one consumer. Once processed, it's gone.

> 📦 **Message Queue** — A holds a job until a worker is free; exactly one worker consumes each message, then it's removed. (SQS, RabbitMQ work queues.)

**Use it for:** distributing a pile of independent jobs across a worker pool — image processing, sending a single email, running a report. You want the work done *once*, by *whoever's free*.

### 2.2 Publish/Subscribe — Topics

One message, broadcast to every subscriber that happens to be listening *right now*. No listener, no delivery (in the simple form).

> 📦 **Publish/Subscribe** — a producer publishes to a topic; every currently-subscribed consumer gets a copy. Fire-and-forget, generally no replay. (SNS, RabbitMQ fanout exchange.)

**Use it for:** fan-out notifications — "tell every interested service this happened" — where a missed message during downtime is acceptable or handled another way (e.g. webhooks, push notifications).

### 2.3 Log / Stream — Kafka-style

Every message is appended to a durable, ordered log. Consumers don't "take" messages — they read the log at their own offset, at their own pace, and can rewind.

> 📦 **Stream** — a durable, ordered, replayable log. Many independent consumer groups each read the entire log, tracking their own position. Nothing is removed on read.

**Use it for:** anything where (a) multiple independent consumers need the *same* sequence of events, (b) ordering matters, or (c) you want to replay history (rebuild a service, backfill a new consumer that joined late).

🍬 **Fun fact:** Kafka is not a message queue with extra features bolted on — it's fundamentally a distributed **write-ahead log**. That single reframe explains everything that seems odd about it: why consumer groups exist, why messages aren't deleted on read, why replay is trivial.

> 🔑 **The tell that separates them.** Ask: *if a second consumer subscribes tomorrow, does it need every message that already happened, or only new ones?* Needs history → stream. Only new ones, fire-and-forget → pub/sub. Needs exactly one worker to do it, once → queue.

---

## 3. Fork — What Do You Build *On Top Of* the Event Stream?

Having a stream of events is infrastructure. What you do with it is architecture. Two more forks live here.

### 3.1 What's Inside the Event?

```
              Event Payload
                    │
        ┌───────────┴───────────┐
        ▼                       ▼
  Event Notification    Event-Carried State Transfer
  "OrderPlaced: id=42"   "OrderPlaced: {full order object}"
```

> 📦 **Event Notification** — a thin event carrying just an ID/fact; consumers call back to fetch details if they need them. Keeps events small, but reintroduces a synchronous dependency on the source service.

> 📦 **Event-Carried State Transfer** — the event carries the full data the consumer would need. No callback required, but the schema is now a public contract, and consumers can drift from the source's current truth.

**When you reach for state transfer:** consumers need to stay fully decoupled, even from the producer's uptime, and can tolerate slightly stale copies. **When notification is enough:** the event is rare enough, or the payload big/sensitive enough, that a callback is cheap and freshness matters more than decoupling.

### 3.2 Is the Event Log Just a Side Effect, or the Actual Source of Truth?

```
Normal:      Database is truth   →   events are a side effect of writes
Inverted:    Event log is truth  →   the database is a side effect (a view) of the events
```

> 📦 **Event Sourcing** — instead of storing current state, store the full sequence of events that led to it. Current state is derived by replaying events (or reading a cached snapshot). The log *is* the database.

**Reach for it when:** you need a full audit trail by construction, the ability to answer "what did this look like at any point in time," or the ability to rebuild completely new read models later just by replaying history. **Don't** when you just need current state — event sourcing makes every read a (potentially expensive) derivation, and it's a one-way architectural door: hard to bolt on later, hard to remove.

Once you accept "the read side is *derived from* events" (whether via event sourcing or a simpler projection), a bigger structural fork opens: should the same model even serve both reads and writes?

### 3.3 One Model, or Two?

```
Command API  →  Write DB  →  publish event ──┐
                                              ▼
                                       Event Stream
                                              │
                                              ▼
                                  Read Model Projection
                                              │
Query API  ←────────────────────────  Read DB
```

> 📦 **CQRS** — separate the write model (validates and persists commands) from the read model (a denormalized projection rebuilt by consuming events). Two shapes, two datastores, connected by the stream.

**Reach for it when:** read and write access patterns genuinely diverge — different volume, different shape, different scaling needs — and you already have (or need) an event stream to keep the read side in sync. **Don't** when reads and writes are symmetric and low-volume; CQRS buys you nothing but a second datastore and eventual-consistency lag.

**What people confuse this with:** a read replica. A replica is the *same* schema, just copied for load — still shaped for writes. CQRS's read model is a *different shape entirely*, built by replaying events, not copying rows.

---

## 4. Fork — The Database Write and the Event Publish Aren't Atomic

Here's a trap that catches almost everyone the first time they wire up events: you write to your database, then publish an event about it. Two separate operations. What if the process crashes between them?

```
1. Save to DB         ✅ succeeded
2. Publish event      ❌ crash before this line runs
```

Now the database says the order exists, but nobody downstream ever finds out. This is the **dual-write problem** — two systems (DB and broker) that need to agree, with no shared transaction between them. (Notice: this is the exact same shape of problem as §5 below, just at a smaller scale — one operation, two resources instead of one operation, several services.)

```
              Write DB
                 │
         insert into outbox table
           (same local transaction)
                 │
      background poller / CDC reads outbox
                 │
            publish to stream
                 │
          mark outbox row as sent
```

> 📦 **Outbox Pattern** — write the event to an "outbox" table in the *same local transaction* as the business write. A separate poller (or **Change Data Capture**, reading the DB's own commit log) reliably publishes outbox rows to the stream afterward. The atomicity you actually needed lives inside the single-database transaction you already had.

**Reach for it whenever** you publish an event as a side effect of a database write and can't afford to silently drop it. It's cheap insurance, not a heavyweight pattern — usually just one extra table.

---

## 5. Fork — One Business Operation, Many Services, No Shared Transaction

Scale the dual-write problem up: instead of one DB + one broker, you have three or four *services*, each with its own database, and one business operation needs all of them to agree.

```
                Distributed Transaction
                         │
        ┌────────────────┴────────────────┐
        ▼                                  ▼
       2PC                                Saga
  (lock everyone,               (local transactions in
   commit together)              sequence + compensation)
```

> 📦 **Two-Phase Commit (2PC)** — a coordinator asks every participant to "prepare" (lock resources), then tells everyone to commit only if all say yes. Atomic, but participants hold locks while waiting on a coordinator and on each other — it doesn't survive a slow or dead participant well, and most modern distributed systems avoid it outright.

> 📦 **Saga** — each service commits its own local transaction immediately; if a later step fails, previously completed steps are undone via **compensating actions** defined up front. No locks held across services, no atomicity — just a guarantee that failure paths are handled explicitly.

**What people confuse this with:** a saga isn't "eventually consistent, so we don't need to think about failure." It's the opposite — you're forced to design an explicit compensating action for *every single step*, which is more upfront design work than 2PC, in exchange for never locking anyone.

Within sagas, there's one more fork — *how* do the steps find out about each other:

| | Choreography | Orchestration |
|---|---|---|
| How steps trigger | Each service reacts to the previous step's event (needs a stream — §2.3) | A central coordinator explicitly calls each service and tracks state |
| Coupling | Loose — services only know events, not each other | Coordinator knows the whole flow; services stay dumb |
| Debuggability | Hard to see the whole flow — it's smeared across services | Easy — the coordinator's state machine *is* the flow |
| Good fit | Few steps, simple compensations | Many steps, branching/conditional failure paths |

**Reach for a saga when:** a business operation spans services with independent databases and you can define a compensation per step. **Don't** when everything touched lives in one database — that's just a transaction, not a saga.

---

## 6. Cross-Cutting Fork — What Happens If a Message Is Delivered Twice, or Never?

This fork doesn't sit at one point in the map — it applies to every queue, stream, and saga step above. Networks retry. Brokers redeliver on ambiguous acks. You have to pick a delivery guarantee and design for it.

```
At-most-once   →  might lose a message, never duplicates      (fire-and-forget)
At-least-once  →  might duplicate a message, never loses one  (the common default)
Exactly-once   →  the ideal; in practice usually "effectively-once" via dedup
```

> 📦 **Idempotency** — designing a consumer so that processing the same message twice has the same effect as processing it once (e.g. keyed by an idempotency key / message ID, upserts instead of increments).

**The rule of thumb:** almost every real system runs on **at-least-once delivery + idempotent consumers**, because true exactly-once delivery across independent systems is either impossible or absurdly expensive to guarantee. If you remember one thing from this section: never write a consumer that assumes a message arrives exactly once, no matter what the broker's marketing claims.

---

## 7. Fork — Serving Many Different Clients

This one lives at the *edge* of the system, orthogonal to everything above — it's about how external clients reach your services, not how services talk to each other.

```
                  How do clients reach services?
                              │
        ┌──────────────┬──────┴───────┬──────────────┐
        ▼              ▼              ▼              ▼
   Single API    API Gateway         BFF          GraphQL
  (call services  (one shared      (one API per   (one flexible
   directly)       edge for all    client type)    query layer,
                   clients: auth,                   client picks
                   rate-limit,                       the shape)
                   routing)
```

| | Single/Shared API | API Gateway | BFF | GraphQL |
|---|---|---|---|---|
| Optimizes for | Nothing — simplest to build | Cross-cutting concerns (auth, rate limiting, routing) for *all* clients uniformly | Per-client shape (mobile vs dashboard vs partner) | Per-request shape, decided by the client itself |
| Breaks down when | Clients have genuinely different needs | Clients still fight over response shape | Duplicated aggregation logic across BFFs | Over-fetching/under-fetching protection shifts to query complexity/caching concerns |

**Reach for a BFF when:** you have a handful of distinct client *types* with stable, different needs (mobile app, internal dashboard, partner API). **Reach for GraphQL when:** clients' needs vary per-screen/per-request in ways too fine-grained for a fixed set of BFFs. **An API Gateway isn't a replacement for either** — it's the layer in front of all of them handling auth/rate-limiting/routing; you can have a gateway routing to several BFFs behind it.

---

## The Whole Mind Map

```
Two services need to interact
│
├─ Sync (§1.1) ─────────────────────────── Request-Driven (REST/gRPC)
│
└─ Async (§1.2) ─ Event-Driven
   │
   ├─ Who receives it? (§2)
   │  ├─ One worker, once ──────────────── Message Queue
   │  ├─ Whoever's listening now ───────── Pub/Sub
   │  └─ Everyone, replayable, ordered ─── Stream / Log
   │
   ├─ What's in the event? (§3.1)
   │  ├─ Just an ID ─────────────────────  Event Notification
   │  └─ Full payload ───────────────────  Event-Carried State Transfer
   │
   ├─ Is the log the source of truth? (§3.2)
   │  ├─ No, DB is truth, events are a side effect ─ (normal case)
   │  └─ Yes, replay events to get state ─────────── Event Sourcing
   │
   ├─ One model or two? (§3.3)
   │  ├─ One ────────────────────────────  plain CRUD
   │  └─ Split ───────────────────────────  CQRS (write model + projected read model)
   │
   ├─ DB write + event publish atomic? (§4)
   │  └─ No ─────────────────────────────  Outbox Pattern / CDC
   │
   └─ One operation, many services, no shared transaction? (§5)
      ├─ Lock everyone, commit together ── 2PC (rare in practice)
      └─ Local txns + compensation ─────── Saga
         ├─ Services react to events ───── Choreography
         └─ Central coordinator ────────── Orchestration

Cross-cutting, always ────────────────────  Delivery guarantees + Idempotency (§6)

Orthogonal, at the edge ───────────────────  Client-facing API layer (§7):
                                             Single API / Gateway / BFF / GraphQL
```

**How to use this in an interview:** when a question hands you a wall — "the caller doesn't need to wait," "three services need to agree," "reads and writes have different load profiles," "three client types want different data" — don't reach for a memorized pattern. Find which fork above matches the wall you just hit, state the two options at that fork, and justify the side you pick. That's the whole game: not knowing that CQRS exists, but knowing *which pain* makes you reach for it over the alternative sitting right next to it on the map.
