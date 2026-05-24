use std::collections::VecDeque;

pub fn run_demo() {
    println!("=== Sliding Window Log ===");

    let mut limiter = SlidingWindowLogLimiter::new(3, 10);
    let requests = [0, 1, 2, 3, 9, 10, 11, 12];

    for timestamp in requests {
        let allowed = limiter.allow(timestamp);
        println!(
            "t={timestamp:>2}s -> {:<7} active_timestamps={:?}",
            verdict(allowed),
            limiter.snapshot()
        );
    }

    println!();
}

struct SlidingWindowLogLimiter {
    limit: usize,
    window_size_secs: u64,
    request_timestamps: VecDeque<u64>,
}

impl SlidingWindowLogLimiter {
    fn new(limit: usize, window_size_secs: u64) -> Self {
        Self {
            limit,
            window_size_secs,
            request_timestamps: VecDeque::new(),
        }
    }

    fn allow(&mut self, now_secs: u64) -> bool {
        while let Some(&front) = self.request_timestamps.front() {
            if now_secs.saturating_sub(front) >= self.window_size_secs {
                self.request_timestamps.pop_front();
            } else {
                break;
            }
        }

        if self.request_timestamps.len() < self.limit {
            self.request_timestamps.push_back(now_secs);
            true
        } else {
            false
        }
    }

    fn snapshot(&self) -> Vec<u64> {
        self.request_timestamps.iter().copied().collect()
    }
}

fn verdict(allowed: bool) -> &'static str {
    if allowed {
        "allow"
    } else {
        "block"
    }
}