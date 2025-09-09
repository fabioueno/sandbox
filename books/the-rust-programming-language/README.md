# The Rust Programming Language

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

These are my personal notes from reading **The Rust Programming Language (2nd
Edition)**. I approached the book as an experienced software engineer but new
to Rust without a C/C++ background, so my notes focus on what’s _different_,
_essential_, or _conceptually deep_, like ownership/borrowing/lifetimes,
references vs. pointers, error handling without exceptions,
traits/monomorphization, and concurrency the Rust way.

## Table of Contents

- [Chapter 01: Getting Started](#chapter-01-getting-started)
- [Chapter 02: Programming a Guessing Game](#chapter-02-programming-a-guessing-game)
- [Chapter 03: Common Programming Concepts](#chapter-03-common-programming-concepts)
- [Chapter 04: Understanding Ownership](#chapter-04-understanding-ownership)
- [Chapter 05: Using Structs to Structure Related Data](#chapter-05-using-structs-to-structure-related-data)
- [Chapter 06: Enums and Pattern Matching](#chapter-06-enums-and-pattern-matching)
- [Chapter 07: Managing Growing Projects with Packages, Crates, and Modules](#chapter-07-managing-growing-projects-with-packages-crates-and-modules)
- [Chapter 08: Common Collections](#chapter-08-common-collections)
- [Chapter 09: Error Handling](#chapter-09-error-handling)
- [Chapter 10: Generic Types, Traits, and Lifetimes](#chapter-10-generic-types-traits-and-lifetimes)
- [Chapter 11: Writing Automated Tests](#chapter-11-writing-automated-tests)
- [Chapter 12: An I/O Project: Building a Command Line Program](#chapter-12-an-io-project-building-a-command-line-program)
- [Chapter 13: Functional Language Features: Iterators and Closures](#chapter-13-functional-language-features-iterators-and-closures)
- [Chapter 14: More About Cargo and Crates.io](#chapter-14-more-about-cargo-and-cratesio)
- [Chapter 15: Smart Pointers](#chapter-15-smart-pointers)
- [Chapter 16: Concurrency](#chapter-16-concurrency)
- [Chapter 17: Object-Oriented Programming Features](#chapter-17-object-oriented-programming-features)
- [Chapter 18: Patterns and Matching](#chapter-18-patterns-and-matching)
- [Chapter 19: Advanced Features](#chapter-19-advanced-features)
- [Chapter 20: Final Project: Building a Multithreaded Web Server](#chapter-20-final-project-building-a-multithreaded-web-server)
- [Next Steps](#next-steps)

---

## Chapter 01: Getting Started

[Rust](https://rust-lang.org) is an ahead-of-time compiled language. Once
compiled, the resulting binary can run anywhere without needing Rust installed.
[Cargo](https://doc.rust-lang.org/cargo), the package manager and build tool,
quickly becomes central to all workflows.

- Cargo handles dependencies, builds, and project metadata in `Cargo.toml`.
- `cargo check` is a fast way to validate syntax and types.
- `cargo run` compiles and executes the project.
- Building in release mode (`cargo build --release`) enables optimizations but
  slows compilation.

---

## Chapter 02: Programming a Guessing Game

This chapter was a great intro to Rust’s safety guarantees in practice. The
first contact with input parsing showed me Rust’s philosophy: errors are
values. **No hidden exceptions**.

- Variables are immutable by default; mutability is explicit (`let mut`).
- Pattern matching (`match`) is expressive and exhaustive. Rust forces you to
  handle all cases.
- The `Result` and `Option` enums are everywhere: they make error handling
  explicit and safe.
- The compiler prevents unused `Result` values unless handled or explicitly
  ignored.
- Type inference is strong but not magical: sometimes explicit types clarify
  intent.

```rust
// Example

// Here, `parse` returns a `Result` and `expect` unwraps it or panics with a
// custom message.
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

---

## Chapter 03: Common Programming Concepts

Nothing surprising for an experienced developer, but I liked Rust’s balance
between safety and simplicity.

- Shadowing allows redefining a variable name with a new type or value, keeping
  immutability.
- Rust enforces that every variable has a single, static type.
- Expressions (no semicolon) return values, statements (with semicolon) don’t.
- Control flow structures (`if`, `loop`, `while`, `for`) are expression-based,
  which means they can return values.

---

## Chapter 04: Understanding Ownership

This is _the_ chapter that makes Rust Rust.

I didn’t come from pointers, so I built the following mental model:

- Stack: **fixed-size**, fast push/pop. Moves are cheap (bitwise copy). Types
  that fit entirely on the stack often implement `Copy` (e.g., `i32`, `bool`),
  so **assignments duplicate the value**.
- Heap: **dynamically sized** data lives here (e.g., `String`, `Vec`). The
  variable on the stack holds a small **pointer + length + capacity** triplet;
  the **payload** lives on the heap.

### Ownership

- Every value has a **single owner**.
- When ownership moves, the previous binding becomes invalid (to prevent double
  free).
- Stack-only types (like integers) implement the `Copy` trait, so **ownership
  moves don’t apply**.

```rust
// Example

let a = String::from("hi");

// This moves ownership from `a` to `b`. Now `a` is invalid:
let b = a;

// Because `a` is invalid, this causes a compile error:
// println!("{a}");

// `clone` creates a deep copy of the value (heap allocation, potentially
// expensive):
let c = b.clone();
```

### Borrowing

- References (`&T`) allow **data access without taking ownership**.
- We can have multiple immutable references or one mutable reference, but never both.
- Rust enforces these rules at compile time, eliminating data races.

```rust
// Example

let mut s = String::from("abc");

// We can create multiple immutable references to the same data:
let r1 = &s;
let r2 = &s;

// Can't create a mutable reference while another reference exists:
// let m = &mut s;

println!("{r1} {r2}");

// After the immutable references go out of scope, we can create a new mutable
// reference:
let m = &mut s;
m.push("d");
```

### Lifetimes

- Lifetimes describe **how long references are valid**.
- Some are inferred, but explicit lifetimes are needed when the compiler can’t
  infer which reference should live longer.

```rust
// Example

// Because both inputs and the output have the same lifetime annotation `'a`,
// we can read this as: "the return value lives as long as the longest of both
// inputs".
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

- Add lifetime parameters on function signatures that return a reference
  derived from inputs.
- For methods taking `&self`, the compiler’s elision rules usually infer what I
  mean.

---

## Chapter 05: Using Structs to Structure Related Data

**Structs are named tuples with intent.** They make ownership of complex data
explicit.

- `#[derive(Debug)]` allows easy printing with `{:?}`.
- Associated functions (`impl MyStruct`) can define constructors or utilities.
- `self` in methods behaves like `this`, but ownership rules apply: `self`,
  `&self`, and `&mut self` all mean something different. The first takes
  ownership, the second takes an immutable reference, and the third takes a
  mutable reference.

```rust
// Example

#[derive(Debug)]
struct Rect {
    w: u32,
    h: u32
}

impl Rect {
    // Area is an associated function that takes an immutable reference to
    // self:
    fn area(&self) -> u32 { self.w * self.h }
}
```

---

## Chapter 06: Enums and Pattern Matching

Enums are powerful: they can hold data and drive pattern matching elegantly.

- `Option<T>` replaces nulls: `Some(value)` or `None`.
- Pattern matching is exhaustive and can destructure values.
- `if let` is shorthand for single-pattern matches.

---

## Chapter 07: Managing Growing Projects with Packages, Crates, and Modules

Rust’s module system reminds me of namespacing in other languages but with
stricter privacy rules.

- **Everything is private** by default; `pub` exposes it.
- The `use` keyword brings paths into scope.
- `pub use` re-exports symbols, shaping the public API.

---

## Chapter 08: Common Collections

The standard collections (`Vec`, `String`, `HashMap`) all enforce ownership
rules.

- `Vec<T>` is a growable list. `get()` returns `Option<&T>`, making
  out-of-bounds access safe.
- `String` is UTF-8 encoded and can’t be indexed directly because characters
  may vary in byte length.
- `HashMap<K, V>` uses ownership semantics, inserting a key or value moves it.

```rust
// Example

let mut map = std::collections::HashMap::new();
let key = String::from("k");

// This moves ownership of `key` into the map:
map.insert(key, 42);
```

---

## Chapter 09: Error Handling

Rust’s philosophy: _handle it or make it explicit._

- Use `Result<T, E>` for recoverable errors and `panic!` for unrecoverable
  ones.
    - Usually, library code returns `Result<T, E>`, while binary code decides
      when to `panic!`.
- The `?` operator is a concise way to propagate errors upward.
- Functions using `?` must return a compatible type (usually `Result` or
  `Option`).

---

## Chapter 10: Generic Types, Traits, and Lifetimes

- Generics are zero-cost due to **monomorphization**: the compiler emits
  concrete versions per type use.
- Traits are Rust’s way of expressing shared behavior. They can act as
  constraints on generic types.
- Lifetimes ensure references in generics stay valid.

```rust
// Example

use std::fmt::Display;

// Both functions below do the same thing: accept any type that implements
// `Display`.
fn show_impl(item: impl Display) { println!("{item}"); }
fn show_generic<T: Display>(item: T) { println!("{item}"); }
```

- Start with `impl Trait` in argument position for readability, and switch to
  explicit generics with bounds when they're needed in multiple places.

---

## Chapter 11: Writing Automated Tests

- Functions annotated with `#[test]` are executed via `cargo test`.
- Tests can be filtered or ignored (`#[ignore]`).
- The basic assertions are `assert!`, `assert_eq!`, and `assert_ne!`.
- Panics can be tested using `#[should_panic]`.

---

## Chapter 12: An I/O Project: Building a Command Line Program

- `std::env::args()` provides an iterator of command-line arguments.
- `std::fs::read_to_string()` reads file contents as a `Result<String>`.
- `eprintln!` outputs to stderr.
- Split logic into a library (`lib.rs`) for testability, keeping `main.rs`
  minimal.

---

## Chapter 13: Functional Language Features: Iterators and Closures

Closures and iterators add a functional flavor to Rust.

- Closures automatically capture their environment by reference or move,
  depending on usage.
- Iterators are lazy i.e., they don’t execute until consumed.
- Iterator adapters (`map`, `filter`, etc.) chain transformations efficiently.
- Iterator consumers realize the chain of operations.
- Performance matches manual loops due to zero-cost abstraction.

---

## Chapter 14: More About Cargo and Crates.io

- Cargo supports multiple profiles: `dev` and `release`.
    - We can tune settings like `opt-level` and `lto`, per profile.
- Documentation comments (`///`) generate viewable docs: `cargo doc --open`.
- Workspaces share a single `Cargo.lock`.
- `cargo install` installs binaries globally.
- `cargo xyz` runs binaries named `cargo-xyz`.

---

## Chapter 15: Smart Pointers

A reference (`&T`, `&mut T`) is a non-owning, temporary view: the compiler
enforces borrowing rules. A smart pointer is an **owning type with extra
behavior** (drop logic, ref counts, interior mutability, etc.).

### Box, Deref, and Drop

- `Box<T>` allocates data on the heap instead of the stack.
- `Deref` lets a smart pointer act like `&T`. This is why `&String` _coerces_
  to `&str` in function calls.
- `Drop` runs when a value goes out of scope. Use `std::mem::drop(x)` to drop
  early.

```rust
// Example

enum List { Cons(i32, Box<List>), Nil }
```

### Reference Counting

- `Rc<T>` enables shared ownership in single-threaded contexts.

```rust
// Example

use std::rc::Rc;

let a = Rc::new(String::from("data"));

// This increases ref count. Both `a` and `b` own the data:
let b = a.clone();
```

### RefCell

- `RefCell<T>` allows interior mutability (checked at runtime) i.e., it lets
  me borrow mutably at runtime even when the binding is immutable. Panics on
  borrow rule violations at runtime.

```rust
// Example

use std::cell::RefCell;

let x = RefCell::new(0);

*x.borrow_mut() = 1;
```

---

## Chapter 16: Concurrency

Rust makes concurrency _safe by design_.

- `thread::spawn` creates threads; `join` waits for them.
- Channels (`mpsc::channel`) enable message passing between threads.
- `Arc<T>` enables shared ownership across threads. It's the thread-safe
  equivalent of `Rc<T>`.
- `Mutex<T>` provides shared-state synchronization.
- `Arc<Mutex<T>>` combines thread-safe reference counting and mutability.
- `Send` and `Sync` traits define whether types can safely be sent or shared
  across threads.

---

## Chapter 17: Object-Oriented Programming Features

Rust doesn’t have traditional inheritance but achieves polymorphism via
**trait objects**.

- Trait objects (`Box<dyn Trait>`) use dynamic dispatch, trading a bit of
  performance for flexibility.

```rust
// Example

trait Draw {
  fn draw(&self);
}

struct Button;

impl Draw for Button {
    fn draw(&self) {}
}

// This is a generic function that accepts any type that implements `Draw`:
fn render(widgets: Vec<Box<dyn Draw>>) {
    for w in widgets { w.draw(); }
}
```

---

## Chapter 18: Patterns and Matching

- Patterns can appear in `let`, `if let`, `while let`, `match`, and function
  parameters.
- The compiler enforces exhaustive matches.
- `_` ignores values; `@` binds values while testing them.
- Pattern destructuring makes matching expressive and compact.

---

## Chapter 19: Advanced Features

Rust’s advanced type system and macros allow metaprogramming without
sacrificing safety.

- `unsafe` blocks allow manual control but only when necessary. It unlocks
  dereferencing raw pointers, calling foreign function interfaces (FFI), and
  more.
- Associated types read better than long generic tuples.
- The newtype pattern wraps existing types for trait customization.
- Function pointers (`fn`) and closures can both be passed around.
- Macros (`macro_rules!`) and procedural macros generate code at compile time.

---

## Chapter 20: Final Project: Building a Multithreaded Web Server

The [web server project](./ws) reinforces everything. It starts simple, with a 
single-threaded TCP listener. Then, adds threads for parallel requests.
Finally, it introduces a thread pool for controlled concurrency and graceful
shutdown.

---

## Next Steps

- **The Cargo Book**: https://doc.rust-lang.org/cargo
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example
- **The Rustonomicon** (for unsafe code insights): https://doc.rust-lang.org/nomicon
- **Rust for Rustaceans**: https://nostarch.com/rust-rustaceans
- **Rust Design Patterns**: https://rust-unofficial.github.io/patterns
- **Rustlings** (interactive exercises): https://rustlings.rust-lang.org