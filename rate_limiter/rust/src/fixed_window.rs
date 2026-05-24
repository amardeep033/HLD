pub fn run_demo() {
    println!("=== Fixed Window ===");

    let mut limiter = FixedWindowLimiter::new(3, 10);
    let requests = [0, 1, 2, 3, 9, 10, 11, 12];

    for timestamp in requests {
        let allowed = limiter.allow(timestamp);
        println!(
            "t={timestamp:>2}s -> {:<7} count_in_window={}",
            verdict(allowed),
            limiter.current_count()
        );
    }

    println!();
}

struct FixedWindowLimiter {
    limit: usize,
    window_size_secs: u64,
    window_start_secs: u64,
    count: usize,
}

impl FixedWindowLimiter {
    fn new(limit: usize, window_size_secs: u64) -> Self {
        Self {
            limit,
            window_size_secs,
            window_start_secs: 0,
            count: 0,
        }
    }

    fn allow(&mut self, now_secs: u64) -> bool {
        if now_secs >= self.window_start_secs + self.window_size_secs {
            self.window_start_secs = (now_secs / self.window_size_secs) * self.window_size_secs;
            self.count = 0;
        }

        if self.count < self.limit {
            self.count += 1;
            true
        } else {
            false
        }
    }

    fn current_count(&self) -> usize {
        self.count
    }
}

fn verdict(allowed: bool) -> &'static str {
    if allowed {
        "allow"
    } else {
        "block"
    }
}