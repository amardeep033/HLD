# 3 Sliding Window Counter / Rolling Buckets

Divide window into smaller buckets.

Example:

60 second window
6 buckets of 10 seconds

Store counts per bucket.

Example:

user_123 -> {
bucket_00: 5,
bucket_10: 9,
bucket_20: 11,
bucket_30: 8,
bucket_40: 6,
bucket_50: 10
}

Total requests = sum of active buckets.

## Pros

- good balance — Provides better accuracy than fixed window without full timestamp storage.
- scalable — Memory usage remains controlled even at high request volume.
- less memory than logs — Stores bucket counts instead of every request timestamp.
- smoother than fixed window — Reduces boundary burst problems significantly.
- production friendly — Commonly used in distributed rate limiters.

## Cons

- slightly approximate — Bucket grouping can introduce small inaccuracies near boundaries.
- more implementation complexity — Bucket rotation and cleanup logic are harder than fixed window.

Very strong choice for large-scale systems.

## When to use

- Large-scale APIs
- Distributed systems
- High-throughput services
- General-purpose production rate limiting
- Systems needing balance between accuracy and scalability

Examples:

- API gateways — Efficiently handles millions of requests with controlled memory usage.
- SaaS platforms — Good fairness without expensive per-request storage.
- microservices traffic protection — Smooth limiting across distributed nodes.

## When NOT to use

- Systems requiring exact precision
- Extremely latency-sensitive paths with complex aggregation logic
- Very small/simple systems where fixed window is sufficient

Reason:

Bucket approximation trades perfect accuracy for scalability and operational efficiency.
