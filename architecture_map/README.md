# Architecture Map — atoms → cells → human

> pure mental model. trees + tables + one-liners. no essays.
> every term has a **parent** (the question it answers) and **siblings** (other answers to the same question).

## the ladder

```
L12  tools ────────────── concrete products per concept                    → 12_tools.md
L11  org & laws ───────── conway, EA, principles, distributed theory       → 11_theories_laws.md
L10  many services ────── monolith ↔ micro, DDD, how to split              → 10_architecture_styles.md
L9   one request e2e ──── bff, gateway, mvc, clean arch, hosting           → 09_full_stack.md
L8   trust ────────────── authn/z, session/jwt, oauth/oidc, cors/csrf      → 08_security_identity.md
L7   client ───────────── spa/ssr/csr, micro-frontends, design systems     → 07_frontend.md
L6   survive & scale ──── retry, cb, rate limit, cache, sharding, obsv     → 06_cross_cutting.md
L5   async services ───── queue/pubsub/stream, saga, cqrs, outbox          → 05_event_driven.md
L4   two things talk ──── rest/grpc/graphql, ws/sse, webhook               → 04_communication.md
L3   one service ──────── boundaries, api design, scaling basics           → 03_system_design.md
L2   execution ────────── sync/async, blocking, threads, event loop        → 02_execution.md
L1   code ─────────────── pop/oop/fp, SOLID, GoF patterns                  → 01_foundations.md
L0   hardware ─────────── cpu, ram, disk, io, memory layout                → 01_foundations.md
```

## one request, all layers

```
user clicks (07) → token checked (08) → CDN/proxy/gateway/BFF (09) → REST call (04)
→ service boundary (03) → handler on a thread / event loop (02) on cpu+ram (01)
→ publishes event (05) with retry/cache/timeouts around every hop (06)
→ inside a microservice or modular monolith (10) shaped by team structure (11) running on kafka/redis/k8s (12)
```

## same idea at every scale (atoms ↔ organs)

| in-process (01) | distributed (04–10) |
|---|---|
| function call / interface | RPC (REST, gRPC) |
| observer | pub/sub |
| command object | message queue / command message |
| mediator | broker / orchestrator / MediatR |
| chain of responsibility | middleware pipeline / gateway filters |
| proxy | reverse proxy / API gateway / sidecar |
| facade | BFF / API gateway |
| adapter | anti-corruption layer |
| decorator | sidecar / middleware (add behaviour, same interface) |
| memento / undo log | snapshot / event sourcing |
| iterator | cursor pagination / kafka offset |
| thread pool + queue | worker pool + message queue |
| lock / mutex | distributed lock (redis, zookeeper) |
| try/catch + rollback | saga + compensation |
| DB transaction | 2PC / saga |
| CPU cache | redis / CDN |
| process isolation | microservice / bulkhead |

## how to add a new term

1. what question does it answer? → that's the parent node
2. what else answers that same question? → siblings, add them too
3. drop it in the tree of the right file + a row in a sibling table
4. add to A–Z index below

## A–Z index (term → file)

| term | file |
|---|---|
| 12-factor app | 11 |
| 2PC / 3PC | 05 |
| ABAC / RBAC / ReBAC | 08 |
| ACID / BASE | 06 |
| active record / data mapper | 09 |
| actor model | 02 |
| ADR / C4 / arc42 | 11 |
| aggregate / entity / value object | 10 |
| Amdahl's / Little's law | 11 |
| anti-corruption layer | 10 |
| API gateway | 09 |
| API versioning / pagination | 03 |
| async / sync / blocking | 02 |
| async over sync / 202 accepted | 05 |
| at-least-once / exactly-once | 05 |
| backpressure | 06 |
| batch vs stream | 05 |
| BFF | 09 |
| blue-green / canary | 10 |
| bounded context | 10 |
| broker vs log | 05 |
| bulkhead | 06 |
| cache patterns / eviction / stampede | 06 |
| callback / webhook | 04, 05 |
| CAP / PACELC | 06, 11 |
| CDC | 05 |
| CDN | 06, 09 |
| choreography / orchestration | 05 |
| circuit breaker | 06 |
| clean / hexagonal / onion / layered | 09 |
| command vs event vs query | 05 |
| consistency models | 06 |
| consistent hashing | 06 |
| consumer group / partition / offset | 05 |
| context map | 10 |
| Conway / inverse Conway | 11 |
| cookies | 07, 08 |
| CORS / CSRF / XSS | 08 |
| CPU / RAM / disk / io | 01 |
| CQRS | 05 |
| CSR / SSR / SSG / ISR | 07 |
| DDD | 10 |
| db-first / code-first | 09 |
| design system / Fluent UI / Figma | 07 |
| design tokens | 07 |
| distributed monolith | 10 |
| DLQ / poison message | 05 |
| ECST / event notification | 05 |
| enterprise architecture / TOGAF | 11 |
| epoll / io_uring / reactor / proactor | 02 |
| event loop | 02 |
| event sourcing | 05 |
| fallacies of distributed computing | 11 |
| forward / reverse proxy | 09 |
| GoF patterns | 01 |
| GraphQL / gRPC / REST / SOAP | 04 |
| hosting (VM, PaaS, serverless) | 09 |
| hydration / islands | 07 |
| idempotency | 03, 05, 06 |
| inbox / outbox | 05 |
| isolation levels | 06 |
| JWT / session | 08 |
| Kafka | 05, 12 |
| load balancer algorithms | 03 |
| localStorage / sessionStorage | 07 |
| long polling / SSE / WebSocket | 04 |
| mediator / MediatR | 01, 10 |
| memory layout / stack / heap | 01 |
| message queue / pub-sub / stream | 05 |
| micro-frontends / module federation | 07 |
| monolith / modular monolith / microservices / SOA | 10 |
| monorepo / polyrepo / Nx | 07 |
| MVC / MVP / MVVM / Flux | 09 |
| nginx | 09, 12 |
| nines / SLA / SLO / SLI | 03 |
| OAuth2 / OIDC / SAML | 08 |
| OpenTelemetry / logs / metrics / traces | 06, 12 |
| OOP / POP / functional | 01 |
| PKCE | 08 |
| process / thread / coroutine | 01, 02 |
| protobuf / avro / json | 04 |
| PWA / SPA / MPA | 07 |
| rate limiting algorithms | 06 |
| RabbitMQ / SQS / SNS | 12 |
| Redis | 06, 12 |
| replication / sharding / partitioning | 06 |
| retry / backoff / jitter | 06 |
| rpc vs interface | 09 |
| saga | 05 |
| service discovery / mesh / sidecar | 10, 12 |
| Socket.IO / SignalR / WebRTC | 04 |
| SOLID | 01 |
| strangler fig | 10 |
| TCC | 05 |
| Team Topologies | 11 |
| Temporal / Camunda | 05, 12 |
| timeout | 06 |
| urls vs api | 09 |
| vertical slice | 09, 10 |
| virtual threads / goroutines | 02 |
| wrapper lib | 07 |

---

old long-form notes (food delivery walkthrough, decision trees) → [_old/](_old/)
