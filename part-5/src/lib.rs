use std::time::Duration;

use cfg_if::cfg_if;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data(pub u64);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputationResult(pub u64);

fn calculate(datum: Data) -> ComputationResult {
    // Make calculations faster in test :p
    cfg_if! { if #[cfg(test)] { fn x() -> u64 { 100 } } else { fn x() -> u64 { 500 } } };
    let compute_time = x();
    // Simulate heavy workload
    std::thread::sleep(Duration::from_millis(compute_time));
    ComputationResult(datum.0 * 2)
}

pub fn serial_calculate(data: Vec<Data>) -> Vec<ComputationResult> {
    // Perform calculate sequentially/serially
    data.into_iter().map(calculate).collect()
}

pub fn parallel_calculate(data: Vec<Data>) -> Vec<ComputationResult> {
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
