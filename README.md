# Concurrency and parallelism-workshop in Rust

In this workshop you'll about concurrency vs parallelism and how Rust supports them.

Through this workshop you'll implement different parts of a concurrent system.

> [!TIP]
> Refer to the Rust Programming Language-book for more info on [concurrency in Rust](https://doc.rust-lang.org/book/ch16-00-concurrency.html).

## Part 1: concurrent threads

Write a program that spawns a thread. The spawned thread should output `Hello from thread!`, the main program should output `Hello from main thread!`. Make sure to wait for the spawned thread before exiting the program.

> [!TIP]
> If you need a starting point the [std::thread](https://doc.rust-lang.org/stable/std/thread/index.html)-module is useful.

> [!TIP]
> Running programs in a Cargo workspace is as simple as running `cargo run -p project-name`!

<details>
<summary>
Solution
</summary>

Change the main function in [part-1/src/main.rs](./part-1/src/main.rs) to:

```rust
fn main() {
    let handle = std::thread::spawn(|| println!("Hello from thread!"));
    handle.join().expect("Could not join thread, it panicked!");
    println!("Hello from main thread!");
}
```

That's it!

</details>

---

## Part 2: passing messages

A simple way to communicate between threads is using message passing and channels. You might know this if you've used Go as it is their preferred way of communicating between green threads.

Modify the `across_the_border`-function in [part-2](./part-2/src/main.rs) to return a receiver that will receive the numbers `0, 1, 2, ..., 9` in that order.

> [!TIP]
> You can run tests for cargo workspaces just as easily as running them using `cargo test -p project-name`!

> [!TIP]
> The [std::sync::mpsc](https://doc.rust-lang.org/stable/std/sync/mpsc/index.html)-module is helpful here.

<details>
<summary>
Solution
</summary>

The implementation can be done quite simply:

```rust
fn across_the_border() -> std::sync::mpsc::Receiver<i32> {
    let (tx, rx) = std::sync::mpsc::channel::<i32>();
    std::thread::spawn(move || (0..10).for_each(|x| tx.send(x).expect("Couldn't send value. Receiver dropped")));
    rx
}
```

The `move`-keyword in the thread spawn is required since it needs ownership of `tx` (the sender-channel) to send values. Since we know from the `main`-function that the receiver will wait until there are no more values in the channel (when the sender is dropped), we don't have to use the thread handle in `across_the_border` and join it.

However, `across_the_border` should probably handle `send`-errors in a realistic scenario, as it would happen if the receiver was dropped before communication finished.

</details>

---

## Part 3: moore threads!

Spawn ten threads, each sending their own, single value (one of `0, 1, ..., 9`) which the receiver will verify.

As this can technically happen out-of-order depending on how threads are executed, the test does not verify the order it receives values to be the same.

<details>
<summary>
Solution
</summary>

This is a bit more tricky:

```rust
fn producers() -> Receiver<i32> {
    let (sender, receiver) = std::sync::mpsc::channel::<i32>();

    (0..10).for_each(|x| {
        // Clone the sender outside the thread scope
        let sender = sender.clone();
        std::thread::spawn(move || {
            // So that the cloned sender can be moved into the thread, giving it ownership
            sender.send(x).expect("Couldn't send message");
        });
    });

    receiver
}
```

Quite similar to the solution of the previous part, but it just needs to clone the sender so each thread gets their own _owned_ version of the sender. This needs to happen outside the closure passed to `thread::spawn` because if we tried to _move_ the original `sender` into it and then clone, we would have moved it in the previous iteration of the loop, and we can't give ownership of a value more than once.

</details>
