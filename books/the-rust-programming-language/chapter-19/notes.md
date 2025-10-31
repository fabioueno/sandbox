# Notes

## Unsafe Rust

General ideas:

- Rust has a feature called `unsafe`.
- `unsafe` doesn't disable other safety checks.
- Static variables have the `'static` lifetime. Accessing a static variable is
  safe, but mutating it is not.
- If one of the methods of a trait is unsafe, the trait is unsafe.

Language syntax:

- Using `extern`, we can define foreign function interfaces (FFI).

## Advanced Traits

General ideas:

- A trait can have placeholder types associated with it. The type is called an
  Associated Type, and the implementator must specify the concrete type.
- When using generics, we can specify a default concrete type.
- If there are two or more methods with the same name (for example by
  implementing traits with the same method), we can specify by adding the trait
  name before the method: `Trait::method`.
- There's a Rust pattern called "Newtype Pattern", whose idea is to create a
  wrapper type around a tuple struct. This allows us to implement traits for
  that type. There's no performance penalty!

Language syntax:

- We can create a trait that depends on another trait: `Trait: Supertrait`.

## Advanced Types

General ideas:

- The Newtype Pattern can be used to abstract the implementations of a type.
- It's possible to create an alias for a type in Rust.
- Diverging functions are functions that return `!` ("empty type" or "never").
- `!` can be coersed to any type.
- Dynamically sized types (DST) are values without known size at compilation.
  We can use `?Sized` to say a type "may or may not be sized".

## Advanced Functions and Closures

General ideas:

- It's possible to pass functions as arguments to other functions. The type is
  `fn` and it's called Function Pointer.

## Macros