# Notes

## Removing Duplication by Extracting a Function

General ideas:
- We can create functions that operate on "placeholder" types to remove
  duplication.

## Generic Data Types

General ideas:
- Generics can be used in functions, structs, enums, and methods. The syntax is
  similar.

```rust
// Function with generics
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Struct with generics
struct Point<T> {
    x: T,
    y: T
}

// Enum with generics
enum Option<T> {
    Some(T),
    None
}

// Method with generics
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

> _Using generic types won't make your program run any slower than it would
> with concrete types. Rust accomplishes this by performing monomorphization of
> the code using generics at compile time. **Monomorphization** is the process
> of turning generic code into specific code by filling in the concrete types
> that are used when compiled._

## Traits: Defining Shared Behavior

> _A trait defines the functionality a particular type has and can share with
> other types. We can use traits to define shared behavior in an abstract way.
> We can use trait bounds to specify that a generic type can be any type that
> has a certain behavior._

General ideas:
- The scope must be brought into scope when using a struct that implements it.
- It's possible to conditionally implement methods using traits.

```rust
// This will only be implemented if the generic `T` of `Pair` has both traits,
// Display and PartialOrd.
impl<T: Display + PartialOrd> Pair<T> {
  fn compare(&self) { ... }
}
```

Language syntax:
- To implement traits, we write `impl <Trait> for <struct>`.
- Traits as parameters is used with `&impl`.
- We can also use traits in parameters with a trait bound:
  `<T: Trait>(parameter: T)`.
- It's possible to implement multiple traits with `+`: `x: &(impl T1 + T2)` and
  `<T: T1 + T2>(x: &T)`.
- We can use `where` to specify trait bounds.

```rust
fn one_function(item: &Display) { ... }

fn some_function<T: Display>(item: &T) { ... }

fn another_function(item: &(impl Display + Debug)) { ... }

fn different_function<T: Display + Debug>(item: &T) { ... }

fn other_function<T, U>(item: &T, another: &U)
where T: Display + Clone, U: Clone + Debug { ... }
```

## Validating References with Lifetimes

> _Lifetimes ensure that references are valid as long as we need them to be._

> _Every reference in Rust has a lifetime, which is the scope for which that
> reference is valid. Most of the time, lifetimes are implicit and inferred. We
> must annotate types only when multiple types are possible. In a similar way,
> we must annotate lifetimes when the lifetimes of references could be related
> in a few different ways._

General ideas:
- Lifetimes prevent dangling references.
- The Rust compiler has a **borrow checker** to validate all borrows.
- Lifetime on function/method parameters are called input lifetimes.
- A static lifetime can live through the entire duration of the program.

Language syntax:
- Lifetimes' names begin with an apostrophe followed by lowercase letters.

> Consider the following function:
> ```rust
> fn longest(x: &str, y: &str) -> &str {
>     if x.len() > y.len() {
>         x
>     } else {
>         y
>     }
> }
> ```
>
> Compiling it, gives the following error:
> ```bash
> error[E0106]: missing lifetime specifier
> --> src/main.rs:1:33
>   |
> 1 | fn longest(x: &str, y: &str) -> &str {
>   |               ----     ----     ^ expected named lifetime parameter
>   |
>   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
> help: consider introducing a named lifetime parameter
>   |
> 1 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
>   |           ++++     ++          ++          ++
> ```

**Lifetime elisions**: In some cases the compiler can infer the lifetime, and
we don't need to explicitly annotate it:
1. The compiler automatically assign a lifetime to each reference.
2. If there's only one parameter reference, its lifetime is assigned to the
   output lifetime.
3. If there a `&self` or `&mut self`, its lifetime is assigned to the output.