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

## Generic Type Parameters, Trait Bounds, and Lifetimes Together