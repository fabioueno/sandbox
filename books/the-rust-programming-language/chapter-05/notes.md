# Notes

## Defining and Instantiating Structs

General ideas:
- Structs are tuples with added meaning: we must name the struct and it's
  values.
- When a parameter name (variable) is the same as the struct's field, we can
  use the **field init shorthand** syntax and avoid repeating the name.
- We can use the **struct update syntax** (`..`) to create an instance based on
  another instance.
- When using the struct update syntax, if any of the fields is stored in the
  heap, such as strings, the value is moved, and the previous instance becomes
  invalid.
- Tuple structs are basically structs, but without giving names to fields, only
  their types.

## An Example Program Using Structs

> [!NOTE]
> Attempting to print a struct raises a compilation error:
> ```bash
> error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
> --> src/main.rs:7:67
> |
> 7 |     println!("The area of the rectangle {} is {} square pixels.", r1, area(&r1));
> |                                         --                        ^^ `Rectangle` cannot be formatted with the default formatter
> |                                         |
> |                                         required by this formatting parameter
> |
> = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
> = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
> = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
> ```
> 
> Trying to use the suggested `{:?}` raises a different compilation error:
> ```bash
> error[E0277]: `Rectangle` doesn't implement `Debug`
> --> src/main.rs:7:69
> |
> 7 |     println!("The area of the rectangle {:?} is {} square pixels.", r1, area(&r1));
> |                                         ----                        ^^ `Rectangle` cannot be formatted using `{:?}` because it doesn't implement `Debug`
> |                                         |
> |                                         required by this formatting parameter
> |
> = help: the trait `Debug` is not implemented for `Rectangle`
> = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
> = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
> help: consider annotating `Rectangle` with `#[derive(Debug)]`
> |
> 11+ #[derive(Debug)]
> 12| struct Rectangle {
> |
> ```
> 
> To fix that, we need to add `#[derive(Debug)]` to the struct.

Language syntax:
- We can use the `dbg!` macro to print a struct. It takes ownership, prints to
  `stderr`, then return ownership.

## Method Syntax

General ideas:
- Methods are like functions, but defined within the context of structs.
- The first parameter of methods is always `self`, the instance of the struct.
- Associated functions are those defined in an `impl` block.
- It's possible to create associated functions without the `self` parameter. In
  this case, they're not methods, and are called using `::`.
- It's possible to have multiple `impl` blocks for the same struct.