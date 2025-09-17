# Notes

## What Is Ownership?

> _Ownership is a set of rules that govern how a Rust program manages memory.
> All programs have to manage the way they use a computer's memory while
> running. Some languages have garbage collection that regularly looks for
> no-longer-used memory as the program runs; in other languages, the programmer
> must explicitly allocate and free the memory. Rust uses a third approach:
> memory is managed through a system of ownership with a set of rules that the
> compiler checks. If any of the rules are violated, the program won't compile.
> None of the features of ownership will slow down your program while it's
> running._

General ideas:
- A `String` has three parts: a pointer (address to the heap), a length, and a
  capacity.
- Assigning a string variable to another copies the **stack**, not the
  **heap**.
- To prevent **double free** errors, Rust considers the first variable no
  longer valid and **moves** the data to the second variable.
- Using the method `clone()` on a string creates a new copy of the data on the
  heap.
- Data with a known size (like integers) can be stored on the stack, and
  assigning it to another variable doesn't move the data.
- We can place the trait `Copy` on a type stored in the stack to indicate that
  we don't want the compiler to move the data, so it can be copied and be valid
  after the assignment.
- The traits `Copy` and `Drop` are mutually exclusive. Rust won't allow us to
  annotate a type with both traits.
- Passing a value to a function transfers ownership.
- Returning a value from a function transfers ownership.

> [!NOTE]
> Trying to access an invalid reference produces a compilation error:
> ```bash
> cargo build
>    Compiling strings v0.1.0 (/.../chapter-04/strings)
> error[E0382]: borrow of moved value: `s1`
>  --> src/main.rs:5:16
>   |
> 2 |     let s1 = String::from("Hello");
>   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
> 3 |     let s2 = s1;
>   |              -- value moved here
> 4 |
> 5 |     println!("{s1}, world!");
>   |                ^^ value borrowed here after move
>   |
>   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
> help: consider cloning the value if the performance cost is acceptable
>   |
> 3 |     let s2 = s1.clone();
>   |                ++++++++
>
> For more information about this error, try `rustc --explain E0382`.
> warning: `strings` (bin "strings") generated 1 warning
> error: could not compile `strings` (bin "strings") due to 1 previous error; 1 warning emitted
> ```

```rust
fn take_ownership(s: String) { ... }
fn give_ownership() -> String { ... }
```

## References and Borrowing

General ideas:
- We can pass a reference (`&`) instead of a pointer. This allows us to access
  the data without moving it, a.k.a. **borrowing**.
- References are immutable by default.
- We can't have more than one reference (`&mut`) at the same time, if one of
  them is mutable.
- Rust prevents dangling references with a compile-time error.

```rust
fn borrow_reference(s: &String) { ... }
fn borrow_mutable_reference(s: &mut String) { ... }
```

> [!NOTE]
> Creating a variable and returning a reference to it inside a function creates
> a dangling pointer, and Rust prevents it with a compile-time error:
> ```bash
>    Compiling ownership v0.1.0 (/.../chapter-04/ownership)
> error[E0106]: missing lifetime specifier
> --> src/main.rs:84:16
> |
> 84 | fn dangle() -> &String {
> |                ^ expected named lifetime parameter
> |
> = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
> help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
> |
> 84 | fn dangle() -> &'static String {
> |                 +++++++
> help: instead, you are more likely to want to return an owned value
> |
> 84 - fn dangle() -> &String {
> 84 + fn dangle() -> String {
> |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `ownership` (bin "ownership") due to 1 previous error
> ```

## The Slice Type

General ideas:
- A slice is a reference to a contiguous sequence of elements in a collection.
- The string slice type is `&str`.