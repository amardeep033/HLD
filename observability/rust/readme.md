# diff.md — Key Distinctions for the Discussion

---

## 1. Trace vs Log in Rust — The Basics (No OTel)

This is the foundation. Before bringing in OpenTelemetry, understand what `tracing` alone gives you.

### 1.1 The Two Building Blocks of `tracing`

The `tracing` crate has exactly two primitives:

```
Event   →  a point in time. Something happened. This IS a log.
Span    →  a duration. A block of work. This IS a trace unit.
```

In code:

```rust
// This is an event (log). A point in time.
tracing::info!(user_id = 123, "payment received");

// This is a span. A duration with a name and fields.
let span = tracing::info_span!("process_payment", order_id = 456);
let _enter = span.enter();
```

The **event** records that something happened at a moment.
The **span** wraps a block of code and says "everything inside belongs to this operation."

### 1.2 The Critical Difference

An event answers: **what happened?**
A span answers: **what was running when it happened?**

```
[span: process_payment — order_id=456]
    ↓ event: "payment received"   user_id=123
    ↓ event: "calling bank API"
    ↓ event: "payment confirmed"  amount=299
[span ends]
```

Every event logged inside a span automatically carries that span's fields. You don't repeat `order_id` on every event — the span carries it for you.

This is why spans exist: **context inheritance**.

### 1.3 What You Get Without OTel

With only `tracing` + `tracing-subscriber`, spans show up in your pretty output like this:

```
INFO obsv::handler: request received
  in obsv::handler::http_request with request_id: abc123, endpoint: "/test"
    in obsv::handler::validate with value: 3
```

The `in` lines show the span hierarchy. You can see which span each log event belongs to.

But these are **not** OTel spans. There is no `trace_id`. There is no `span_id`. The spans are just named execution contexts — useful for grouping logs, not for distributed tracing.

```
Without OTel:
  tracing span = a named block that adds fields to child events
  tracing event = a log line

With OTel:
  tracing span = a named block + a real trace_id + a real span_id + exported to Jaeger
  tracing event = a log line AND an OTel event recorded on the span
```

> **Mental model:** `tracing` gives you hierarchy and context. OTel gives you IDs and export.

### 1.4 What `tracing-subscriber` Does

Neither `tracing` events nor spans know where they go. `tracing-subscriber` is the runtime that decides.

```
Your code calls: tracing::info!("request received")
                           ↓
           tracing-subscriber receives it
                           ↓
            ┌──────────────┬──────────────┐
            ↓              ↓              ↓
        stdout         log.txt       OTel layer
    (pretty fmt)    (JSON fmt)   (converts to OTel span)
```

You stack layers. Each layer handles one output target. The same event or span can be handled by multiple layers simultaneously.

This is why comments in the code say `tracing = API, tracing-subscriber = runtime`. One produces. The other decides what to do with it.

### 1.5 Events vs Spans — The Design Rule

Do not mix them up in purpose.

```
Logs (events)  →  business events. What happened.
               →  "request received", "payment failed", "user created"
               →  appear in log.txt

Spans          →  execution flow. Where it happened.
               →  "http_request", "validate", "business_logic"
               →  appear in trace output (otel.log or stdout)
```

A span should NOT be a log. A log should NOT be a span.

Bad:
```rust
// You're using a span to log something. Wrong tool.
let _s = tracing::info_span!("payment received", user_id = 123).entered();
```

Good:
```rust
// Span = execution boundary
let _s = tracing::info_span!("process_payment", order_id = 456).entered();

// Event = business fact
tracing::info!(user_id = 123, amount = 299, "payment received");
```

---

## 2. `tracing` vs OpenTelemetry

### 2.1 The Problem That Creates the Confusion

`tracing` can produce structured logs AND spans. OpenTelemetry also deals with spans. So why are there two things?

Because they solve **different problems** at different layers.

`tracing` is a Rust-specific instrumentation API. It is designed for developers to write nice structured logs and spans with minimal boilerplate. It has no opinion about where the data goes.

OpenTelemetry is a vendor-neutral standard for transporting telemetry. It defines the format for traces (trace_id, span_id, parent_span_id), the propagation protocol (W3C traceparent header), and the export protocol (OTLP). It works across languages — the same standard works in Rust, Go, Java, Python.

```
tracing   →  how you write instrumentation in Rust
OTel      →  how telemetry is represented and transported to backends
```

### 2.2 What Changes When You Add OTel

Without OTel, a `tracing` span is a named block that:
- Adds fields to child events
- Shows up in pretty stdout output

