# 5 Leaky Bucket

Requests leave at constant rate.

Example:

Incoming requests -> Queue -> Process at fixed speed

Example store:

user_123 -> {
queue_size: 12,
last_processed: 12:00:10
}

Good when smoothness matters more.

## Pros

- smooth outgoing traffic — Requests are processed at a steady fixed rate.
- protects downstream systems — Prevents sudden spikes from overwhelming services.
- predictable throughput — Easy to estimate processing capacity.

## Cons

- queues/drops bursts — Sudden traffic spikes may wait or get rejected.
- can increase latency — Requests may sit in queue before processing.
- less flexible for bursty workloads — User experience may degrade during spikes.

Good when processing smoothness matters more.

## When to use

- Traffic shaping
- Network routers
- Payment processing systems
- Downstream protection systems
- Systems requiring steady throughput

Examples:

- database protection — Prevents DB overload from request spikes.
- message processing pipelines — Maintains stable processing speed.
- third-party API integrations — Avoids exceeding partner rate limits.

## When NOT to use

- Low-latency real-time systems
- User-facing APIs expecting instant responses
- Workloads requiring burst flexibility

Reason:
Queued requests can introduce latency and poor responsiveness during traffic spikes.
