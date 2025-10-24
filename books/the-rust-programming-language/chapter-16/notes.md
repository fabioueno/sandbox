# Notes

## Using Threads to Run Code Simultaneously

General ideas:
- In Rust, a thread is mapped 1:1 to a OS thread.

Language syntax:
- We can create new threads with `thread::spawn` with a closure parameter.
- `thread::spawn` returns a `JoinHandler<T>`. Calling `join` on it blocks
  the currently running thread until it completes.
- `thread::sleep` pauses the thread.
- Using `move` before a closure makes it take ownership of values, instead of
  borrowing.

> _Note that when the main thread of a Rust program completes, all spawned
> threads are shut down, whether or not they have finished running._

## Using Message Passing to Transfer Data Between Threads

General ideas:
- We can use pass messages between threads through channels.
- The transmitter's `send` method take ownership of the parameter.

Language syntax:
- Channels can be created with `mpsc::channel` (multiple producer, single
  consumer). It returns a tuple, containing the transmitter (usually called
  `tx`) and the receiver (usually named `rx`).
- Transmitter can send messages with the `send` method.
- Receivers can get data with either `recv` or `try_recv`. The first blocks
  the threads execution and wait for the result, while the second returns a
  `Result<T, E>` with the message if available or an error otherwise.
- To create multiple transmitters, we can call `clone`.

## Shared-State Concurrency

## Extensible Concurrency with the Send and Sync Traits