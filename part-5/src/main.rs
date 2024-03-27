use common::timed;
use part_5::{parallel_calculate, serial_calculate, Data};

fn main() {
    let data = vec![Data(1), Data(2), Data(3)];

    let serial_results = timed("Serial calculate", || serial_calculate(data.clone()));
    let parallel_results = timed("Parallel calculate", || parallel_calculate(data));

    assert_eq!(serial_results, parallel_results);
}

#[test]
fn parallel_is_faster() {
    use common::{ensure_can_run_parallel_test, time_elapsed};
    ensure_can_run_parallel_test();

    // This test kind of assumes threads are executed on a system with more than one core
    // Hehe if your system is a single-core machine or the OS doesn't parallelize
    // threads this may fail...
    let data = vec![Data(1), Data(2), Data(3)];

    let (serial, serial_elapsed) = time_elapsed("serial", || serial_calculate(data.clone()));
    let (parallel, parallel_elapsed) =
        time_elapsed("parallel", || parallel_calculate(data.clone()));

    assert_eq!(serial, parallel);
    assert!(serial_elapsed > parallel_elapsed);
}
