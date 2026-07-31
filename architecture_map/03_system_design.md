# 03 · System Design — boundaries, API design, scaling basics

> cells. one service and its edge.
> prev ← [02](02_execution.md) · next → [04 communication](04_communication.md)

## map

```
system design
├── requirements
│   ├── functional ── what it does (place order, track order)
│   └── non-functional (NFR) ── scalability | availability | reliability | latency | throughput
│                               consistency | durability | security | cost | maintainability | observability
├── estimation ── DAU → QPS (peak ~2-3x avg) | storage/day | bandwidth | read:write ratio
├── boundaries (what's inside a service vs outside)
│   ├── business boundary ── bounded context (→ 10 DDD)
│   ├── data boundary ── who owns this table (db per service)
│   ├── contract ── API / schema = boundary made concrete
│   ├── trust boundary ── where authn/authz happens (edge / gateway) → 08
│   └── deploy boundary ── one deployable = one unit of scale & failure
├── api design
│   ├── style ── resource (REST) | action (RPC) | query (GraphQL) | event (async) → 04
│   ├── contract-first (OpenAPI / .proto / AsyncAPI / GraphQL SDL) vs code-first (annotations → spec)
│   ├── resource naming ── nouns, plural, nested max 1-2 levels (/orders/42/items)
│   ├── verbs + status codes
│   ├── versioning ── url /v1 | header | query ?v=1 | media type
│   ├── pagination ── offset/limit | cursor / keyset | page token
│   ├── filter / sort / sparse fields (?fields=)
│   ├── idempotency ── natural (GET/PUT/DELETE) | Idempotency-Key header (POST)
│   ├── errors ── consistent shape, problem+json (RFC 9457)
│   ├── long-running ops ── 202 Accepted + status url (poll) | callback/webhook
│   ├── backward compat ── add fields ok, remove/rename = breaking (Hyrum's law → 11)
│   └── protect ── authn/authz → 08 | rate limit → 06 | input validation
├── scaling
│   ├── vertical (bigger box) | horizontal (more boxes)
│   ├── stateless (any instance serves any request) vs stateful (sticky / partitioned)
│   ├── load balancer ── L4 (tcp) vs L7 (http) + algorithms
│   └── data ── replication | partitioning | sharding → 06
├── distributed truths ── CAP, PACELC, consistency models → 06, 11
└── promises ── SLA (contract) ⊃ SLO (target) ⊃ SLI (measured) · error budget · nines
```

## http verbs

| verb | use | safe | idempotent |
|---|---|---|---|
| GET | read | ✓ | ✓ |
| POST | create / action | ✗ | ✗ (use Idempotency-Key) |
| PUT | replace whole | ✗ | ✓ |
| PATCH | partial update | ✗ | ✗ (can be) |
| DELETE | remove | ✗ | ✓ |
| HEAD / OPTIONS | metadata / CORS preflight | ✓ | ✓ |

## status code families

| range | means | common |
|---|---|---|
| 1xx | info | 101 switching protocols (websocket upgrade) |
| 2xx | ok | 200, 201 created, 202 accepted (async), 204 no content |
| 3xx | go elsewhere | 301 permanent, 302/307 temp, 304 not modified (cache) |
| 4xx | your fault | 400, 401 not authn, 403 not authz, 404, 409 conflict, 422, 429 rate limited |
| 5xx | my fault | 500, 502 bad gateway, 503 unavailable, 504 gateway timeout |

## pagination (siblings)

| type | how | good | bad |
|---|---|---|---|
| offset | `?offset=40&limit=20` | jump to page N | slow deep pages, skips/dupes on insert |
| cursor / keyset | `?after=<last_id>` | fast, stable | no jump to page N |
| page token | opaque token from server | hides impl | same as cursor |

## versioning (siblings)

| type | example | one-liner |
|---|---|---|
| url path | `/v1/orders` | most common, visible |
| header | `Api-Version: 2` | clean urls |
| query | `?api-version=2` | azure style |
| media type | `Accept: application/vnd.x.v2+json` | purist REST |

## load balancing algorithms

| algo | one-liner |
|---|---|
| round robin | next in line |
| weighted round robin | bigger boxes get more |
| least connections | fewest open conns |
| least response time | fastest recently |
| ip hash / sticky | same client → same server |
| consistent hash | key → server, minimal reshuffle on change |
| power of two choices | pick 2 random, take less loaded |

## nines

| availability | downtime / year | / month |
|---|---|---|
| 99% | 3.65 d | 7.3 h |
| 99.9% | 8.76 h | 43.8 m |
| 99.99% | 52.6 m | 4.4 m |
| 99.999% | 5.26 m | 26 s |

- serial deps multiply: two 99.9% services in chain ≈ 99.8%.
