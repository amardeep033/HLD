# 1 Fixed Window

Example:

100 requests per minute
Counter resets every minute.

user_123 -> {
count: 57,
window_start: 12:00:00
}

## Pros

- very simple - Only needs a single counter and window timestamp, so implementation logic is straightforward.
- fast - Request check is usually just one counter increment + comparison (counter < limit).
- cheap - Minimal memory and storage are needed because only one active counter per key/window is stored.

## Cons

Burst problem:
100 requests at 12:00:59
100 requests at 12:01:00

Effectively:
200 requests in 2 seconds

## When to use

- Simple internal services
- Low to medium traffic systems
- Non-critical APIs
- When slight traffic bursts are acceptable
- Cheap and easy first implementation

Examples:

- login attempts — Small burst spikes are usually acceptable, and implementation simplicity matters more than perfect accuracy.
- basic public APIs — Cheap and fast limiting is enough for general abuse protection in many low-cost APIs.
- admin/internal tools — Traffic is predictable and low volume, so fixed window overhead stays minimal.

## When NOT to use

- Payment systems — Accuracy and fairness are critical, and bursts can cause financial loss.
- Expensive AI/ML APIs — High-cost operations require strict control to prevent abuse.
- Strict traffic smoothing requirements — Systems needing consistent request rates cannot tolerate bursts.
- Highly burst-sensitive systems — Any sudden spike can overwhelm the system.
- Distributed systems needing precise fairness — Fixed window can lead to unfair distribution across nodes.

Reason:

Sudden burst spikes can overload downstream systems despite limits being technically enforced.
