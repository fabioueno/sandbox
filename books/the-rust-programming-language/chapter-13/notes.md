# Notes

## Closures: Anonymous Functions That Capture Their Environment

General ideas:
- Closures are anonymous functions. They don't require type annotations.
- Depending on the function bode, closures decide how to capture values. Either
  by borrowing immutably, borrowing mutably, or taking ownership.

- There are several ways of writing closures:
```rust
fn  add_one_v1 = (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

> _Closures will automatically implement one, two or all three of these `Fn`
> traits, in an addition fashion, depending on how the closure's body handles
> the values:_
> - _`FnOnce` applies to closures that can be called once. All closures
>   implement at least this trait because all closures can be called. A closure
>   that moves captured values out of its body will only implement `FnOnce` and
>   none of the other `Fn` traits because it can only be called one._
> - _`FnMut` applies to closures that don't move captured values out of their
>   body, but that might mutate the captured values. These closures can be
>   called more than once._
> - _`Fn` applies to closures that don't move captured values out of their body
>   and that don't mutate captured values, as well as closures that capture
>   nothing from their environment. These closures can be called more than once
>   without mutating their environment, which is important in cases such as
>   calling a closure multiple times concurrently._

## Processing a Series of Items with Iterators

- Iterators are lazy, i.e. need to be consumed.
- Consuming adapters are methods that call `next`, and consume the iterator.
- Iterator adapters are methods that produce a different iterator. They are
  lazy.

## Improving Our I/O Project

## Comparing Performance: Loops vs. Iterators