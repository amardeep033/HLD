# Monitoring, Observability, Logging, and Alerting — A Mental Model

Your service is deployed to production. Now the real question begins: **"Is it actually working?"**

Not "did it start?" — that's easy. But: is it serving requests correctly? Are users experiencing errors? Is latency creeping up? Is something silently degrading at 3 AM?

Monitoring, observability, logging, alerting, and feature flags are your answer.

---

## 1. The Problem — Why You Need This

Without visibility, your production system is a black box:

```
User → [Your Service] → ???
```

Something breaks. You find out from a customer support ticket. By then, it's been broken for two hours.

With observability:

```
User → [Your Service] → Metrics  →  Dashboards  →  Alerts
                      → Logs     →  Search       →  Debug
                      → Traces   →  Spans        →  Root cause
```

You see problems **before** users report them. You know **where** to look when something goes wrong. You can answer: "What changed at 14:32 that caused the error spike?"

---

## 2. Monitoring vs Observability — The Difference

These terms are used interchangeably. They are not the same.

**Monitoring** answers: "Is the system working?"
**Observability** answers: "Why is the system behaving the way it is?"

| | Monitoring | Observability |
|---|---|---|
| Focus | Known failure modes | Unknown failure modes |
| Approach | Define metrics, check thresholds | Ask arbitrary questions about system state |
| Signal | Dashboards, alerts on known metrics | Logs, traces, and metrics used together |
| Example question | "Is CPU above 80%?" | "Why is this specific user's checkout slow?" |

Monitoring is a subset of observability. You can have monitoring without full observability, but not the other way around.

> **Monitoring tells you something is wrong. Observability helps you understand why.**

The three pillars of observability:

```
Observability
├── Metrics  — aggregated numbers over time (CPU, request rate, error rate)
├── Logs     — discrete timestamped event records from your application
└── Traces   — end-to-end path of a single request through multiple services
```

---

## 3. Metrics — Numbers Over Time

A metric is a **numeric measurement collected at regular intervals**. It tells you how your system behaves over time.

### 3.1: The Four Golden Signals (Google SRE)

If you can measure only four things, measure these:

| Signal | What it measures | Example target |
|---|---|---|
| **Latency** | How long requests take | p99 response time < 200ms |
| **Traffic** | How much demand | Requests per second |
| **Errors** | How many requests fail | HTTP 5xx rate < 0.1% |
| **Saturation** | How full is the system | CPU < 80%, queue depth < 100 |

### 3.1.1: Latency Percentiles — What p99 Actually Means

You will see `p50`, `p95`, `p99`, `p99.9` everywhere in monitoring. They are **percentiles of your latency distribution**.

If you collect the response time of every request over a 1-minute window and sort them from fastest to slowest:

```
p50  (median)  — 50% of requests finished at or below this time
p90            — 90% of requests finished at or below this time
p95            — 95% of requests finished at or below this time
p99            — 99% of requests finished at or below this time
p99.9          — 99.9% of requests finished at or below this time
```

Example with 1,000 requests in a minute:

```
p50  =  45ms   ← the "typical" user experience
p90  = 120ms
p95  = 180ms
p99  = 450ms   ← 10 users had a bad time
p999 = 2100ms  ← 1 user had a terrible time
```

**Why not just use the average?**

The average hides outliers. If 990 requests take 50ms and 10 requests take 5,000ms, the average is ~99ms — looks fine. But those 10 users are experiencing a broken service.

> **p99 is the standard because it represents your worst real users, not a theoretical edge case.** If your service handles 1,000 req/s, p99 latency is being felt by 10 real users every second.

**The tail latency problem in microservices:**

When a single request fans out across multiple downstream services, tail latency compounds:

```
If each of 5 services has p99 = 50ms:
  The combined p99 is NOT 50ms.
  The chance of hitting the slow tail in at least one service grows with each hop.
  In practice, a 5-service chain p99 can be 200-300ms even if each service looks fast alone.
```

This is why distributed tracing matters — you need to see which service's tail is inflating the overall request time.

---

### 3.2: RED and USE Methods

**RED** — for measuring service health:
- **R**ate — requests per second
- **E**rror — error rate
- **D**uration — latency distribution

**USE** — for measuring resource health:
- **U**tilization — % of time the resource is busy
- **S**aturation — amount of work queued
- **E**rrors — error events

### 3.3: Prometheus + Grafana — The Standard Stack

Prometheus is the de-facto open-source metrics system. The model:

- Your services **expose** metrics at an HTTP endpoint (`/metrics`)
- Prometheus **scrapes** those endpoints on a regular interval
- Metrics are stored as time-series data
- Grafana **visualizes** them as dashboards

