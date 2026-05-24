pub fn run_demo() {
    println!("=== Token Bucket ===");

    let mut limiter = TokenBucketLimiter::new(3.0, 1.0);
    let requests = [0, 0, 0, 0, 1, 2, 3, 4];

    for timestamp in requests {
        let tokens_before = limiter.available_tokens(timestamp);
        let allowed = limiter.allow(timestamp);
        let tokens_after = limiter.available_tokens(timestamp);

        println!(
            "t={timestamp:>2}s -> {:<7} tokens_before={tokens_before:.2} tokens_after={tokens_after:.2}",
            verdict(allowed)
        );
    }

    println!();
}

struct TokenBucketLimiter {
    capacity: f64,
    refill_per_sec: f64,
    tokens: f64,
    last_refill_secs: u64,
}

impl TokenBucketLimiter {
    fn new(capacity: f64, refill_per_sec: f64) -> Self {
        Self {
            capacity,
            refill_per_sec,
            tokens: capacity,
            last_refill_secs: 0,
        }
    }

    fn allow(&mut self, now_secs: u64) -> bool {
        self.refill(now_secs);

        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    fn available_tokens(&mut self, now_secs: u64) -> f64 {
        self.refill(now_secs);
        self.tokens
    }

    fn refill(&mut self, now_secs: u64) {
        let elapsed = now_secs.saturating_sub(self.last_refill_secs) as f64;
        let replenished = elapsed * self.refill_per_sec;

        self.tokens = (self.tokens + replenished).min(self.capacity);
        self.last_refill_secs = now_secs;
    }
}

fn verdict(allowed: bool) -> &'static str {
    if allowed {
        "allow"
    } else {
        "block"
    }
}