# HLD Study Pattern

This file defines exactly how every HLD topic is studied and how its folder is structured.
When asked to create a folder for a topic, read this file fully and follow every instruction.

---

## Study Pattern (Per Topic)

Every topic moves through 4 stages in order. Do not skip ahead — each stage builds on the previous one.

**Stage 1 — Theory**
Read about the topic. Understand what problem it solves and why it was invented. You are not designing anything yet — you are just building a mental model.

**Stage 2 — Notes (`notes.md`)**
Write your understanding in your own words. Fill in the notes.md file using the instructions below. If you cannot fill a section, you don't understand that part yet — go back to Stage 1 for that gap.

**Stage 3 — Design (`design.md`)**
Apply the full design walkthrough. Trace a request. Justify every decision. This is the interview answer you would give on a whiteboard.

**Stage 4 — Code + Q&A (`rust/` + `qa.md`)**
Implement the core logic in Rust. Then write the interview Q&A document. Only do this stage for topics where implementation is meaningful (see folder structure rules below).

---

## Folder Structure

When creating a folder for a new topic, create it at the root of this workspace with the slugified topic name (lowercase, hyphens). Inside it, create exactly these files:

```
{topic}/
  notes.md
  design.md
  algos/          ← only if the topic has multiple named algorithms or strategies
    1_{algo_name}.md
    2_{algo_name}.md
    ...
  rust/           ← only if the topic has implementable logic
    Cargo.toml
    src/
      main.rs
      {module}.rs
      ...
  qa.md
```

Create `algos/` only when the topic has distinct named algorithms or strategy variants worth comparing side by side (e.g. rate limiting has 5 algorithms; caching has LRU, LFU, ARC; load balancing has round-robin, least connections, consistent hashing). If the topic is a pure concept (e.g. CAP theorem, CQRS) skip `algos/`.

Create `rust/` only when the core logic is implementable in code — counters, data structures, state machines, queues. Skip it for pure architectural patterns (e.g. blue-green deploy, strangler pattern) where there is nothing meaningful to implement.

---

## File-by-File Instructions

### `notes.md` — What it is and why you write it first

This is the first file you fill in. Purpose: force yourself to understand the topic in plain language before you touch design or code. You cannot design something you haven't understood. Writing notes exposes gaps immediately.

Sections to include and why each matters:

**What problem does this solve?**
Write the pain without this topic. Why was it invented. What breaks or degrades without it. This anchors everything that follows — every design decision you make later should trace back to this problem.

**Core idea in one paragraph**
Explain it like you're telling a colleague who has never heard of it. No bullet points. Full sentences. This forces you to actually understand it rather than copy-paste definitions. If you can't write this paragraph, you don't know it yet.

**Key components / moving parts**
Name the components (e.g. for circuit breaker: closed state, open state, half-open state, threshold, timeout). One line per component. Explain what each one does and why it exists — what would break if that component was removed.

**When to use**
List the signals/conditions that tell you this topic is the right tool. Be specific — not "use when you need reliability" but "use when a downstream service has unpredictable latency spikes and you need to stop making calls to it before your thread pool fills up."

**When NOT to use**
Anti-patterns. Cases where this adds complexity with no benefit. This section is as important as "when to use" because interviewers probe here to see if you know the limits.

**Real-world examples**
Name actual systems, companies, or libraries that use this. Concrete examples make abstract concepts stick and give you interview talking points (e.g. "Netflix uses Hystrix for circuit breaking", "AWS API Gateway uses token bucket for rate limiting").

**Rough tradeoffs**
Two or three sentences on what you give up. Not a pros/cons table — a narrative. Tradeoffs reveal understanding. "This gives you smooth traffic but increases tail latency because requests queue instead of failing fast."

---

### `design.md` — What it is and why you write it second

This is the deep design document. Purpose: simulate an actual system design interview answer. You write it after notes so you already have the mental model and are now applying it to a scaled, production-grade system. This is where you stop thinking about the concept and start thinking about the system.

Sections to include and why each matters:

**Prerequisites / Setup**
What exists before the first request arrives. Config loaded into memory, DB schemas, Redis key structure defined, sharding topology set. Why this matters: interviewers notice when you can't explain where configuration lives. Saying "it reads from config" is not enough — say what format, where it's cached, when it's refreshed.

