use std::time::Duration;

/// Panics if the machine it runs on only has one core
pub fn ensure_can_run_parallel_test() {
    // Check that machine will allow test to succeed
    let two = std::num::NonZeroUsize::new(2).unwrap();
    match std::thread::available_parallelism() {
        Ok(x) if x < two => panic!("Please run on a system with more than 1 core"),
        Err(_) => panic!("Unexpected error. Does your system even have cores?"),
        _ => (),
    }
}

/// Handy timer function to calculate elapsed time for closure
/// and output it to stdout.
pub fn timed<F: FnOnce() -> U, U>(name: &str, f: F) -> U {
    let start = std::time::Instant::now();
    let results = f();
    let elapsed = start.elapsed();

    println!("{name} - elapsed time: {} ms", elapsed.as_millis());

    results
}

/// Handy timer function to calculate elapsed time for closure
/// and output it to stdout.
pub fn time_elapsed<F: FnOnce() -> U, U>(name: &str, f: F) -> (U, Duration) {
    let start = std::time::Instant::now();
    let results = f();
    let elapsed = start.elapsed();

    println!("{name} - elapsed time: {} ms", elapsed.as_millis());

    (results, elapsed)
}
