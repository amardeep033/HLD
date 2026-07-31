# 09 · Full Stack — client ↔ edge ↔ server ↔ db, and code inside one service

> the body. one request's whole journey + how one service's code is shaped.
> prev ← [08](08_security_identity.md) · next → [10 architecture styles](10_architecture_styles.md)

## map

```
full stack
├── request path
│   browser → DNS → CDN → WAF → load balancer / reverse proxy (nginx) → API gateway → BFF → services → cache / db
├── edge components (siblings)
│   ├── forward proxy ── sits for CLIENTS (corporate proxy, VPN)
│   ├── reverse proxy ── sits for SERVERS (tls termination, routing, cache, compression)
│   ├── load balancer ── spread traffic (L4 tcp / L7 http)
│   ├── API gateway ── one front door for all clients: auth, rate limit, routing, api keys
│   ├── BFF ── one backend PER client type, shapes/aggregates data for that UI
│   ├── GraphQL gateway / federation ── one graph over many services
│   ├── service mesh sidecar ── proxy per service for east-west traffic (mTLS, retries)
│   └── CDN / WAF ── edge cache / edge firewall
├── urls vs api
│   ├── page url  /orders/42      → html (view) for humans & browser
│   └── api url   /api/orders/42  → json (data) for code (spa, mobile, other services)
├── server runtime stack
│   ├── web server ── nginx | apache | caddy | IIS  (static files, tls, proxy)
│   ├── app server ── tomcat | jetty | kestrel | gunicorn/uvicorn | node  (runs your code)
│   └── framework ── spring boot | asp.net core | express/nest | django/fastapi | rails | axum
├── presentation patterns (MV*)
│   ├── MVC | MVP | MVVM | MVI | Flux / Redux | MVU (Elm)
│   └── server MVC controller (returns view) vs API controller (returns json)
├── code architecture inside one service
│   ├── layered / n-tier ── controller → service → repository → db (deps point down to db)
│   ├── hexagonal (ports & adapters) ── core + ports, adapters plug in (web, db, mq)
│   ├── onion ── rings, domain center
│   ├── clean ── entities → use cases → adapters → frameworks, deps point INWARD
│   ├── vertical slice ── folder per feature, each owns its whole stack (→ 10)
│   └── package by layer vs package by feature
├── domain logic patterns (Fowler) ── transaction script | table module | domain model | service layer
├── data access ── active record | data mapper (ORM) | repository | unit of work | DAO | query builder | raw SQL
├── schema strategy ── db-first (scaffold code from db) | code/model-first (generate db from classes) | migrations-first (flyway, liquibase, EF migrations)
├── rpc vs interface
│   ├── interface call ── same process, ns, fails whole or not at all
│   ├── rpc ── looks like interface call, crosses network: latency, partial failure, serialization, versioning
│   └── trap ── "location transparency" hides the fallacies of distributed computing (→ 11)
├── hosting / deploy targets
│   └── bare metal | VM (IaaS) | container (docker → k8s) | PaaS (app service, heroku) | serverless / FaaS (lambda, functions)
│       | static hosting (S3+CloudFront, azure static web apps, vercel, netlify) | edge functions (cloudflare workers)
└── rendering location ── server (MVC / SSR) vs client (SPA) → 07
```

## edge layer (siblings)

| | serves | reshapes response? | one-liner |
|---|---|---|---|
| forward proxy | clients | ✗ | hides/controls clients going out |
| reverse proxy | servers | ✗ | hides servers, tls, cache |
| load balancer | servers | ✗ | spreads load |
| API gateway | all clients | rarely | shared cross-cutting (auth, rate limit, routing) |
| BFF | one client type | ✓ | tailor-made api per UI (mobile vs web vs partner) |
| GraphQL | any client | ✓ (client decides) | client picks shape per request |
| sidecar / mesh | services (east-west) | ✗ | network concerns out of app code |

> gateway in front of several BFFs = common. BFF is not a gateway.

## MV* (siblings)

| | middle piece | view ↔ middle | where |
|---|---|---|---|
| MVC | controller | controller picks view, view reads model | spring mvc, rails, asp.net mvc |
| MVP | presenter | view passive, presenter updates it via interface | winforms, old android |
| MVVM | view model | two-way data binding | WPF, angular, knockout, SwiftUI-ish |
| MVI / Flux / Redux | store + reducer | one-way: action → state → view | react + redux, android MVI |
| MVU | update fn | Elm architecture | elm, flutter bloc-ish |

## code architecture (siblings)

| | center | dependency direction | one-liner |
|---|---|---|---|
| layered | database | top → down to db | simple, domain leaks db concerns |
| hexagonal | domain | adapters → ports → core | swap web/db/mq without touching core |
| onion | domain model | outer → inner rings | hexagonal with more rings |
| clean | entities / use cases | frameworks → adapters → use cases → entities | uncle bob, same family as hexagonal/onion |
| vertical slice | feature | inside slice, anything | minimize cross-feature coupling |

## data access (siblings)

| pattern | one-liner | example |
|---|---|---|
| active record | object saves itself (`user.save()`) | rails AR, django ORM |
| data mapper | separate mapper moves object ↔ row | hibernate/JPA, EF core |
| repository | collection-like interface over aggregates | spring data, DDD |
| unit of work | track changes, commit once | EF DbContext, hibernate session |
| DAO | table-oriented data access object | classic java |
| query builder / raw | you write sql-ish | jOOQ, knex, sqlx, dapper |

## schema strategy (siblings)

| | truth lives in | one-liner |
|---|---|---|
| db-first | database | DBA designs, code generated/scaffolded from it |
| code / model-first | classes | ORM generates tables from entities |
| migrations-first | versioned sql scripts | each change is a script, both code & db follow |

## hosting (siblings)

| | you manage | scale | one-liner |
|---|---|---|---|
| bare metal | everything | manual | own hardware |
| VM (IaaS) | os + up | manual / VMSS | rent a computer |
| container / k8s | image + manifests | pods autoscale | ship the box, orchestrator runs it |
| PaaS | code + config | platform | push code, it runs |
| serverless / FaaS | function | per request, to zero | pay per invocation, cold starts |
| static + CDN | files | infinite | spa / ssg html from edge |
| edge functions | function at edge | global | tiny code near user |