With OTel (via `tracing-opentelemetry`), a `tracing` span also:
- Gets a real `trace_id` (32 hex chars)
- Gets a real `span_id` (16 hex chars)
- Gets a `parent_span_id` linking to the parent
- Gets exported to Jaeger, Tempo, Datadog, or any OTel backend

```
Without OTel:
  tracing::info_span!("process_payment")
    → named context block in your log output
    → no trace_id

With OTel:
  tracing::info_span!("process_payment")
    → named context block in your log output  (unchanged)
    → ALSO an OTel span with trace_id=abc123, span_id=def456
    → exported to Jaeger
```

You write the same code. The bridge (`tracing-opentelemetry`) handles the conversion.

### 2.3 The Architecture

```
Your Rust code
     ↓
tracing::info_span!("validate")     ← you write this
     ↓
tracing-subscriber registry
     ├── stdout layer     → pretty output for development
     ├── JSON file layer  → structured log.txt
     └── OTel layer       → converts span to OTel span
                                    ↓
                          opentelemetry_sdk (TracerProvider)
                                    ↓
                             SpanExporter
                                    ↓
                          Jaeger / Tempo / Datadog
```

### 2.4 `#[tracing::instrument]` vs Manual Spans

Both are from the `tracing` crate. The macro is just shorthand.

```rust
// This:
#[tracing::instrument(skip(body), fields(request_id = tracing::field::Empty))]
pub async fn test(body: web::Json<TestRequest>) -> HttpResponse {
    let request_id = Uuid::new_v4().to_string();
    tracing::Span::current().record("request_id", request_id.as_str());
    // ...
}

// Is equivalent to:
pub async fn test(body: web::Json<TestRequest>) -> HttpResponse {
    let request_id = Uuid::new_v4().to_string();
    let span = tracing::info_span!("test", request_id = %request_id);
    let _enter = span.enter();
    // ...
}
```

Use `#[instrument]` when the function parameters are the span fields.
Use manual `info_span!` when you need to generate fields inside the function (like a UUID) before attaching them.

> **Why not always use `#[instrument]`?** Because `request_id` is generated inside the handler. It doesn't exist at the moment the span opens. So you either pre-declare it as `Empty` and `.record()` it, or you just use `info_span!` directly. For an discussion, the explicit form is clearer to explain.

### 2.5 Comparison Table

| | `tracing` | OpenTelemetry |
|---|---|---|
| What it is | Rust instrumentation API | Vendor-neutral telemetry standard |
| Who uses it | Application developers | Platform / infrastructure layer |
| What it produces | Events and spans | Spans with trace_id/span_id, metrics, logs |
| Where it lives | Inside your Rust code | Inside your Rust code + collector + backend |
| Vendor specific? | No | No — that's the whole point |
| Needs a backend? | No — can write to stdout/file | Yes — needs Jaeger, Tempo, Datadog, etc. |
| Discussion demo | Use this directly | Mention as "production path" |

> **Discussion answer:** `tracing` is the Rust API for creating structured logs and spans. OpenTelemetry is the standard for giving those spans real IDs and shipping them to a backend. `tracing-opentelemetry` bridges the two. You write `tracing`, you get OTel output.

---

## 3. Prometheus Crate vs OpenTelemetry Metrics

### 3.1 The Pain

Your service produces numbers — request counts, latency, queue depth. Somewhere a dashboard needs to show these. Something has to collect them, store them, and make them queryable. Two different approaches exist.

### 3.2 Prometheus Crate — the Direct Approach

```rust
// You define a metric
let counter = prometheus::register_counter_vec!(
    Opts::new("http_requests_total", "Total HTTP requests"),
    &["endpoint", "status_code"]
).unwrap();

// You increment it
counter.with_label_values(&["/test", "200"]).inc();

// Prometheus scrapes /metrics every 15 seconds
// GET /metrics → returns text in Prometheus format
```

```
Your Rust service
    ↓ exposes /metrics endpoint
Prometheus server scrapes it every 15 seconds
    ↓
Grafana queries Prometheus
    ↓
Dashboard
```

The service is passive. It just holds the current metric values. Prometheus comes to collect.

### 3.3 OpenTelemetry Metrics — the Standard Approach

```rust
// You define a metric through the OTel Meter API
let meter = global::meter("service");
let counter = meter.u64_counter("http_requests_total").init();

// You record a value
counter.add(1, &[
    KeyValue::new("endpoint", "/test"),
    KeyValue::new("status_code", "200"),
]);
```

```
Your Rust service
    ↓
OTel SDK (Meter API)
    ↓ pushes via OTLP
OTel Collector
    ↓
Prometheus / Datadog / whatever backend
```

