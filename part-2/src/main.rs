fn main() {
    let receiver = across_the_border();

    while let Ok(x) = receiver.recv() {
        println!("Got: {x}");
    }
}

fn across_the_border() -> std::sync::mpsc::Receiver<i32> {
    todo!()
}

#[test]
fn sends_data_correctly() {
    let receiver = across_the_border();
    let mut result = Vec::new();
    while let Ok(x) = receiver.recv() {
        result.push(x);
    }

    assert_eq!(result, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
}
