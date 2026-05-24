# 4 Token Bucket

Tokens refill over time.
Request consumes token.

Example:
Capacity = 100 tokens
Refill = 10 tokens/sec

user_123 -> {
tokens: 45,
last_refill: 12:00:05 -- when new request arrives, calculate tokens to add based on elapsed time since last_refill
}

## Pros

- allows controlled bursts — Users can temporarily exceed steady rate if tokens were saved.
- industry favorite — Widely used in networking, cloud systems, and API gateways.
- smooth traffic — Refill mechanism naturally spreads traffic over time.
- efficient — Only token count + refill timestamp need to be stored.
- flexible — Easy to tune burst size and refill speed independently.

## Cons

- slightly harder implementation — Requires refill calculations based on elapsed time.
- distributed coordination complexity — Shared token state becomes harder across regions/nodes.

Very commonly preferred.

## When to use

- Public APIs
- API gateways
- Cloud services
- Systems allowing short bursts
- Large-scale distributed systems

Examples:

- AWS/API gateway rate limiting — Allows temporary spikes while maintaining long-term fairness.
- user-facing APIs — Better user experience during short activity bursts.
- networking traffic shaping — Smoothly controls packet/request rates.

## When NOT to use

- Systems needing perfectly constant outgoing traffic
- Very simple applications where fixed window is enough
- Systems where burst allowance itself is risky

Reason:
Token bucket intentionally allows bursts, which may not suit strictly uniform traffic requirements.
