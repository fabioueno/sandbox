# Notes

## All the Places Patterns Can Be Used

General ideas:

- Patterns can be used in:
    - `match` arms.
    - `if let` and `while let` expressions.
    - `while` and `for` loops.
    - `let` bindings.
    - `if` and `else` blocks.
    - `fn` parameters.

## Refutability: Whether a Pattern Might Fail to Match

General ideas:

- Irrefutable patterns are those that always match, while refutable might not
  match.

## Pattern Syntax

General ideas:

- Patterns can match literals.
- It's possible to match variables, but because `match` creates a new scope,
  variables can be shadowed.
- We can match multiple patterns with the OR operator (`|`) and ranges with
  `..=`.
- Patterns can be used to destructure structs, enums, and tuples.
- We can ignore values with the `_` wildcard. If a value has multiple parts,
  it's possible to ignore some of them with the `..` operator.
- Extra conditions can be added to `match` patterns with the `if` operator.
- The AT operator (`@`) can be used to bind a value to a variable while also
  matching it against a pattern.