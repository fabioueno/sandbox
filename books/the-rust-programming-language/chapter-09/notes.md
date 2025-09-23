# Notes

## Unrecoverable Errors with panic!

General ideas:
- `panic!` is a macro that creates an unrecoverable error.
- It's possible to prevent unwinding on panic, but then the OS must clean up
  the memory that was being used by the program.
- The `RUST_BACKTRACE` environment variable controls the backtrace.

> _By default, these panics will print a failure message, unwind, clean up the
> stack, and quit._

## Recoverable Errors with Result

Non-critical errors (recoverable) return a `Result`:
```rust
enum Result<T, E> {
    Ok(T),
    Err(E)
}
```

- `T` is a generic type that represents the type of the value (success).
- `E` is a generic type of the error (failure).

Language syntax:
- The `unwrap` method is a shortcut for matching a `Result`. If successful, it
  returns the value; if unsuccessful, it calls `panic!`.
- `expect` is similar to `unwrap`, but receives an error message.
- We can return an error to propagate to the caller.
- The `?` shortcut after a `Result` returns the error to the caller without the
  need for pattern matching.

> _We're only allowed to use the `?` operator in a function that returns
> `Result`, `Option`, or another type that implements `FromResidual`._

## To panic! or Not to panic!