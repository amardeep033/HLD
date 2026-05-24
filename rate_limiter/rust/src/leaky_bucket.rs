pub fn run_demo() {
    println!("=== Leaky Bucket ===");

    let mut limiter = LeakyBucketLimiter::new(3.0, 1.0);
    let requests = [0, 0, 0, 0, 1, 2, 3, 4];

    for timestamp in requests {
        let level_before = limiter.current_level(timestamp);
        let allowed = limiter.allow(timestamp);
        let level_after = limiter.current_level(timestamp);

        println!(
            "t={timestamp:>2}s -> {:<7} level_before={level_before:.2} level_after={level_after:.2}",
            verdict(allowed)
        );
    }

    println!();
}

struct LeakyBucketLimiter {
    capacity: f64,
    leak_per_sec: f64,
    water_level: f64,
    last_update_secs: u64,
}

impl LeakyBucketLimiter {
    fn new(capacity: f64, leak_per_sec: f64) -> Self {
        Self {
            capacity,
            leak_per_sec,
            water_level: 0.0,
            last_update_secs: 0,
        }
    }

    fn allow(&mut self, now_secs: u64) -> bool {
        self.leak(now_secs);

        if self.water_level + 1.0 <= self.capacity {
            self.water_level += 1.0;
            true
        } else {
            false
        }
    }

    fn current_level(&mut self, now_secs: u64) -> f64 {
        self.leak(now_secs);
        self.water_level
    }

    fn leak(&mut self, now_secs: u64) {
        let elapsed = now_secs.saturating_sub(self.last_update_secs) as f64;
        let drained = elapsed * self.leak_per_sec;

        self.water_level = (self.water_level - drained).max(0.0);
        self.last_update_secs = now_secs;
    }
}

fn verdict(allowed: bool) -> &'static str {
    if allowed {
        "allow"
    } else {
        "block"
    }
}