```
App (exposes /metrics)
       │
       │  scrape every 15s
       ▼
  Prometheus (stores time-series)
       │
       ├──► Grafana (dashboards)
       └──► Alertmanager (alerts)
```

PromQL query example:

```promql
# HTTP 5xx error rate over the last 5 minutes
rate(http_requests_total{status=~"5.."}[5m])
/
rate(http_requests_total[5m])
```

---

## 4. Logging — What Happened

A log is a **discrete, timestamped record of an event**. Where metrics give you aggregate numbers, logs tell you exactly what happened at a specific moment — and to whom.

### 4.1: Structured Logging

Write logs as **JSON**, not plain text strings. JSON logs are machine-parseable, filterable, and searchable at scale.

```json
// Bad — unstructured, hard to query
"User 1234 placed order 9876 for $45.99 at 2024-11-14T14:22:01Z"

// Good — structured, queryable
{
  "timestamp": "2024-11-14T14:22:01Z",
  "level": "info",
  "event": "order_placed",
  "user_id": 1234,
  "order_id": 9876,
  "amount": 45.99,
  "trace_id": "abc123def456"
}
```

The `trace_id` field is critical — it lets you correlate a log entry to the full distributed trace of that request.

### 4.2: Log Levels — Use Them Correctly

| Level | When to use |
|---|---|
| `ERROR` | Something failed that needs attention |
| `WARN` | Something unexpected happened but the system recovered |
| `INFO` | Normal operational events (request received, order placed) |
| `DEBUG` | Detailed internals useful during development — disable in prod |

> Never log sensitive data (passwords, tokens, PII). Run at `INFO` in production. `DEBUG` floods your log storage and adds cost with zero value.

### 4.3: Centralized Logging — ELK / EFK Stack

In a microservices system, logs are scattered across dozens of containers. You need a central place to search and correlate them.

```
App containers (log to stdout)
       │
       ▼
Logstash / Fluentd   ──► collect and parse logs
       │
       ▼
Elasticsearch        ──► index and store logs
       │
       ▼
Kibana               ──► search, filter, visualize
```

**ELK** = Elasticsearch + Logstash + Kibana
**EFK** = Elasticsearch + Fluentd + Kibana (Fluentd is lighter, preferred in Kubernetes)

In Kubernetes: a **DaemonSet** runs Fluentd on every node, tailing container logs from `/var/log/containers/` and forwarding them to Elasticsearch.

---

## 5. Distributed Tracing — Following a Request Across Services

In a monolith, a stack trace tells you where something broke. In microservices, a single user request may touch six services. A stack trace in Service A tells you nothing about what Service D was doing at the same time.

Distributed tracing stitches the full journey together:

```
User Request
  └── API Gateway        10ms
        └── Auth Service  5ms
        └── Order Service 120ms
              └── Inventory Service  80ms
              └── Payment Service    30ms
        └── Notification Service     8ms

Total: 173ms  ← if p99 is 500ms, something upstream is the culprit
```

Each service propagates a `trace_id` in request headers. Each **span** records: service name, operation, start time, duration, status. A tracing backend (Jaeger, Zipkin, Grafana Tempo) stores and visualizes them.

**OpenTelemetry** is the vendor-neutral standard for instrumenting traces, metrics, and logs — instrument once, send to any backend.

---

## 6. Alerting — Being Told When Things Break

Metrics and logs give you data. Alerts tell you **when to act**.

### 6.1: Alert on Symptoms, Not Causes

The most common alerting mistake: alerting on low-level resource metrics instead of user-facing symptoms.

| Bad alert (cause) | Good alert (symptom) |
|---|---|
| CPU > 80% | HTTP 5xx rate > 1% for 5 minutes |
| Memory > 90% | Request latency p99 > 500ms for 10 min |
| Disk > 85% | Checkout flow error rate elevated |

CPU being high is not necessarily a problem. Users getting errors always is.

### 6.2: Alert Fatigue — The Silent Killer of On-Call

If your alerting fires too often, engineers start ignoring it. This is **alert fatigue**, and it's how major outages happen during active monitoring.

Rules:
- Every alert must be **actionable** — there must be a clear response
- Every alert must be **urgent** — if it can wait, it's a report, not an alert
- Use `for: 5m` in Prometheus rules to avoid flapping on transient spikes

```yaml
# Prometheus alerting rule
groups:
  - name: myapp
    rules:
      - alert: HighErrorRate
        expr: |
          rate(http_requests_total{status=~"5.."}[5m])
          /
          rate(http_requests_total[5m]) > 0.01
        for: 5m                        # must be true for 5 min before firing
        labels:
          severity: critical
        annotations:
          summary: "High HTTP error rate on {{ $labels.service }}"
          description: "Error rate is {{ $value | humanizePercentage }}"
```

### 6.3: Alertmanager — Routing, Grouping, Silencing

