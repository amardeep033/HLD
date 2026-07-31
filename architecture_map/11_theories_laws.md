# 11 · Theories, Laws, Principles — why systems end up the way they do

> the human (and the society of humans). org shape + physics of software.
> prev ← [10](10_architecture_styles.md) · next → [12 tools](12_tools.md)

## map

```
theories
├── enterprise architecture (EA) ── architecture of the whole company, not one system
│   ├── frameworks ── TOGAF (process: ADM) | Zachman (taxonomy grid) | FEAF (US gov) | Gartner
│   ├── notation ── ArchiMate
│   └── EA layers ── business → data → application → technology
├── describing an architecture
│   └── C4 (context → container → component → code) | 4+1 view (logical, process, dev, physical + scenarios) | arc42 | ADR (decision records) | UML | sequence diagrams
├── socio-technical (org ↔ system)
│   ├── Conway's law ── system structure mirrors org communication structure
│   ├── inverse Conway maneuver ── design teams to get the architecture you want
│   ├── Team Topologies
│   │   ├── team types ── stream-aligned | platform | enabling | complicated-subsystem
│   │   └── interaction ── collaboration | x-as-a-service | facilitating
│   ├── cognitive load ── team can only own what fits in its head
│   ├── two-pizza team ── small enough to feed with 2 pizzas
│   └── Brooks's law ── adding people to a late project makes it later
├── software evolution laws
│   ├── Gall's law ── working complex system evolved from working simple one
│   ├── Hyrum's law ── with enough users, every observable behaviour is depended on
│   ├── Postel's law (robustness) ── be liberal in what you accept, conservative in what you send
│   ├── Lehman's laws ── software must keep changing or become less useful; complexity grows unless worked against
│   ├── Law of Demeter ── talk only to immediate friends (a.b().c().d() = smell)
│   ├── Kernighan's law ── debugging is 2x harder than writing
│   ├── Goodhart's law ── measure becomes target → stops being good measure
│   ├── Parkinson's law ── work expands to fill the time
│   ├── Hofstadter's law ── takes longer than expected, even accounting for this
│   └── Zawinski's law / second-system effect ── everything bloats
├── performance / math laws
│   ├── Amdahl's law ── speedup capped by the serial part
│   ├── Gustafson's law ── bigger problems use more cores well
│   ├── Little's law ── L = λ × W (in-flight = arrival rate × time in system)
│   ├── Universal Scalability Law ── contention + coherence make scaling go backwards
│   └── queueing theory ── latency explodes as utilization → 100%
├── distributed systems theory
│   ├── CAP ── under partition: consistency OR availability
│   ├── PACELC ── + else: latency OR consistency
│   ├── FLP impossibility ── no deterministic consensus in async system with 1 faulty node
│   ├── Two Generals ── no guaranteed agreement over unreliable link (why exactly-once is hard)
│   ├── Byzantine Generals ── consensus with lying nodes (→ blockchain)
│   ├── 8 fallacies of distributed computing
│   └── end-to-end argument ── correctness checks belong at the endpoints
├── design principles
│   ├── KISS | YAGNI | DRY (vs WET, AHA) | separation of concerns | high cohesion, low coupling
│   ├── SOLID (→ 01) | GRASP | composition over inheritance | least astonishment | fail fast
│   └── tell don't ask | principle of least privilege | convention over configuration
└── manifestos / methodologies
    └── 12-factor app | reactive manifesto | agile manifesto | DevOps / CALMS | SRE
```

## Conway family

| | one-liner |
|---|---|
| Conway's law | 4 teams building a compiler → 4-pass compiler |
| inverse Conway | want 5 independent services? make 5 independent teams first |
| Team Topologies | concrete team types + interaction modes to apply inverse Conway |
| cognitive load | if the service doesn't fit the team's head, split service or grow platform |

## 8 fallacies of distributed computing

1. network is reliable
2. latency is zero
3. bandwidth is infinite
4. network is secure
5. topology doesn't change
6. there is one administrator
7. transport cost is zero
8. network is homogeneous

## 12-factor app

| # | factor | one-liner |
|---|---|---|
| 1 | codebase | one repo per app, many deploys |
| 2 | dependencies | declare explicitly |
| 3 | config | in env, not code |
| 4 | backing services | db/queue = attached resources |
| 5 | build, release, run | separate stages |
| 6 | processes | stateless |
| 7 | port binding | app exports http itself |
| 8 | concurrency | scale by processes |
| 9 | disposability | fast start, graceful stop |
| 10 | dev/prod parity | keep envs alike |
| 11 | logs | event streams to stdout |
| 12 | admin processes | one-off tasks as processes |

## reactive manifesto

responsive ← resilient + elastic ← message driven

## EA frameworks (siblings)

| | type | one-liner |
|---|---|---|
| TOGAF | method | ADM cycle: vision → business → IS → tech → migrate → govern |
| Zachman | taxonomy | 6×6 grid: what/how/where/who/when/why × stakeholder views |
| FEAF | reference model | US federal gov |
| ArchiMate | notation | diagram language, pairs with TOGAF |
| C4 | lightweight diagrams | zoom levels for one software system |
