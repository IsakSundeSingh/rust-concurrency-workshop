fn main() {
    let handle = std::thread::spawn(|| println!("Hello from thread!"));
    handle.join().expect("Could not join thread, it panicked!");
    println!("Hello from main thread!");
}
