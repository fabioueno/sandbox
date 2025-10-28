# Notes

## Accepting Command Line Arguments

General ideas:

- The function `std::env::args` returns an iterator of the CLI arguments.
- `collect` converts the iterator into a vector.

## Reading a File

General ideas:

- The function `std::fs::read_to_string` takes a file path and returns an
  `std::io::Result<String>` with the file's contents.

## Refactoring to Improve Modularity and Error Handling

General ideas:

- The `main.rs` should be responsible for executing the program, while `lib.rs`
  should contain the logic.
- It's possible to use the `clone` method on strings to avoid dealing with
  ownership.

Language syntax:

- The function `process::exit` finished the program.

## Working with Environment Variables

Language syntax:

- The function `std::env::var` returns an `std::io::Result<String>` with the
  value of the environment variable.

## Writing Error Messages to Standard Error Instead of Standard Output

Language syntax:

- The function `eprintln!` writes to standard error instead of standard output.