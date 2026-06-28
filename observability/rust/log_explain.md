# explain.md — What Happens When You Run curl.md

Run the five commands in [curl.md](../curl.md) in order. For each request, three outputs change:

1. **Response** — what the client sees
2. **log.txt** — what the JSON layer writes to file
3. **stdout** — what the pretty formatter prints to terminal

---

## 1. `curl -s localhost:8080/health`

### Response

```
{"status":"up"}
```

Status 200. No headers shown (`-s` not `-si`).

### log.txt

Nothing. The health handler has zero `tracing::info!()` calls. No log lines written.

### stdout

Nothing. No spans entered, no events emitted.

> **Why silence is correct:** Infrastructure endpoints do not produce business events. Only `POST /test` instrumented because that is where business logic lives.

---

## 2. `curl -si -X POST localhost:8080/test -d '{"value": 9}'`

Value 9 → valid (1–10), odd → **accepted → 200 OK**.

### Response

```
HTTP/1.1 200 OK
content-length: 15
content-type: application/json
x-request-id: 3469fa9a-083e-42f7-83e3-f82c52c4920d
date: Sun, 21 Jun 2026 11:53:45 GMT

{"result":"ok"}
```

`x-request-id` is the UUID generated at the top of the handler. It ties this HTTP response to every log line and every span for this request.

### log.txt

Three entries, one per `tracing::info!()` call:

```json
{"timestamp":"2026-06-21T11:53:45.123Z","level":"INFO","fields":{"message":"request received"},"target":"obsv::handler","span":{"name":"http_request","request_id":"3469fa9a-083e-42f7-83e3-f82c52c4920d","endpoint":"/test"},"spans":[{"name":"http_request","request_id":"3469fa9a-083e-42f7-83e3-f82c52c4920d","endpoint":"/test"}]}
{"timestamp":"2026-06-21T11:53:45.124Z","level":"INFO","fields":{"message":"business logic completed","value":9,"accepted":true},"target":"obsv::handler","span":{"name":"http_request","request_id":"3469fa9a-083e-42f7-83e3-f82c52c4920d","endpoint":"/test"},"spans":[{"name":"http_request","request_id":"3469fa9a-083e-42f7-83e3-f82c52c4920d","endpoint":"/test"}]}
{"timestamp":"2026-06-21T11:53:45.124Z","level":"INFO","fields":{"message":"request completed","status":200},"target":"obsv::handler","span":{"name":"http_request","request_id":"3469fa9a-083e-42f7-83e3-f82c52c4920d","endpoint":"/test"},"spans":[{"name":"http_request","request_id":"3469fa9a-083e-42f7-83e3-f82c52c4920d","endpoint":"/test"}]}
```

What to read:
- `span.request_id` matches `x-request-id` in the HTTP response — same UUID, same request
- `span.endpoint` appears on every line automatically — set once on the root span, inherited by all events
- `fields` holds the event's own key-value pairs — each log has different fields
- `validate` and `business_logic` child spans do NOT appear here — they are execution boundaries only, not log events

### stdout

Three lines, one per event, each showing which span it belongs to:

```
  2026-06-21T11:53:45.123456Z  INFO obsv::handler: request received
    in http_request{request_id="3469fa9a-083e-42f7-83e3-f82c52c4920d", endpoint="/test"}

  2026-06-21T11:53:45.124111Z  INFO obsv::handler: business logic completed
    value=9 accepted=true
    in http_request{request_id="3469fa9a-083e-42f7-83e3-f82c52c4920d", endpoint="/test"}

  2026-06-21T11:53:45.124300Z  INFO obsv::handler: request completed
    status=200
    in http_request{request_id="3469fa9a-083e-42f7-83e3-f82c52c4920d", endpoint="/test"}
```

> **Mental model:** The `in http_request{...}` line is the pretty formatter showing which span this event belongs to. You never write `request_id` in the `info!()` call — the span carries it automatically.

