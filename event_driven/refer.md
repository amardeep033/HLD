# The Order That Refused to Block: How Every Bottleneck Built an Event-Driven System

Event-driven architecture looks like a pile of buzzwords — BFF, queues, streams, CQRS, sagas — until you notice something: almost every idea in it exists because a synchronous, one-database system hit a wall.

So let's build a food delivery backend (Swiggy/Zomand-style) ourselves. Every time it breaks, we'll fix exactly one problem. By the end, we'll have accidentally recreated the core ideas behind event-driven systems.

## It Starts With One API

One backend server, one database. Customer app calls it, restaurant dashboard calls it, delivery app calls it. Every request goes through the same endpoints, reads the same tables.

Simple. Nothing async, nothing eventual. Every response is exactly as fresh as the database.

Enjoy it. It's the last time anything is simple.

---

## 1. Three Clients, One API

The customer app wants restaurants, cart, and order status. The restaurant dashboard wants incoming orders and a kitchen queue. The delivery app wants pickup location and route. One API tries to serve all three.

At first you add optional fields: `?include=cart`, `?view=kitchen`. Six months later every endpoint has a dozen flags, half the clients over-fetch data they don't render, and a single response schema change breaks two apps you weren't even thinking about.

The real issue: **three different clients are not the same consumer**, and forcing them through one general-purpose contract makes every one of them worse.

The fix — give each client its own front door:

```
Customer App        →  Customer BFF        ─┐
Restaurant Dashboard →  Restaurant BFF      ─┼─→  Domain Services
Delivery App         →  Delivery BFF        ─┘
```

> 📦 **BFF (Backend for Frontend)** — a thin API layer dedicated to one client, shaped exactly to what that client needs, sitting in front of shared domain services.

**What people confuse this with:** an API Gateway. A gateway is one layer that routes/authenticates/rate-limits traffic for *everyone* — it doesn't reshape responses per client. A BFF is the opposite instinct: multiple thin layers, each biased toward a single consumer's needs. You often have both — a gateway in front of several BFFs.

**When you reach for it:** you have genuinely different client shapes (mobile vs dashboard vs partner app) and a shared API is accumulating client-specific branches. **When you don't:** one client type, or clients that all want roughly the same shape — a BFF here is just an extra hop for nothing.

---

## 2. The Request That Wouldn't Return

Order placed. Before the app gets its "success" response, the server also sends a confirmation email, a driver-matching SMS, generates an invoice PDF, and processes a receipt image. Each of those is slow — an email provider can take a second, invoice rendering another two.

The customer's phone sits there spinning for work that has nothing to do with "was my order accepted."

We don't need those things to happen *before* we respond. We need them to happen — eventually, reliably — without the customer paying for the wait.

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

> 📦 **Message Queue** — a buffer that holds a unit of work until a worker is free to process it. Each message is consumed by exactly one worker, then removed.

The request handler's job shrinks to: validate, save, push a message, return. Everything slow happens off to the side, on someone else's clock.

One catch: a worker can crash *after* sending the email but *before* acknowledging the message, and the broker will redeliver it. So the email worker has to be **idempotent** — sending the same "order confirmed" email twice must be harmless, not a customer getting five copies.

**What people confuse this with:** a background thread or `setTimeout`. Those live and die with the process — a server restart loses the work. A queue is durable and external to any one server; if every server dies, the messages are still sitting there waiting.

**When you reach for it:** work is slow, non-critical to the immediate response, and can tolerate a short delay. **When you don't:** the caller genuinely needs the result synchronously (e.g. "is this payment authorized") — queuing that just adds a round trip with extra steps.

---

## 3. Everyone Wants to Know What Just Happened

The order service is humming along, queuing background jobs, returning fast responses. Then the analytics team asks: "can we get notified every time an order is placed?" Then loyalty: "we need the same thing, plus `Delivered` events." Then fraud detection. Then recommendations.

