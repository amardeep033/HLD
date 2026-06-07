use rand::Rng;
use std::future::Future;
use std::time::Duration;
use tokio::time::{sleep, timeout};

#[derive(Debug)]
enum AppError {
    Timeout,
    NetworkError,
    ServiceUnavailable,
    ValidationError,
}

impl AppError {
    fn is_retryable(&self) -> bool {
        matches!(
            self,
            AppError::Timeout
                | AppError::NetworkError
                | AppError::ServiceUnavailable
        )
    }
}

#[derive(Debug, Clone)]
struct RetryConfig {
    max_retries: u32,
    base_delay: Duration,
    max_backoff: Duration,
    timeout_per_attempt: Duration,
    jitter_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 5,
            base_delay: Duration::from_millis(200),
            max_backoff: Duration::from_secs(5),
            timeout_per_attempt: Duration::from_secs(2),
            jitter_ms: 250,
        }
    }
}

async fn retry_with_backoff<F, Fut, T>(
    config: RetryConfig,
    mut operation: F,
) -> Result<T, AppError>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, AppError>>,
{
    let mut attempt = 0;

    loop {
        println!("Attempt {}", attempt + 1);

        let result = timeout(
            config.timeout_per_attempt,
            operation(),
        )
        .await;

        match result {
            Ok(inner_result) => match inner_result {
                Ok(value) => {
                    println!("Request succeeded");
                    return Ok(value);
                }

                Err(err) => {
                    // fail fast for non-retryable
                    if !err.is_retryable() {
                        println!(
                            "Non-retryable error encountered: {:?}",
                            err
                        );

                        return Err(err);
                    }

                    // retry limit reached
                    if attempt >= config.max_retries {
                        println!(
                            "Retry limit exhausted. Last error: {:?}",
                            err
                        );

                        return Err(err);
                    }

                    println!("Retryable error: {:?}", err);
                }
            },

            // tokio timeout
            Err(_) => {
                if attempt >= config.max_retries {
                    println!("Retry limit exhausted due to timeout");

                    return Err(AppError::Timeout);
                }

                println!("Request timed out");
            }
        }

        // exponential backoff
        let exponential_delay_ms =
            config.base_delay.as_millis() as u64
                * 2_u64.pow(attempt);

        // cap max backoff
        let capped_delay_ms = exponential_delay_ms.min(
            config.max_backoff.as_millis() as u64,
        );

        // jitter
        let jitter =
            rand::thread_rng().gen_range(0..config.jitter_ms);

        let final_delay =
            Duration::from_millis(capped_delay_ms + jitter);

        println!(
            "Sleeping for {:?} before retry",
            final_delay
        );

        sleep(final_delay).await;

        attempt += 1;
    }
}

// ----------------------------------------------------
// DEMO
// ----------------------------------------------------

async fn fake_api_call() -> Result<String, AppError> {
    use std::sync::atomic::{AtomicU32, Ordering};

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    let current =
        COUNTER.fetch_add(1, Ordering::SeqCst);

    if current < 2 {
        Err(AppError::ServiceUnavailable)
    } else {
        Ok("Success response".to_string())
    }
}

#[tokio::main]
async fn main() {
    let config = RetryConfig::default();

    let result = retry_with_backoff(
        config,
        fake_api_call,
    )
    .await;

    println!("Final Result: {:?}", result);
}