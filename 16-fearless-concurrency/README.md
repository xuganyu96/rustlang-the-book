# Chapter 16 fearless concurrency
There is not much code to show for the concepts of this chapter so here is a TL;DR of the topics covered.

## Threading
- Use `thread::spawn` to execute code in a separate thread. This function returns a handle that can be used to wait for the execution to complete using the `join()` method
- `JoinHandle<T>.join()` returns a `Result` enum that needs to be unwrapped 
- Use the `move` keyword to allow a child thread to own data from its parent thread

```rust
fn main() {
    let messages = vec!["msg 1", "msg 2", "msg 3", "oops my bad"];

    let proc_handle = thread::spawn(move || {
        let mut count = 0usize;
        for msg in messages {
            if msg.contains("oops") {
                println!("Message corrupted");
            } else {
                count += 1;
                println!("Message \"{msg}\" processed");
            }
            thread::sleep(Duration::from_secs(1));
        }
        return count;
    });

    let x = proc_handle.join();  // Result<usize, ...>
    let x = x.unwrap();
    println!("{x} message processed")
}
```

## Channels
- Calling `std::sync::mpsc` returns a tuple `(transmitter, receiver)`
- `transmitter.send()` will send a message and return a `Result` that errors out if the channel has been closed
- `receiver.recv()` (blocking), `receiver.try_recv()` (non-blocking) will attempt to receive a message and return `Result` that evals to `Err` if the channel is closed (with `recv`) or if no message is received (with `try_recv`)
- `receiver` implements the `Iterator` trait and will call `recv()` to output messages until the channel is closed
- `transmitter.clone()` can be called to pass multiple transmitters on the same channel into multiple threads

```rust
fn main() {
    let messages1 = vec![1, 2, 3, 4];
    let messages2 = vec![5, 6, 7, 8];
    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();

    let sender1 = thread::spawn(move|| {
        let mut counter = 0usize;
        for msg in messages1 {
            match tx1.send(msg) {
                Ok(_) => {
                    counter += 1;
                    println!("sender1: emitted {msg}");
                },
                Err(_) => {
                    println!("sender1: failed to emit message");
                }
            }
            thread::sleep(Duration::from_millis(500));
        }
        return counter;
    });

    let sender2 = thread::spawn(move|| {
        let mut counter = 0usize;
        for msg in messages2 {
            match tx2.send(msg) {
                Ok(_) => {
                    counter += 1;
                    println!("sender2: emitted {msg}");
                },
                Err(_) => {
                    println!("sender2: failed to emit message");
                }
            }
            thread::sleep(Duration::from_millis(500));
        }
        println!("Exited sender2");
        return counter;
    });

    let processor = thread::spawn(move || {
        let mut s = 0;
        for msg in rx {
            println!("Received {msg}");
            s += msg;
        }
        println!("{s}");
        return s;
    });

    let s1 = sender1.join().unwrap();
    let s2 = sender2.join().unwrap();
    let p = processor.join().unwrap();
    println!("{s1}, {s2}, {p}");

}
```

## Mutex
- Lock data behind thread-safe mutable container `std::sync::Mutex` 
- Calling `m.lock()` returns a `Result` that unwraps into a `MutexGuard<T>`, which implements `Deref` and `DerefMut`.
- Wrap mutex behind `std::sync::Arc` to pass them into multiple threads
- `Arc` and `Mutex` are the thread-safe equivalents of `Rc` and `RefCell`, but ensuring thread-safety comes with performance penalty

```rust
use std::sync::{Mutex, Arc};
use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    let counter = Arc::new(Mutex::new(0usize));

    let mut pool: Vec<JoinHandle<i32>> = vec![];

    for i in 0..10 {
        let thread_counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 200));
            let mut counter = thread_counter.lock().unwrap();
            *counter += 1;

            return 0;
        });
        pool.push(handle);
    }

    for handle in pool {
        handle.join().unwrap();
    }

    let counter = counter.lock().unwrap();
    println!("{counter}");
}
```