**Request Lifecycle (full trace)**
Walk through a single request from client to response, step by step. Name every component touched in order. At each step say: what decision is made, what data is read or written, and what happens on success vs failure. Why this matters: this is the most revealing section of any design. If you can trace a request completely you understand the system. If you can't, you have gaps. Rate limiter example: client → gateway → IP check in Redis → user check → per-API check → backend → response with headers.

**Data Model and Storage**
For each piece of state: what is it, where does it live (Redis, DB, memory), what is the key format, what is the TTL, why that storage choice. Key format matters because it determines sharding, collision risk, and observability. TTL matters because it controls memory usage and staleness.

**Why this storage choice fits**
Don't just say "use Redis". Say: Redis because (1) sub-millisecond reads are needed on the hot path, (2) atomic increment operations (INCR) prevent race conditions without a distributed lock, (3) TTL support means no manual cleanup of expired state. This reasoning is what separates a junior answer from a senior one.

**Sharding / Partitioning strategy**
What is the shard key, why that key, and what happens with hot keys. Why this matters: a system that shards by a popular endpoint (e.g. `/search`) will create a hot node. Sharding by userId spreads load evenly. Explain the reasoning explicitly — don't just name the key.

**Algorithm / Strategy choice**
Which algorithm or design strategy was chosen and why it fits this specific system's requirements. Compare it to at least one alternative and explain why the alternative was rejected. "Token bucket instead of fixed window because this API serves mobile clients that batch requests on reconnect — short burst tolerance matters more than strict per-second fairness."

**Failure Handling**
What happens when each dependency fails. Redis down, DB down, one region down, dependency slow. For each: fail-open (allow traffic, accept risk) or fail-closed (reject, accept unavailability). The choice depends on the endpoint's business criticality — payment endpoints fail closed, public content endpoints fail open. Say which and why.

**Concurrency and Race Conditions**
Where can two simultaneous requests cause incorrect behavior. What atomic operation or lock prevents it. Why a non-atomic read-increment-write would break the invariant. This section shows you understand distributed systems at a low level.

**Security and Abuse**
What are the abuse vectors specific to this system. How does the design close them. For rate limiting: IP spoofing, header manipulation, distributed bots. For caching: cache poisoning, key enumeration. Name the vector, name the mitigation.

**Observability**
What metrics you would track, what each metric tells you, what alert condition you would set. Why this matters: an unobservable system is an undebuggable system. Knowing what to measure shows you've operated systems before.

**Bottlenecks at 10x scale**
Where does the design break if load increases by 10x. What is the first thing that saturates. What is the mitigation. This is a standard senior follow-up and you should pre-answer it in the document so you're never surprised.

---

### `algos/{n}_{name}.md` — What it is and why one file per algorithm

Each algorithm gets its own file. Purpose: force a fair side-by-side comparison without one algorithm's explanation contaminating another. When you study token bucket you focus only on token bucket — its mechanics, its tradeoffs, its failure modes. Then when you read leaky bucket you can compare cleanly.

Sections to include and why each matters:

**Core mechanic**
Explain the algorithm in concrete terms: what data is stored, what happens on each request, what the check and update operations are. Use a real example with numbers. Abstract explanations don't stick. "Tokens refill at 10/sec, capacity 100, user had 45 tokens, 1 second passed, now has 55 tokens, request costs 1 token, allow" is infinitely more useful than "tokens are generated over time."

**What it stores per key**
The exact fields stored in Redis or memory per user/IP/entity. This matters because storage size determines scalability, and knowing the fields means you can discuss sharding, TTL, and atomic operations concretely.

**Pros**
Not generic. Specific to this algorithm. "Allows burst" is not a pro for fixed window — it's a bug. It is a pro for token bucket. Make sure each pro is an actual property of the algorithm, not a general wish.

**Cons**
Same rule. Specific, honest, traceable to the algorithm's mechanics.

**Why it fits — when to use**
The conditions under which this algorithm is the right choice. Be specific about what properties of the use case match what properties of the algorithm. "Token bucket fits public APIs with mobile clients because mobile clients reconnect and burst — token bucket's burst tolerance handles this gracefully while still enforcing a long-term rate."

