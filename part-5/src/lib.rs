use std::time::Duration;

use cfg_if::cfg_if;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data(pub u64);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputationResult(pub u64);

pub fn calculate(datum: Data) -> ComputationResult {
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
    todo!()
}