The obvious move is to have the order service call each of these directly — `notifyAnalytics()`, `notifyLoyalty()`, `notifyFraud()` — right after saving the order. But now the order service's code changes every time some *other* team builds a new feature, and if the fraud service is slow or down, does that block placing an order? It shouldn't, but now you have to remember to guard against it, every single time.

The order service shouldn't need to know who's listening. It should just announce what happened.

```
OrderPlaced → OrderAccepted → PaymentCompleted → DriverAssigned → Delivered
                              │
        ┌─────────────┬───────┼──────────────┬───────────────┐
        ▼             ▼       ▼              ▼               ▼
   Analytics      Loyalty  Notifications  Fraud Detection  Recommendations
```

> 📦 **Stream** — a durable, ordered, replayable log of events. Producers append; any number of independent consumer groups read the whole log at their own pace, without the producer knowing they exist.

**What people confuse this with:** the message queue from §2. They look similar — both move messages between services — but the consumption model is opposite:

| | Message Queue | Stream |
|---|---|---|
| Who consumes a message | One worker (competing consumers) | Every subscribed consumer group, independently |
| After it's read | Removed | Stays in the log (until retention expires) |
| Mental model | "Do this job, once" | "Broadcast this fact, to whoever's listening" |

🍬 **Fun fact:** Kafka isn't really a message queue at all — it's a distributed write-ahead log. Producers append; every consumer group replays it independently, from whatever offset it wants, at its own pace. Once you see Kafka as a log rather than a queue, its "weird" behaviors — consumer groups, offset tracking, replaying old messages — stop being weird.

**When you reach for it:** multiple independent services need to react to the same sequence of events, and you want to add new consumers without touching the producer. **When you don't:** there's exactly one consumer and the work is a one-off job — that's just §2's queue, don't reach for a whole log for it.

> 🔑 **Mental model so far.** Everything up to here answers one question: *how do services talk to each other without blocking or coupling?* Direct call → too tightly coupled and synchronous. Queue → fine for one consumer, one job. Stream → the answer once *many* consumers care about the *same* sequence of facts.

---

## 4. The Tracking Screen Is Always a Little Behind — and Always Under Load

The order stream now exists. One of its consumers rebuilds a "current order state" view — status, ETA, driver's live location — that the customer's tracking screen polls every few seconds. That's a *lot* more reads than writes: one `PlaceOrder` command per order, but hundreds of tracking reads over the order's lifetime.

Try serving both from the same table and the same model, and you feel the tension immediately. The write side wants a normalized schema you can validate business rules against (`orders`, `order_items`, `payments`, foreign keys, transactions). The read side wants one flat, denormalized shape — "everything the tracking screen needs" in a single fetch — and it wants it fast, under high concurrent load, without joins.

One model, two jobs, pulling in opposite directions.

```
Command API  →  Write DB  →  publish event ──┐
                                              ▼
                                     Event Stream (§3)
                                              │
                                              ▼
                                  Read Model Projection
                                              │
Query API  ←────────────────────────  Read DB
```

> 📦 **CQRS (Command Query Responsibility Segregation)** — split the write model (commands: `PlaceOrder`, `AcceptOrder`, `CancelOrder`, validated and persisted normally) from the read model (a denormalized projection, rebuilt by consuming the same event stream that already exists from §3).

This is the moment the stream stops being "just for other teams" and becomes load-bearing infrastructure for your *own* tracking feature. The write side emits `OrderPlaced`; a projector — just another stream consumer — folds that event into a read-optimized document; the query API serves straight from it.

**What people confuse this with:** just "having a read replica." A read replica is the same schema, copied — still normalized, still shaped for writes, just less loaded. CQRS's read model is a *different shape entirely*, built by replaying events, not by copying rows.

**When you reach for it:** read and write patterns diverge enough that one schema serves neither well, and you already have (or need) an event stream to keep the read side updated. **When you don't:** reads and writes are roughly symmetric and low-volume — CQRS adds a second datastore and eventual-consistency lag for no payoff.

The cost is real, though: the read model lags the write model by however long the projector takes to catch up. Place an order and refresh the tracking screen in the same instant, and you *might* briefly see nothing — the write succeeded, the projection hasn't landed yet.

