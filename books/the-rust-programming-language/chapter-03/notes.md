# Notes

## Variables and Mutability

General ideas:
- Constants are always immutable. The type must be annotated.
- Rust's naming convention for constants is `SCREAMING_SNAKE_CASE`.
- When a variable shadows another, it takes any uses of the variable to itself
  until either it itself is shadowed or the end of the scope.
- Because shadowing effectively creates a new variable (in contrast to `mut`),
  the type can be changed. But we're not allowed to change the type of a
  mutable variable.

## Data Types

General ideas:
- Arrays are fixed-size. Vectors can grow/shrink.
- If we try to access an element of an array that's out of bounds, we get a
  panic—Rust immediately stops the program, preventing invalid memory access.

## Functions

General ideas:
- The order of the functions doesn't matter.
- In function signatures, the type of each parameter must be declared
- The return type of functions must be declared.
- If no explicit `return` is given, the function returns the last expression
  evaluated.

> - _Statements are instructions that perform some actions and do not return a
> value._
> - _Expressions evaluate to a resultant value._

> _Expressions do not include ending semicolons. If you add a semicolon to the
> end of an expression, you turn it into a statement, and it will then not
> return a value._

```rust
let y = {
    let x = 3; // Statement
    x + 1      // Expression
};
```

## Control Flow

General ideas:
- Rust doesn't automatically convert non-boolean values to booleans.
- Rust variables must have a single type.