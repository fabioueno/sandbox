# Notes

## How to Write Tests

Testing macros and annotations:

- We define tests by annotating functions with the `#[test]`.
- The `assert!`macro ensures a boolean condition is true.
- The `assert_eq!` macro tests for equality, while `assert_ne!` tests for
  inequality.
- It's possible to add a custom error message to `assert!`, `assert_eq!`, and
  `assert_ne!`.

Testing panics:

- A test can expect to panic. To do that, we add `#[should_panic]` after the
  test annotation and before the test function definition.
- We can add an optional `expected` to `should_panic` to specify the failure
  message.

## Controlling How Tests Are Run

Testing flags:

- We can control the parallelism of tests with `--test-threads`.
- `--show-output` shows the output of successful tests (only shown in failures
  by default).

Filtering tests:

- It's possible to run only tests whose name contain some string with
  `cargo test <part>`.
- We can add `#[ignore]` to tests to ignore them.
- We can run only ignored tests with `--ignored`.
- We can run all tests, including ignored with `--include-ignored`.

## Test Organization

General ideas:

- Unit tests live in the `src` folder, in a module named `tests` annotated with
  `cfg(test)`.
- Integration tests stay in the `tests` folder and don't need the config
  annotation.