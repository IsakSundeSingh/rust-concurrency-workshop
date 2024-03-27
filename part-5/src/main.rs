use std::time::Duration;

use cfg_if::cfg_if;

fn main() {
    let data = vec![Data(1), Data(2), Data(3)];

    let serial_results = timed("Serial calculate", || serial_calculate(data.clone()));
    let parallel_results = timed("Parallel calculate", || parallel_calculate(data));

    assert_eq!(serial_results, parallel_results);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Data(u64);
#[derive(Debug, Clone, PartialEq, Eq)]
struct ComputationResult(u64);

fn calculate(datum: Data) -> ComputationResult {
    // Make calculations faster in test :p
    cfg_if! { if #[cfg(test)] { fn x() -> u64 { 100 } } else { fn x() -> u64 { 500 } } };
    let compute_time = x();
    // Simulate heavy workload
    std::thread::sleep(Duration::from_millis(compute_time));
    ComputationResult(datum.0 * 2)
}

fn serial_calculate(data: Vec<Data>) -> Vec<ComputationResult> {
    // Perform calculate sequentially/serially
    data.into_iter().map(calculate).collect()
}

fn parallel_calculate(data: Vec<Data>) -> Vec<ComputationResult> {
    // A simple way to do this is to blindly spawn one thread per element to be computed.
    // This will probably not work in practice if there are a lot of elements.
    // It will also definitely _not_ be the fastest way to do it.
    // E.g. consider when one thread uses 100 ms to finish and another 10s,
    // that is 99 % of the execution time for the first thread wasted, when
    // it could have picked a new piece of data to compute.
    // We will look at faster methods later.

    // This also blindly assumes we can spawn one thread per core.
    // If num threads >> num cores then this will be a lot slower.
    // A smarter way could be to use `std::thread::available_parallelism()`
    // to get the number of parallel resources (cores), and spawn that number of
    // threads, chunking the data up into that amount.

    let mut handles = Vec::new();
    // Use for loop explicitly to avoid lazy iterators
    for datum in data {
        handles.push(std::thread::spawn(|| calculate(datum)))
    }

    // Now that the threads have been spawned and started computing,
    // we can lazily collect them. Though it is not the most efficient.
    // E.g. if thread 4 completes before threads 1-3, we will wait a longer
    // time for it to complete.
    let results = handles.into_iter().map(|x| x.join().unwrap());

    results.collect()
}

/// Handy timer function to calculate elapsed time for closure
/// and output it to stdout.
fn timed<F: FnOnce() -> U, U>(name: &str, f: F) -> U {
    let start = std::time::Instant::now();
    let results = f();
    let elapsed = start.elapsed();

    println!("{name} - elapsed time: {} ms", elapsed.as_millis());

    results
}

#[test]
fn parallel_is_faster() {
    // Check that machine will allow test to succeed
    let two = std::num::NonZeroUsize::new(2).unwrap();
    match std::thread::available_parallelism() {
        Ok(x) if x < two => panic!("Please run on a system with more than 1 core"),
        Err(_) => panic!("Unexpected error. Does your system even have cores?"),
        _ => (),
    }

    // This test kind of assumes threads are executed on a system with more than one core
    // Hehe if your system is a single-core machine or the OS doesn't parallelize
    // threads this may fail...
    let data = vec![Data(1), Data(2), Data(3)];

    let start_serial = std::time::Instant::now();
    let serial = serial_calculate(data.clone());
    let serial_elapsed = start_serial.elapsed();

    let start_parallel = std::time::Instant::now();
    let parallel = parallel_calculate(data);
    let parallel_elapsed = start_parallel.elapsed();

    // Only outputted by Cargo when tests fail
    println!(
        "serial {} parallel {}",
        serial_elapsed.as_millis(),
        parallel_elapsed.as_millis()
    );
    assert_eq!(serial, parallel);
    assert!(serial_elapsed > parallel_elapsed);
}
