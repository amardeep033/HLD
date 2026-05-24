# 2 Sliding Window Log

Store exact timestamps of every request.

Example:

user_123 -> [
12:00:01,
12:00:12,
12:00:45
]

Before allowing a request:

remove timestamps older than window
count remaining timestamps
if count < limit → allow request

## Pros

- highly accurate — Every request timestamp is tracked exactly, so no approximation errors.
- fair limiting — Prevents burst issues seen in fixed window algorithms.
- precise traffic control — Good when strict enforcement is important.

## Cons

- memory heavy — Need to store every request timestamp individually.
- expensive at scale — Cleanup and timestamp management become costly for millions of requests.
- higher CPU overhead — Frequent insertions/removals from logs increase processing cost

Usually not preferred at very high scale.

## When to use

- Security-sensitive APIs
- Authentication systems
- Payment systems
- Abuse prevention systems
- Low to medium scale systems requiring strict fairness

## Examples:

- OTP verification — Precise limiting helps prevent brute force attacks.
- payment APIs — Strict accuracy prevents abuse and financial risk.
- password reset endpoints — Exact request tracking improves security enforcement.

## When NOT to use

- Very high throughput systems
- Large-scale distributed systems
- Memory-constrained environments
- Systems prioritizing low operational cost

Reason:

Storing every request timestamp becomes expensive in memory and processing at scale.
