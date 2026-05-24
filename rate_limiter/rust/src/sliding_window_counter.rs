pub fn run_demo() {
    println!("=== Sliding Window Counter ===");

    let mut limiter = SlidingWindowCounterLimiter::new(3, 10);
    let requests = [0, 1, 2, 3, 9, 10, 11, 12];

    for timestamp in requests {
        let estimate_before = limiter.estimated_count(timestamp);
        let allowed = limiter.allow(timestamp);
        let estimate_after = limiter.estimated_count(timestamp);

        println!(
            "t={timestamp:>2}s -> {:<7} est_before={estimate_before:.2} est_after={estimate_after:.2}",
            verdict(allowed)
        );
    }

    println!();
}

struct SlidingWindowCounterLimiter {
    limit: f64,
    window_size_secs: u64,
    current_window_start: u64,
    current_count: u64,
    previous_count: u64,
}

impl SlidingWindowCounterLimiter {
    fn new(limit: u64, window_size_secs: u64) -> Self {
        Self {
            limit: limit as f64,
            window_size_secs,
            current_window_start: 0,
            current_count: 0,
            previous_count: 0,
        }
    }

    fn allow(&mut self, now_secs: u64) -> bool {
        self.roll_window(now_secs);

        if self.estimated_count(now_secs) < self.limit {
            self.current_count += 1;
            true
        } else {
            false
        }
    }

    fn estimated_count(&self, now_secs: u64) -> f64 {
        let elapsed = now_secs.saturating_sub(self.current_window_start);
        let overlap_ratio =
            (self.window_size_secs.saturating_sub(elapsed)) as f64 / self.window_size_secs as f64;

        self.current_count as f64 + (self.previous_count as f64 * overlap_ratio.max(0.0))
    }

    fn roll_window(&mut self, now_secs: u64) {
        let window_start = (now_secs / self.window_size_secs) * self.window_size_secs;

        if window_start == self.current_window_start {
            return;
        }

        if window_start == self.current_window_start + self.window_size_secs {
            self.previous_count = self.current_count;
        } else {
            self.previous_count = 0;
        }

        self.current_window_start = window_start;
        self.current_count = 0;
    }
}

fn verdict(allowed: bool) -> &'static str {
    if allowed {
        "allow"
    } else {
        "block"
    }
}