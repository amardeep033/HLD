# 10 · Architecture Styles — monolith ↔ microservices, DDD, how to split

> organ systems. how many deployables, and where to cut.
> prev ← [09](09_full_stack.md) · next → [11 theories](11_theories_laws.md)

## map

```
architecture styles
├── by deployment unit (siblings)
│   ├── monolith ── one deployable, one db  (unstructured = big ball of mud)
│   ├── layered monolith ── one deployable, split by tech layer
│   ├── modular monolith ── one deployable, split by business module, strict module boundaries
│   ├── SOA ── services + ESB (smart pipes), shared contracts, enterprise
│   ├── microservices ── many small deployables, own db each, dumb pipes
│   ├── serverless / FaaS ── functions triggered by events
│   └── distributed monolith ── microservices that must deploy together (anti-pattern)
├── by structure / communication (siblings)
│   └── client-server | layered | pipe & filter | microkernel / plugin | event driven | space based | peer-to-peer | broker
├── how to split (decomposition)
│   ├── by technical layer (horizontal) ── ui / api / db teams → every feature crosses all ✗
│   ├── by business capability ── "payments", "catalog", "delivery"
│   ├── by subdomain / bounded context ── DDD
│   ├── vertical slice ── each feature owns ui→api→db slice
│   └── migrating ── strangler fig (wrap old, replace piece by piece) | branch by abstraction | ACL in front of legacy
├── DDD
│   ├── strategic (big picture, where to cut)
│   │   ├── domain → subdomains ── core (your edge) | supporting (needed, custom) | generic (buy it: auth, email)
│   │   ├── bounded context ── boundary inside which a model/word has ONE meaning
│   │   ├── ubiquitous language ── same words in code, docs, talk
│   │   └── context map ── how contexts relate
│   │       └── partnership | shared kernel | customer-supplier | conformist | anti-corruption layer (ACL) | open host service | published language | separate ways
│   └── tactical (inside one context)
│       └── entity | value object | aggregate (+ root) | domain event | repository | factory | domain service | application service
├── layering inside a service w/ mediator
│   ├── [API] controllers ── thin, map http → command/query
│   ├── [APP] handlers / use cases ── orchestrate, transactions
│   ├── [DOMAIN] aggregates, rules ── no framework deps
│   ├── [INFRA] db, broker, external apis
│   └── how API talks to APP (siblings) ── direct service call | mediator (MediatR, in-process) | in-process bus | distributed message bus (→ 05)
│       └── mediator pipeline behaviours ── validation, logging, tx, caching (chain of responsibility)
└── microservice support patterns
    ├── entry ── API gateway | BFF (→ 09)
    ├── find ── service discovery (client-side / server-side) | registry (consul, eureka, k8s dns)
    ├── network ── service mesh | sidecar | ambassador | adapter
    ├── data ── database per service | shared db (anti-ish) | saga / outbox / CQRS (→ 05) | API composition
    ├── config ── externalized config | config server
    ├── deploy ── rolling | blue-green | canary | feature flags | dark launch
    └── ops ── health check api | distributed tracing | log aggregation (→ 06)
```

## deployment styles (siblings)

| | deployables | db | team scale | pain |
|---|---|---|---|---|
| monolith | 1 | 1 shared | small | tangles as it grows |
| modular monolith | 1 | 1, schema per module | small–mid | discipline to keep boundaries |
| SOA | several | often shared | enterprise | ESB bottleneck, heavy governance |
| microservices | many | 1 per service | many teams | network, data consistency, ops |
| serverless | functions | managed | any | cold start, vendor lock, debugging |

> default path: monolith → modular monolith → extract microservices where scale/team pain proves it.

## decomposition (siblings)

| cut by | one-liner | good / bad |
|---|---|---|
| technical layer | ui / service / data | ✗ one feature = touch all layers & teams |
| business capability | what the business does | ✓ stable, maps to org |
| subdomain (DDD) | bounded contexts | ✓ language + model boundaries |
| vertical slice | per feature/use case | ✓ inside a service, low coupling |
| volatility | what changes together | ✓ fewer cross-service deploys |

## horizontal vs vertical slicing

```
horizontal (layers)          vertical (slices)
┌─────────────────────┐      ┌──────┬──────┬──────┐
│ controllers         │      │place │track │cancel│
├─────────────────────┤      │order │order │order │
│ services            │      │ api  │ api  │ api  │
├─────────────────────┤      │ app  │ app  │ app  │
│ repositories        │      │ db   │ db   │ db   │
└─────────────────────┘      └──────┴──────┴──────┘
```

## DDD tactical

| building block | one-liner |
|---|---|
| entity | has identity, changes over time (Order #42) |
| value object | no identity, immutable, compared by value (Money, Address) |
| aggregate | cluster of objects changed together as one unit, 1 tx = 1 aggregate |
| aggregate root | only entry point into the aggregate |
| domain event | something meaningful happened in the domain |
| repository | load/save whole aggregates |
| factory | builds complex aggregates |
| domain service | domain logic that fits no single entity |
| application service | orchestrates use case, no business rules |

## context map relations

| relation | one-liner |
|---|---|
| partnership | two teams succeed/fail together, co-evolve |
| shared kernel | small shared model both own |
| customer-supplier | downstream's needs influence upstream |
| conformist | downstream just accepts upstream's model |
| anti-corruption layer | downstream translates upstream model to protect its own |
| open host service | upstream offers a clean public protocol |
| published language | shared documented format (e.g. iso standards, event schema) |
| separate ways | no integration, duplicate if needed |

## deployment strategies (siblings)

| | one-liner |
|---|---|
| recreate | stop old, start new (downtime) |
| rolling | replace instances gradually |
| blue-green | full new env, switch traffic at once |
| canary | small % of traffic to new first |
| feature flag | deploy dark, turn on by config |
| shadow / dark launch | mirror traffic to new, discard responses |