---

## 5. One Order, Three Databases, No Shared Transaction

Placing an order isn't one write. It's: reserve food at the restaurant's system, charge the customer via a payment provider, assign a delivery partner from the delivery service. Three separate services, three separate databases. There is no `BEGIN TRANSACTION` that spans all of them.

Say the food reservation succeeds, the payment succeeds — and then no delivery partner is available. The naive systems now has a customer who was charged, for food being cooked, that nobody will ever pick up. Nothing crashed. Nothing threw an error. The *business logic* is just inconsistent, and a normal database rollback can't fix it because it never had all three pieces in one transaction to roll back.

You can't lock three services together and commit atomically. So instead: treat it as a sequence of local transactions, and if a later step fails, explicitly undo the earlier ones.

```
Reserve Food → Process Payment → Assign Delivery Partner → Confirm Order

If "Assign Delivery Partner" fails:

Cancel Payment → Release Food Reservation → Notify Customer
```

> 📦 **Saga** — a sequence of local transactions across services, coordinated via events (§3) or an orchestrator, where each step has a matching **compensating action** to undo it if a later step fails. There's no rollback — only forward-moving corrections.

Two ways to wire the steps together:

| | Choreography | Orchestration |
|---|---|---|
| How steps trigger | Each service listens for the previous service's event on the stream and reacts | A central coordinator explicitly calls each service and tracks state |
| Coupling | Loose — services only know about events | Coordinator knows the whole flow; services stay simple |
| Good fit | Few steps, simple compensations | Many steps, branching failure paths |

**What people confuse this with:** two-phase commit (2PC). 2PC tries to make a distributed transaction *atomic* — everyone commits or everyone aborts, together, while holding locks and waiting on a coordinator. A saga gives up atomicity on purpose: each step commits immediately and permanently, and inconsistency is allowed to exist *briefly* — fixed afterward by compensation, not prevented by locking.

**When you reach for it:** a business operation spans multiple services/databases with no shared transaction, and you can define a compensating action for each step. **When you don't:** everything touched lives in one database — just use a real transaction, a saga there is solving a problem you don't have.

---

## The Whole Picture

Look back at what forced each idea into existence.

Three clients wanted different shapes → **BFF**. Slow side-work was blocking the response → **Message Queue**. Many independent services needed to react to the same facts → **Stream**. Reads and writes pulled the data model in opposite directions → **CQRS**, built on top of the stream. A single operation spanned three databases with no shared transaction → **Saga**, coordinated over the same stream.

```
                     Customer App
                          │
                     Customer BFF                    (§1 BFF)
                          │
                     Order Service
                          │
                  CQRS Command API                    (§4 CQRS — write side)
                          │
                    Write Database
                          │
                 publish OrderPlaced
                          │
              =========================
                 Event Stream (Kafka)                 (§3 Stream)
              =========================
               │          │           │
          Payment   Restaurant    Delivery
               │          │           │
               +----------+-----------+
                          │
                        Saga                          (§5 Saga)
                          │
                Compensation on failure
                          │
              =========================
              Read Model Projection                   (§4 CQRS — read side)
              =========================
                          │
                   CQRS Query API
                          │
                    Customer BFF
                          │
                    Customer App  (real-time tracking)

        Background Tasks (email / SMS / invoice)
                          │
                    Message Queue                     (§2 Message Queue)
                          │
              Email / SMS / Invoice Workers
```

Every event-driven system you'll ever read about — order platforms, ride-hailing, ticketing — answers the same underlying questions:

- **Who talks to whom, and shaped how?** → BFF
- **What work can happen later, off the request path?** → Message Queue
- **What facts does more than one service need to react to?** → Stream
- **Does one data model actually serve both reads and writes well?** → CQRS
- **Does this operation span services with no shared transaction?** → Saga

The interesting part was never that these five things exist. It's *which pain forced each one into being* — once you can trace that chain, the architecture stops looking like a buzzword pile.

It starts looking inevitable.