---

## 3. `curl -si -X POST localhost:8080/test -d '{"value": 10}'`

Value 10 → valid range, even → **rejected → 422**.

### Response

```
HTTP/1.1 422 Unprocessable Entity
content-length: 55
x-request-id: 24a2d82d-5cea-4c6a-b5d4-c18695cb2f65
content-type: application/json
date: Sun, 21 Jun 2026 11:56:02 GMT

{"result":"error","reason":"even numbers are rejected"}
```

### log.txt

Three entries. Log 2 is `WARN` because even numbers hit `tracing::warn!()`:

```json
{"timestamp":"2026-06-21T11:56:02.501Z","level":"INFO","fields":{"message":"request received"},...,"span":{"name":"http_request","request_id":"24a2d82d-5cea-4c6a-b5d4-c18695cb2f65","endpoint":"/test"}}
{"timestamp":"2026-06-21T11:56:02.502Z","level":"WARN","fields":{"message":"business logic completed","value":10,"accepted":false},...,"span":{"name":"http_request","request_id":"24a2d82d-5cea-4c6a-b5d4-c18695cb2f65","endpoint":"/test"}}
{"timestamp":"2026-06-21T11:56:02.502Z","level":"INFO","fields":{"message":"request completed","status":422},...,"span":{"name":"http_request","request_id":"24a2d82d-5cea-4c6a-b5d4-c18695cb2f65","endpoint":"/test"}}
```

What to read:
- `level: WARN` on log 2 — even numbers are a business rejection, elevated to warn
- `accepted: false` explains exactly why 422 was returned
- `request_id` is different — new UUID per request

### stdout

```
  2026-06-21T11:56:02.501234Z  INFO obsv::handler: request received
    in http_request{request_id="24a2d82d-5cea-4c6a-b5d4-c18695cb2f65", endpoint="/test"}

  2026-06-21T11:56:02.502100Z  WARN obsv::handler: business logic completed
    value=10 accepted=false
    in http_request{request_id="24a2d82d-5cea-4c6a-b5d4-c18695cb2f65", endpoint="/test"}

  2026-06-21T11:56:02.502300Z  INFO obsv::handler: request completed
    status=422
    in http_request{request_id="24a2d82d-5cea-4c6a-b5d4-c18695cb2f65", endpoint="/test"}
```

---

## 4. `curl -si -X POST localhost:8080/test -d '{"value": 11}'`

Value 11 → out of range → **rejected at validation → 400**.

### Response

```
HTTP/1.1 400 Bad Request
content-length: 60
x-request-id: 6c27ec5b-eed8-4e75-98ad-3f023f11ffc2
content-type: application/json
date: Sun, 21 Jun 2026 11:56:07 GMT

{"result":"error","reason":"value must be between 1 and 10"}
```

### log.txt

Three entries. Log 2 is the validation failure — the `business_logic` span was never reached:

```json
{"timestamp":"2026-06-21T11:56:07.601Z","level":"INFO","fields":{"message":"request received"},...,"span":{"name":"http_request","request_id":"6c27ec5b-eed8-4e75-98ad-3f023f11ffc2","endpoint":"/test"}}
{"timestamp":"2026-06-21T11:56:07.601Z","level":"WARN","fields":{"message":"validation failed","value":11},...,"span":{"name":"http_request","request_id":"6c27ec5b-eed8-4e75-98ad-3f023f11ffc2","endpoint":"/test"}}
{"timestamp":"2026-06-21T11:56:07.601Z","level":"INFO","fields":{"message":"request completed","status":400},...,"span":{"name":"http_request","request_id":"6c27ec5b-eed8-4e75-98ad-3f023f11ffc2","endpoint":"/test"}}
```

What to read:
- Log 2 says `validation failed`, not `business logic completed` — this tells you the request short-circuited in the validate span
- No `accepted` field — `business_logic` was never entered, so that field never existed
- `value: 11` on the warn — makes the rejection reason explicit without reading the code