The service actively pushes data. The Collector receives, transforms, routes.

### 3.4 The Real Difference

It is not about pull vs push as a concept. It is about coupling.

The Prometheus crate is **tightly coupled to Prometheus**. If you ever switch to Datadog or another backend, you rewrite your metrics code.

OTel Metrics is **decoupled**. You write to the OTel Meter API. The backend is configured externally via the Collector. Switching backends means changing collector config, not application code.

```
Prometheus crate:
  code → Prometheus format → Prometheus only

OTel Metrics:
  code → OTel format → Collector → anything
```

> **Mental model:** Prometheus crate is like writing directly to MySQL. OTel Metrics is like writing to an interface that can be backed by any database. Faster to start with the direct approach. Harder to change later.

### 3.5 Comparison Table

| Aspect | Prometheus crate | OTel Metrics |
|---|---|---|
| API | `CounterVec`, `HistogramVec`, `Gauge` | `Meter`, `Counter`, `Histogram` |
| Backend | Prometheus only | Any OTLP-compatible backend |
| Collection model | Pull — Prometheus scrapes `/metrics` | Usually push — OTLP to Collector |
| Endpoint needed | Yes — `/metrics` | No — Collector handles it |
| Complexity | Low | Medium |
| Good for | Discussion demos, simple setups | Production, multi-backend, vendor-neutral |
| Switching backends | Rewrite metrics code | Change collector config |

### 3.6 Discussion Recommendation

Use the Prometheus crate for the coding round.

Why:
- Two lines to define a metric, one line to record
- `/metrics` endpoint is visible and easy to demo with curl
- No external dependencies needed — it all runs in one process
- Easy to explain to an discussioner without setting up a Collector

In a production conversation, mention OTel Metrics as the right long-term choice.

---

## 4. One-Page Summary

```
LAYER 1 — Instrumentation (inside your service)

  tracing::info!(...)          →  event (log line)
  tracing::info_span!(...)     →  span (execution boundary + context carrier)

  Without OTel: spans are just named blocks in log output. No trace_id.
  With OTel:    spans get trace_id + span_id + exported to Jaeger.

  tracing            = the API you write
  tracing-subscriber = the runtime that routes events (stdout, file, OTel)
  tracing-opentelemetry = bridge: tracing span → OTel span


LAYER 2 — Metrics

  Prometheus crate   = fast, simple, Prometheus-only, pull model, /metrics endpoint
  OTel Metrics       = vendor-neutral, push model, needs Collector, flexible


LAYER 3 — The Three Pillars

  Logs     →  what happened in this specific request  (log.txt)
  Metrics  →  how is the system behaving overall      (/metrics)
  Traces   →  where was the request slow              (otel.log → Jaeger)


LAYER 4 — Cardinality (the production gotcha)

  Low cardinality labels   →  endpoint, method, status_code  →  OK
  High cardinality labels  →  request_id, user_id, email     →  kills Prometheus

  Never use request_id as a metric label.
  Use it in logs. Use it in spans. Never in metrics.
```

---

## 5. Discussion Answers — Say Exactly This

**"What is the difference between `tracing` and OpenTelemetry?"**

> `tracing` is the Rust API for creating structured logs and spans. It produces events and spans, but does not give them trace IDs or export them. OpenTelemetry is the vendor-neutral standard for representing and transporting telemetry — it defines what a span looks like (trace_id, span_id, parent_span_id) and how to export it (OTLP). `tracing-opentelemetry` bridges the two: you write `tracing`, you get OTel output.

**"What is the difference between a log and a trace in Rust?"**

> In the `tracing` crate, a log is an event — a point-in-time record of something that happened. A trace (span) is a duration — a block of work that carries context. Events live inside spans and automatically inherit their fields. Without OpenTelemetry, spans are just named execution contexts. With OpenTelemetry, each span also gets a trace_id and span_id and can be sent to Jaeger.

**"Why Prometheus crate instead of OTel Metrics?"**

> For a demo, Prometheus crate is simpler — define a metric in two lines, increment in one, expose at `/metrics`. For production, OTel Metrics is better because it's vendor-neutral: switching backends means changing the Collector config, not the application code.

**"What is high cardinality and why is it a problem?"**

> High cardinality means a label has many unique values — like `user_id` which could be millions of values. Prometheus stores one time series per unique label combination. A million users means a million time series for one metric, which exhausts memory. Labels should have small, bounded value sets: `endpoint`, `status_code`, `method` — not `user_id`, `email`, `request_id`.
