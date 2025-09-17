# Notes

## Defining an enum

General ideas:
- Enums provides a way to enumerate a set of possible values.
- We can put directly into each enum variant (each variant can have different
  data type, even no data).
- It's possible to define methods on enums.
- The `Option` type is a special enum: it says a variable can either have a
  value (`Some`) or nothing (`None`). It's so common that it's included in the
  prelude.
- Rust doesn't have the null feature.

## The match Control Flow Construct

> _Combining `match` and enums is useful in many situations. You'll se this
> pattern a lot in Rust code: `match` against an enum, bind a variable to the
> data inside, and then execute code based on it.

Language syntax:
- A `match`'s arms must cover all possible scenarios.
- In the last arm, we can use a variable as a catch-all.
- We can use `_` as a catch-all that ignores the actual values (without binding
  to any variable).

## Concise Control Flow with if let

Language syntax:
- `if let` combines both `if` and `let`.
- The left hand side of the equality is the pattern to match.