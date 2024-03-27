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

---

## Part 4: shared-state concurrency

### A note on shared-state-concurrency

Consider this exercpt from the [Rust-book](https://doc.rust-lang.org/book/ch16-03-shared-state.html):

> Consider this part of the slogan from the Go language documentation again: “do not communicate by sharing memory.”
>
> What would communicating by sharing memory look like? In addition, why would message-passing enthusiasts caution not to use memory sharing?

In Go and other languages, this makes a lot of sense. Shared-state concurrency, or sharing resources in a concurrent system, is host to a ton of potential errors and hazards, and when channel-communication in Go is so easily available it makes sense to try to avoid shared-state. However, Rust has a whole system to deal with not accidentally mutating other's state, through ownership, lifetimes and the borrow checker.

A `Mutex<T>` in Rust is a type which requires the user to acquire a lock to the value to access `T` within. Acquiring a lock can block the thread until the lock in question is available. Let's see a simple example usage:

```rust
// Outer scope
let m = Mutex::new(5);

{
    // Inner scope
    let mut num = m.lock().unwrap();
    *num = 6;
}

println!("m = {m:?}");
```

In the outer scope, we create a mutex `m` wrapping an integer, in this case `5`. Even though the mutex itself is not mutable, it provides interior mutability through the `lock`-method. It returns a result, which will only fail if another thread holding the lock has panicked and crashed ([lock poisoning](https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html#poisoning)). The lock method returns a guard, in this case `num`, which when dropped releases the lock. The guard-type is a smart-pointer which allows you to dereference it (`*`) as if it was a mutable reference to the number itself.

The print in the outer scope will print `6`.

However, `Mutex<T>: !Send`, so we cannot send it across threads. Why? Because as we saw in its API, a `Mutex<T>` only ensures safe access to a value within a single thread, if you were to have a clone on another thread one could introduce data races. So we need to send something that can be shared across threads with the mutex inside it, to provide both safe access within a thread, and also safe access to the mutex itself across threads.

### Problem description

Create a mutex holding a boolean value `false`. Use the provided method `assignment` and spawn one thread that mutates the value inside the mutex to `true`. After the method is completed one should be able to consistently assert that the value is `true` (i.e. ensure the thread finishes).

Unfortunately, even though I would like you to figure out how to send the value and mutex across the thread boundary yourself, writing tests for this is really difficult since Rust tries to enforce correct usage everywhere, so the problem wouldn't compile 😅 But please try to understand why the signature is as it is and make the program run without failing (and/or run the test).

> [!TIP]
> Having trouble understanding sharing a `Mutex` across threads? Check out [this chapter](https://doc.rust-lang.org/book/ch16-03-shared-state.html#multiple-ownership-with-multiple-threads) of the Rust book.

<details>
<summary>
Summary
</summary>

The assignment can be implemented as such:

```rust
fn assignment(x: Arc<Mutex<bool>>) {
    thread::spawn(move || {
        // `x.lock.unwrap()` gives us a mutex guard, which is a smart pointer
        // so we can just dereference it with `*` to update the value within.
        *x.lock().unwrap() = true;
        // The mutex guard is dropped here, which also releases the lock for other threads to use.
    })
    // Join the thread so we are certain the value is changed before being asserted
    .join()
    .expect("Couldn't join thread");
}
```

As for the explanation of `Arc<Mutex<bool>>`:

- `bool: Send + Sync`, but you can't share the state mutably, only clone/copy it. Ownership and borrows cannot be asserted across thread boundaries deterministically so the compiler doesn't let us share it mutably.
- `Mutex<bool>: Send + Sync`, but you can't share the state mutably, for the same reasons as above.
- `Rc<Mutex<bool>>: !Send + !Sync` (RC = reference count) allows multiple owners of the value to share it mutably on a single thread by runtime reference counting, deallocating the value when the count reaches 0. However, it is itself not shareable across threads. This is because the RC keeps a count on how many borrows it has at runtime, and that count is subject to data races (RAW, WAR, WAW).
- `Arc<Mutex<bool>>: Send + Sync` Arc = atomic reference counting. It is the same as the `Rc`-type, but keeps an atomic count which atomically updates the count (a CPU-level instruction prohibits data races at the cost of a higher runtime cost). This means we can cheaply clone an `Arc` and pass that to another thread. Cloning an `Arc` just increases a counter and copies a pointer to the data within. The mutex inside it provides ownership guarantees across threads because of the lock.

<br>

> ![NOTE]
> Difficult? Hopefully not too much. In _most_ cases, sharing state across threads mutably is as easy as using `Arc<Mutex<T>>`, which has an easy interface. The only thing you need to worry about are deadlocks such as when you have more than one lock to acquire, e.g. thread one acquires `Arc<Mutex<T>`
>
> If you tried figuring this part out on your own, I hope the one thing you're left with is that Rust truly backs you up! Consider how many languages would not give you _any_ warning where Rust refused to compile your code because of bugs. It truly enables _fearless concurrency_.

</details>