**Why it doesn't fit — when NOT to use**
The conditions that make this algorithm the wrong choice. "Token bucket is wrong for payment processing because burst tolerance is a liability — a burst of 100 payments in 2 seconds at 3am is almost certainly fraud, and you want to reject it, not accommodate it."

**Comparison note**
One sentence saying which algorithm you would use instead in the cases where this one doesn't fit, and why. This closes the loop and shows you know the full landscape.

---

### `rust/` — What it is and why you implement it

Code exists to prove understanding. If you can't implement it, you don't fully understand it. The Rust implementation is not about language syntax — it's about being forced to think through the state, the operations, the concurrency, and the edge cases at the code level.

**`Cargo.toml`**
Standard Rust binary crate. Name it `{topic}_demo`. Use only what is needed — no unnecessary dependencies.

**`src/main.rs`**
A demo that exercises all the implemented modules with realistic scenarios. Not a unit test file — a readable demonstration that shows each algorithm or strategy working. Include cases that hit the limit, cases that pass, and edge cases (e.g. exact boundary, burst, refill after gap). Why: you should be able to run this and see the behavior, which reinforces understanding.

**`src/{module}.rs`**
One file per algorithm or strategy variant. Each module implements a struct with a `allow_request` or equivalent method. The struct holds the state that would live in Redis in a real system. Implementing this cements exactly what state is needed and why. Comments in code should explain the design decision, not the syntax — "using atomic fetch-and-increment to match what Redis INCR does" not "this increments the counter."

---

### `qa.md` — What it is and why you write it last

This file contains the interview questions and your answers for this topic. You write it last because by then you've done notes, design, and code — you know the topic well enough to anticipate what an interviewer would ask.

Sections to include and why each matters:

**Core concept questions**
The questions that check whether you understand the basics. Write the question, then write your answer in full sentences as if you are in an interview. Don't write bullet points — write the answer you would actually say out loud. Bullet points compress your thinking; full sentences expose it.

**Design follow-ups**
The deeper questions that come after the basic design is on the whiteboard. "What if Redis goes down?", "How do you handle multi-region?", "What if one user generates 100x traffic of any other user?" These are the questions that separate a prepared candidate from someone who just knows the definition.

**Tradeoff questions**
"Why did you choose X over Y?", "What would you change at 10x scale?", "What's the weakest part of your design?" Write honest answers. Interviewers know the weaknesses — they want to see if you know them too.

**Gotcha questions**
The questions designed to catch people who memorized without understanding. "Token bucket allows bursts — is that always a good thing?", "You said use Redis for state, but what if Redis is the bottleneck?", "You said fail-open for public content — what if an attacker knows you fail open?" Write what the trap is and how to escape it.

---

## Topic List (context for folder creation)

When creating a folder, use this list to understand where the topic fits and what category of depth is expected.

**RELIABILITY** — topics about preventing and handling failure
Circuit Breaker, Retry + Backoff, DLQ, Saga Pattern, Transactional Outbox, Idempotency, Timeout, Health Checks, Sidecar/Ambassador, Strangler Pattern

**SCALABILITY** — topics about handling more load
Load Balancing, Caching, CDN, Message Queue vs Stream, Scatter Gather, Reverse Proxy, Backend for Frontend, Auto Scaling, Orchestration vs Choreography, Map Reduce, Pipes and Filters

**THEORY** — foundational concepts that underpin design decisions
CAP Theorem, Consistent Hashing, CQRS, Event Sourcing, Materialized View, API Gateway, REST vs gRPC, Polling vs WebHooks, Bloom Filter, Merkle Tree, Gossip Protocol, 0 to Million Users

**DATABASE** — topics about storing and querying data reliably and at scale
SQL vs NoSQL, Replication, Sharding, Indexing, Transactions, Partitioning, Read Replicas, ACID vs BASE, Query Optimization, OLTP vs OLAP

**DEPLOY / OPS** — topics about shipping and operating software
Blue-Green Deploy, Canary Deploy, Rolling Deploy, Chaos Testing, Monitoring, Observability, Logging, Alerting, Feature Flags

**PRACTICE PROBLEMS** — end-to-end system design exercises
Rate Limiter, LRU/LFU Cache, URL Shortener, Key-Value Store, Message Queue, Notification System, Web Crawler, Job Scheduler, HTTP Server
