# Notes

## Processing a Guess

> _By default, Rust has a set of items defined in the standard library that it
> brings into the scope of every program. This set is called the prelude, and
> you can see everything in it at https://doc.rust-lang.org/std/prelude/index.html._

Language syntax:
- The keyword `use` is used to "import" types into scope.
- The keyword `let` created a new **immutable** variable. To make it mutable
  use `mut`.
- Comments can be initiated with `//`.
- Using `&` in an argument indicates that the argument is a **reference**.
  References are also immutable by default—to make them mutable we need to use
  `&mut`
- Strings can be interpolated with `{}` (placeholders).

The `std::io::stdin` function returns an instance of `std::io::Stdin`, which is
a type that handles the terminal's standard input.

`read_line` returns a `Result`, which is an enumeration with variants `Ok` and
`Err`.

> [!NOTE]
> If we don't add the `use` statement, the error looks like this:
> ```text
>    Compiling guessing_game v0.1.0 (/Users/fabioueno/Projects/sandbox/books/the-rust-programming-language/chapter-02/guessing_game)
> error[E0433]: failed to resolve: use of unresolved module or unlinked crate `io`
>   --> src/main.rs:10:5
>    |
> 10 |     io::stdin()
>    |     ^^ use of unresolved module or unlinked crate `io`
>    |
>    = help: if you wanted to use a crate named `io`, use `cargo add io` to add it to your `Cargo.toml`
> help: a builtin type with a similar name exists
>    |
> 10 -     io::stdin()
> 10 +     i8::stdin()
>    |
> help: consider importing this module
>    |
> 3  + use std::io;
>    |
> 
> For more information about this error, try `rustc --explain E0433`.
> error: could not compile `guessing_game` (bin "guessing_game") due to 1 previous error
> ```

> [!NOTE]
> Not handling the `Result` trigger a compilation warning:
> ```text
> warning: unused `Result` that must be used
>   --> src/main.rs:10:5
>    |
> 10 | /     io::stdin()
> 11 | |         .read_line(&mut guess);
>    | |______________________________^
>    |
>    = note: this `Result` may be an `Err` variant, which should be handled
>    = note: `#[warn(unused_must_use)]` on by default
> help: use `let _ = ...` to ignore the resulting value
>    |
> 10 |     let _ = io::stdin()
>    |     +++++++
> 
> warning: `guessing_game` (bin "guessing_game") generated 1 warning
>    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
>    Running `target/debug/guessing_game`

## Generating a Secret Number

Language syntax:
- `start..=end` creates a lower- and upper-bound inclusive range.

A `Trait` defines a functionality for a particular type, that can be shared
with other types.

## Comparing the Guess to the Secret Number

> _A `match` expression is made up of arms. An arm consists of a pattern to
> match against, and the code that should be run if the value given to `match`
> fits that arm's pattern.

General ideas:
- Rust is strongly typed, but also has type inference.
- It's possible to reuse a variable (**shadowing**).

The `parse` method converts strings to other types.

> [!NOTE]
> Not parsing the string creates a compilation error:
> ```text
> Compiling guessing_game v0.1.0 (/.../chapter-02/guessing_game)
> error[E0308]: mismatched types
> --> src/main.rs:27:25
> |
> 27  |         match guess.cmp(&secret_number) {
> |                     --- ^^^^^^^^^^^^^^ expected `&String`, found `&{integer}`
> |                     |
> |                     arguments to this method are incorrect
> |
> = note: expected reference `&String`
> found reference `&{integer}`
> note: method defined here
> --> /.../rustlib/src/rust/library/core/src/cmp.rs:976:8
> |
> 976 |     fn cmp(&self, other: &Self) -> Ordering;
> |        ^^^
> 
> For more information about this error, try `rustc --explain E0308`.
> error: could not compile `guessing_game` (bin "guessing_game") due to 1 previous error
> ```

## Allowing Multiple Guesses with Looping

Language syntax:
- The `loop` keyword creates an infinite loop. We can use `break` to exit it.

> [!TIP]
> The original code in the book uses the `println!` macro to prompt an input
> from the user. This breaks a line between the text and the user's input. To
> prevent that, I decided to use the `print!` macro, which doesn't insert a new
> line, but it requires flushing the standard output:
> ```rust
> use std::io::Write;
> 
> io::stdout()
>   .flush()
>   .expect("Failed to flush output");
> ```