-> Two types of failiure: 
1. Transient: use retry (Spring Retry, Polly) -- retry load of file which failed once due to network issue
2. persistent: use circuit breaker which safeguards from cascading fail (Resilience4j, Hystrix) -- 
db fault which caused issue with 'inventory' cascading(chain reaction) fail to -> order -> cart -> recommendation system : act as proxy like service unavailable
if retry in persistent - resource wastage

1.1 Fixed Interval	Retry after a fixed delay (e.g., every 2 seconds).
1.2 Exponential Backoff	Increase delay after each failure (e.g., 1s → 2s → 4s → 8s).
1.3 Randomized Backoff (Jitter)	Add randomness to delays to avoid retry storms.

2.1 Closed state: monitor health like request fail, latency
2.2 Open state: when fail request > threshold [args: fail rate threshold]
2.3 Half open state: after timeout, limited number of test request [args: wait duration in open state, permitted number of calls in half open state]
