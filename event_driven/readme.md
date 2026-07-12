# Event-Driven Architecture — Online Food Delivery Platform

Study notes covering BFF, Message Queues, Streams, CQRS, and Saga — built up incrementally the way you'd walk an interviewer through the design (see [ques.md](ques.md) for the prompt).

---

## Table of Contents

1. [Setting the Stage](#1-setting-the-stage)
2. [BFF — Backend for Frontend](#2-bff--backend-for-frontend)
3. [Message Queue](#3-message-queue)
4. [Stream](#4-stream)
5. [CQRS](#5-cqrs--command-query-responsibility-segregation)
6. [Saga](#6-saga)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Quick Reference](#8-quick-reference)

---

## 1. Setting the Stage

Start simple: one client, one API, one database.

```
Customer App → API Server → Database
```

This breaks down fast for a food delivery platform because:

- There are **three very different clients** (customer, restaurant, delivery partner) with different data needs.
- Order **writes** (place/accept/cancel) and order **tracking reads** (status/ETA/location) have opposite load and latency characteristics.
- Some work (sending an email) is **slow and non-critical** — it shouldn't block the request.
- Some events (an order being placed) need to be **observed by many independent services** (analytics, fraud, loyalty) without the order service knowing they exist.
- Placing an order touches **multiple services with no shared database** — a single ACID transaction isn't possible.

Each of the next five sections solves exactly one of these problems. The order below is deliberate: each concept is a little more structurally complex than the last, and later ones build on top of earlier ones.

---

## 2. BFF — Backend for Frontend

### Problem

A single general-purpose API tries to serve the customer app, restaurant dashboard, and delivery app at once. It ends up either over-fetching for some clients or requiring client-side joins/aggregation for others.

### Core Idea

Give each client its **own API layer** (the BFF), tailored to exactly what that client needs. The BFF talks to the underlying domain services and shapes the response per client.

| Client | BFF Responsibilities |
|---|---|
| Customer App | Restaurant listing, cart, order status |
| Restaurant Dashboard | Incoming orders, kitchen queue |
| Delivery App | Pickup location, delivery route |

```
Customer App        →  Customer BFF        ─┐
Restaurant Dashboard →  Restaurant BFF      ─┼─→  Domain Services
Delivery App         →  Delivery BFF        ─┘
```

### Why It Matters Here

Without BFFs, the order service's API would need to satisfy every client's shape at once — leading to bloated payloads or excessive endpoint variants (`?include=`, `?fields=` flags everywhere).

### Tradeoffs

| Benefit | Cost |
|---|---|
| Each client gets an optimal, purpose-built API | More services to deploy and maintain |
| Backend teams can evolve per-client logic independently | Some duplication of aggregation logic across BFFs |
| Reduces over-fetching/under-fetching | Needs clear ownership to avoid becoming a dumping ground |

---

## 3. Message Queue

### Problem

Placing an order should feel instant to the customer, but it triggers several slow, non-critical side effects: sending a confirmation email, an SMS, generating an invoice, processing a receipt image. If these run synchronously inside the request, the user waits on work that has nothing to do with confirming their order.

### Core Idea

Push non-critical, slow work onto a **queue** and let dedicated **workers** consume it asynchronously, off the request path.

```
Order Placed
     │
     ▼
Message Queue
     │
     ├──→ Email Worker
     ├──→ SMS Worker
     └──→ Invoice Worker
```

- A message is typically consumed by **one** worker (point-to-point) — once processed, it's removed from the queue.
- Workers can scale independently and retry failed jobs without affecting the user-facing request.

### Why It Matters Here

The customer gets an immediate "Order Placed" response; email/SMS/invoice generation happen in the background, decoupled from request latency.

### Tradeoffs

| Benefit | Cost |
|---|---|
| Request latency stays low | Eventual, not immediate, side effects (e.g. email arrives seconds later) |
| Workers can retry/back off on failure independently | Requires idempotent consumers (a message might be redelivered) |
| Load spikes are absorbed by the queue instead of the API | Adds an operational component (broker) to run and monitor |

---

## 4. Stream

### Problem

A message queue is great for "one job, one worker." But an order going through its lifecycle (`OrderPlaced → OrderAccepted → PaymentCompleted → DriverAssigned → Delivered`) needs to be observed by **many unrelated services at once**: analytics, loyalty, fraud detection, recommendations, notifications. If the order service had to know about and call each of these directly, every new consumer would mean modifying the order service.

### Core Idea

Publish every significant state change as an **event** onto a durable, ordered, replayable log (e.g. Kafka). Every interested service subscribes independently and processes the same events at its own pace.

```
OrderPlaced → OrderAccepted → PaymentCompleted → DriverAssigned → Delivered
                              │
        ┌─────────────┬───────┼──────────────┬───────────────┐
        ▼             ▼       ▼              ▼               ▼
   Analytics      Loyalty  Notifications  Fraud Detection  Recommendations
```

### Stream vs Message Queue

| | Message Queue | Stream |
|---|---|---|
| Consumption | One consumer per message (competing consumers) | Many independent consumer groups, each reads all events |
| Retention | Message deleted after ack | Log retained for a configurable window — replayable |
| Use case here | "Send this one email" | "Broadcast this order lifecycle event to everyone" |
| Ordering guarantee | Per-queue, not always needed | Per-partition (e.g. per `order_id`) ordering matters |

### Why It Matters Here

New consumers (say, a future "estimated restaurant load" service) can subscribe to the existing event stream without the order service ever changing — this is the backbone that decouples the whole system.

### Tradeoffs

| Benefit | Cost |
|---|---|
| Add new consumers without touching producers | Consumers must handle out-of-order/duplicate events (at-least-once delivery) |
| Replayable — rebuild a service's state from history | Operationally heavier (partitioning, consumer groups, offset management) |
| Natural fit for event sourcing / audit trail | Harder to reason about end-to-end latency across many hops |

---

## 5. CQRS — Command Query Responsibility Segregation

### Problem

Order **writes** (place/accept/cancel) and order **tracking reads** (current status, ETA, driver location, order history) have very different shapes and access patterns. A single model/database optimized for transactional writes is a poor fit for fast, flexible reads — and vice versa.

### Core Idea

Split the system into two independent paths:

```
Command API  →  Write DB  →  publish event ──┐
                                              ▼
                                     Event Stream (§4)
                                              │
                                              ▼
                                  Read Model Projection
                                              │
Query API  ←────────────────────────  Read DB
```

- **Command side**: handles `PlaceOrder`, `AcceptOrder`, `CancelOrder` — validates business rules, writes to a normalized write store, emits an event.
- **Query side**: a projector consumes the event stream and builds a **denormalized read model** (e.g. "current order status + ETA + driver location" as one document) optimized purely for fast reads.

### Why It Matters Here

The write side stays simple and consistent for order mutations. The read side (order tracking, which is polled/queried far more often than orders are placed) can be scaled and shaped independently — even stored in a different database technology (e.g. write in Postgres, read in a fast document/KV store).

### Tradeoffs

| Benefit | Cost |
|---|---|
| Read and write sides scale independently | Read model is eventually consistent (lag after a write) |
| Read model can be denormalized for the exact query pattern | Two models to maintain instead of one |
| Write side stays simple/normalized for correctness | More moving parts (projector, second datastore) |

> **Note:** CQRS depends on the event stream from §4 to propagate writes into the read model — this is why it comes after Stream, not before.

---

## 6. Saga

### Problem

Placing an order isn't one write — it spans multiple services with **no shared database**: reserve food at the restaurant, charge the customer's payment method, assign a delivery partner. A single ACID transaction across these services doesn't exist. If delivery assignment fails after payment succeeded, the system must not end up in an inconsistent state (customer charged, no driver, restaurant still cooking).

### Core Idea

Model the multi-step process as a **saga**: a sequence of local transactions, each publishing an event that triggers the next step. If a step fails, previously completed steps are undone via **compensating actions** — there's no rollback, only forward-moving corrections.

```
Reserve Food → Process Payment → Assign Delivery Partner → Confirm Order

If "Assign Delivery Partner" fails:

Cancel Payment → Release Food Reservation → Notify Customer
```

### Choreography vs Orchestration

| | Choreography | Orchestration |
|---|---|---|
| How steps are triggered | Each service listens for the previous service's event and reacts (uses the stream from §4) | A central saga coordinator explicitly calls each service and tracks state |
| Coupling | Loose — services only know about events, not each other | Coordinator knows the whole flow; services are simpler |
| Best fit | Few steps, simple compensation | Complex flows with many steps/branches |

### Why It Matters Here

This is the most structurally demanding piece: it relies on the **event stream** (§4) to sequence steps and often coexists with **message queues** (§3) for triggering individual compensating actions (e.g. queuing a "send cancellation email" job).

### Tradeoffs

| Benefit | Cost |
|---|---|
| Avoids distributed 2PC transactions across services | Requires designing a compensating action for every step |
| Each service keeps its own local transaction/database | System is only *eventually* consistent during the saga |
| Failures are handled explicitly and visibly | Debugging a partially-completed saga is harder than a single rolled-back transaction |

---

## 7. Putting It All Together

```
                     Customer App
                          │
                     Customer BFF                    (§2 BFF)
                          │
                     Order Service
                          │
                  CQRS Command API                    (§5 CQRS — write side)
                          │
                    Write Database
                          │
                 publish OrderPlaced
                          │
              =========================
                 Event Stream (Kafka)                 (§4 Stream)
              =========================
               │          │           │
          Payment   Restaurant    Delivery
               │          │           │
               +----------+-----------+
                          │
                        Saga                          (§6 Saga — coordinates the 3 services above)
                          │
                Compensation on failure
                          │
              =========================
              Read Model Projection                   (§5 CQRS — read side)
              =========================
                          │
                   CQRS Query API
                          │
                    Customer BFF
                          │
                    Customer App  (real-time tracking)

        Background Tasks (email / SMS / invoice)
                          │
                    Message Queue                     (§3 Message Queue)
                          │
              Email / SMS / Invoice Workers
```

**End-to-end walkthrough** (answers the final part of [ques.md](ques.md)):

1. Customer taps "Place Order" → request hits the **Customer BFF**, shaped for the mobile app.
2. The **CQRS command API** validates and writes the order, then publishes `OrderPlaced` onto the **event stream**.
3. Payment, Restaurant, and Delivery services consume the stream as steps of a **saga**; each local transaction succeeds or triggers compensations on failure.
4. Every event on the stream is also projected into a **read model**, which the **CQRS query API** serves back through the same BFF for real-time order tracking.
5. Independently, `OrderPlaced`/`OrderAccepted`/etc. also fan out to analytics, loyalty, and fraud detection — none of which the order service knows about.
6. Non-critical side effects (email, SMS, invoice) are pushed onto a **message queue** and handled by workers, off the critical request path.

---

## 8. Quick Reference

| Concept | Solves | Key mechanism |
|---|---|---|
| BFF | One API can't fit three different clients | Dedicated API layer per client |
| Message Queue | Slow, non-critical work blocking the request | Async job handed to a worker, one consumer per message |
| Stream | Many independent services need the same events | Durable, ordered, replayable log; many consumer groups |
| CQRS | One data model can't serve heavy writes and heavy reads well | Separate command (write) and query (read) models, linked by events |
| Saga | Multi-service transaction with no shared database | Sequence of local transactions + compensating actions on failure |
