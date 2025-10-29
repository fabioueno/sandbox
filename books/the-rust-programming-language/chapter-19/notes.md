# Notes

## Unsafe Rust

General ideas:

- Rust has a feature called `unsafe`.
- `unsafe` doesn't disable other safety checks.
- Static variables have the `'static` lifetime. Accessing a static variable is
  safe, but mutating it is not.

Language syntax:

- Using `extern`, we can define foreign function interfaces (FFI).

## Advanced Traits

## Advanced Types

## Advanced Functions and Closures

## Macros