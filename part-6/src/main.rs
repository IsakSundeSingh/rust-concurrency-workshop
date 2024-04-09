use part_5::{ComputationResult, Data};
#[cfg(test)]
use serial_test::serial;

fn main() {}

fn rayon_parallel_calculate(data: Vec<Data>) -> Vec<ComputationResult> {
    // Use the rayon prelude and use the same calculate-method as before
    use part_5::calculate;
    use rayon::prelude::*;
    todo!()
}

#[cfg(test)]
fn run_test(data_set: Vec<Data>) {
    use common::time_elapsed;

    let (parallel_part_5, parallel_part_5_elapsed) = time_elapsed("Hand-implemented", || {
        part_5::parallel_calculate(data_set.clone())
    });

    let (parallel_rayon, parallel_rayon_elapsed) = time_elapsed("Rayon-implementation", || {
        rayon_parallel_calculate(data_set)
    });

    assert_eq!(parallel_part_5, parallel_rayon);
    assert!(parallel_part_5_elapsed > parallel_rayon_elapsed);
}

#[test]
#[serial]
fn small_dataset() {
    let data = vec![Data(1), Data(2), Data(3)];
    run_test(data);
}

#[test]
#[serial]
fn large_dataset() {
    let data = (0..100).map(Data).collect();
    run_test(data);
}
