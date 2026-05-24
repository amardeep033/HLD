mod fixed_window;
mod leaky_bucket;
mod sliding_window_counter;
mod sliding_window_log;
mod token_bucket;

fn main() {
    println!("Rate limiter algorithm demos\n");

    fixed_window::run_demo();
    sliding_window_log::run_demo();
    sliding_window_counter::run_demo();
    token_bucket::run_demo();
    leaky_bucket::run_demo();
}