### stdout

```
  2026-06-21T11:56:07.601000Z  INFO obsv::handler: request received
    in http_request{request_id="6c27ec5b-eed8-4e75-98ad-3f023f11ffc2", endpoint="/test"}

  2026-06-21T11:56:07.601200Z  WARN obsv::handler: validation failed
    value=11
    in http_request{request_id="6c27ec5b-eed8-4e75-98ad-3f023f11ffc2", endpoint="/test"}

  2026-06-21T11:56:07.601400Z  INFO obsv::handler: request completed
    status=400
    in http_request{request_id="6c27ec5b-eed8-4e75-98ad-3f023f11ffc2", endpoint="/test"}
```

> **Compare requests 3 and 4 in log.txt.** Same structure — 3 lines, same keys — but log 2 says `validation failed` vs `business logic completed`. The log answers *why* the status was different.

---

## 5. `curl -s localhost:8080/metrics`

### Response

```
# HELP http_request_duration_seconds Request latency in seconds
# TYPE http_request_duration_seconds histogram
http_request_duration_seconds_bucket{endpoint="/test",le="0.01"} 3
http_request_duration_seconds_bucket{endpoint="/test",le="0.05"} 3
http_request_duration_seconds_bucket{endpoint="/test",le="0.1"} 3
http_request_duration_seconds_bucket{endpoint="/test",le="0.5"} 3
http_request_duration_seconds_bucket{endpoint="/test",le="1"} 3
http_request_duration_seconds_bucket{endpoint="/test",le="+Inf"} 3
http_request_duration_seconds_sum{endpoint="/test"} 0.0008190019999999999
http_request_duration_seconds_count{endpoint="/test"} 3
# HELP http_requests_total Total HTTP requests
# TYPE http_requests_total counter
http_requests_total{endpoint="/test",status_code="200"} 1
http_requests_total{endpoint="/test",status_code="400"} 1
http_requests_total{endpoint="/test",status_code="422"} 1
```

### log.txt

Nothing. The metrics handler has no `tracing::info!()` calls.

### stdout

Nothing. No spans entered.

What to read:
- `http_requests_total` has three entries — one per status code. The three `/test` calls (200, 422, 400) each incremented a different label combination
- All 3 requests fell in the `le="0.01"` bucket — all completed in under 10ms
- `http_request_duration_seconds_sum ≈ 0.00082s` total for 3 requests — average ~0.27ms per request
- Prometheus pulls this in pull mode — the service just holds current values, Prometheus (or curl) comes to read

---

## The Full Picture

```
One POST /test request:

  curl → HTTP response
           x-request-id: 3469fa9a-...    ← ties response to logs and spans

  log.txt (3 lines per request)
    line 1  INFO  "request received"                    request_id + endpoint from span
    line 2  INFO  "business logic completed"  value=9   accepted=true
            WARN  "business logic completed"  value=10  accepted=false     (422)
            WARN  "validation failed"         value=11                     (400)
    line 3  INFO  "request completed"         status=200 / 422 / 400

  stdout (3 events, each showing span context)
    same events as log.txt, different format
    "in http_request{...}" shows which span the event belongs to

  /metrics (accumulated across all requests)
    http_requests_total{status_code="200"} 1
    http_requests_total{status_code="422"} 1
    http_requests_total{status_code="400"} 1
    http_request_duration_seconds — all 3 under 10ms
```

The three outputs answer different questions:

| Output | Question it answers | Granularity |
|---|---|---|
| log.txt | Why did this specific request succeed or fail? | Per request, per event |
| stdout | What was the execution context for each event? | Per request, per event |
| /metrics | How many requests? How fast? | Aggregated across all requests |

You cannot answer "why did request X fail?" from metrics alone — you need logs.
You cannot answer "how many 422s in the last hour?" from logs alone — you need metrics.
That is why both exist.