Prometheus fires alerts to **Alertmanager**, which handles:

- **Routing** — send critical alerts to PagerDuty, warnings to Slack
- **Grouping** — batch related alerts into one notification instead of a flood
- **Silencing** — suppress alerts during planned maintenance windows

```
Prometheus alerts
       │
       ▼
 Alertmanager
       │
       ├──► PagerDuty  (severity: critical — wakes someone up)
       ├──► Slack      (severity: warning — visible but not urgent)
       └──► Email      (daily digest)
```

### 6.4: SLOs, SLAs, and Error Budgets

**SLA (Service Level Agreement)** — a contractual commitment to customers. "We guarantee 99.9% uptime."
**SLO (Service Level Objective)** — your internal target, usually stricter. "We target 99.95% uptime."
**Error budget** — the allowed amount of unreliability before you breach the SLO.

```
99.9% SLO = 0.1% error budget = ~43.8 minutes of downtime per month allowed

Budget used 40 of 43 minutes this month:
  → slow down deployments
  → focus on reliability work
  → no new risky feature rollouts until budget resets

Budget used 0 minutes:
  → the budget is permission to move fast
  → burn some on faster, riskier deployments
```

> Error budgets make the trade-off between reliability and velocity **explicit and quantitative**.

---

## 7. Feature Flags — Decouple Deployment from Release

A feature flag (also called a **feature toggle**) lets you deploy code to production but control whether users see the feature — without a new deployment.

```
Code deployed to prod
  │
  ├── feature flag OFF  →  users see old behavior
  └── feature flag ON   →  users see new feature
```

This is the mechanism behind **trunk-based development** — you ship code continuously, but incomplete features are hidden behind flags until they're ready.

### 7.1: What Feature Flags Enable

- **Dark launches** — deploy and run the new code silently in the background, compare output with the old system, without users knowing
- **Canary releases without infrastructure complexity** — route 5% of users to a new feature via a flag, not traffic weighting
- **Kill switches** — instantly disable a broken feature without rolling back the entire deployment
- **A/B testing** — serve different behavior to different user segments and measure outcomes
- **Gradual rollouts** — internal employees → beta users → 10% of all users → 100%

```
Flag: checkout_v2

  OFF  → all users        (default)
  ON   → internal employees
  ON   → 5% of paid users  (canary)
  ON   → 100% of users     (full rollout)
```

### 7.2: Feature Flag Types

| Type | What it controls | Example |
|---|---|---|
| **Release toggle** | Hide incomplete features | New checkout flow not ready yet |
| **Kill switch** | Disable broken functionality in prod | Turn off recommendations if they cause errors |
| **Experiment toggle** | A/B test different behaviors | Old vs new pricing page |
| **Permission toggle** | Enable for specific users or groups | Beta user program |
| **Ops toggle** | Adjust system behavior under stress | Disable expensive search features under high load |

### 7.3: Feature Flag Tools

| Tool | Type | Notes |
|---|---|---|
| LaunchDarkly | SaaS | Industry standard, real-time flag evaluation |
| Unleash | Self-hosted / SaaS | Open-source, flexible targeting rules |
| Flagsmith | Self-hosted / SaaS | Open-source with remote config support |
| AWS AppConfig | Cloud-native | Integrated with AWS, supports gradual deployments |
| Flipt | Self-hosted | Lightweight, Kubernetes-native |

### 7.4: Flag Debt — Clean Up After Rollout

Feature flags are technical debt if not cleaned up. Old flags whose conditions are always true add dead code branches. Teams that accumulate hundreds of stale flags end up with untestable code paths and cognitive overhead.

**Rule:** when a flag reaches 100% rollout and has been stable, delete the flag and the old code path. Treat it like paying off a loan.

> Never leave a release toggle in the codebase permanently. The only flags that live forever are kill switches and ops toggles.

---

## 8. Tools Reference

| Tool | Category | Role |
|---|---|---|
| Prometheus | Metrics | Scrape and store time-series metrics |
| Grafana | Visualization | Dashboards for metrics, logs, and traces |
| Alertmanager | Alerting | Route, group, and silence Prometheus alerts |
| Datadog | Full-stack platform | Metrics, logs, traces, and alerts in one SaaS |
| Elasticsearch | Log storage | Index and search log data at scale |
| Logstash / Fluentd | Log shipper | Collect, parse, and forward logs from containers |
| Kibana | Log UI | Search and visualize logs from Elasticsearch |
| Jaeger | Distributed tracing | Trace visualization and analysis |
| OpenTelemetry | Observability SDK | Vendor-neutral instrumentation for traces, metrics, logs |
| LaunchDarkly | Feature flags | Real-time feature flag management and targeting |
| PagerDuty | Incident management | On-call scheduling, alert routing, and escalation |
