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

## Developing the Library's Functionality with Test-Driven Development

## Working with Environment Variables

## Writing Error Messages to Standard Error Instead of Standard Output