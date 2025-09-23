# Notes

## Storing Lists of Values with Vectors

General ideas:
- Vector can only store values of the same type.
- To modify a vector (with `push`, for example), we need to make it mutable.

Language syntax:
- We can create a vector with `Vec::new()` or with the macro `vec!`.
- The `push` method adds elements to the end of the vector.
- The `pop` method removes and returns the last element of the vector.
- We can access an element with `&` and `[]` (`&v[2]`) or with `get`. The
  latter returns an `Option`.

> [!NOTE]
> As before, having a mutable and immutable reference to an element of the
> vector is forbidden:
> ```text
> error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
> --> src/main.rs:6:5
> |
> 4 |     let first = &v[0];
> |                  - immutable borrow occurs here
> 5 |
> 6 |     v.push(6);
> |     ^^^^^^^^^ mutable borrow occurs here
> 7 |
> 8 |     println!("The first element is: {first}");
> |                                      ----- immutable borrow later used here
> ```

## Storing UTF-8 Encoded Text with Strings

> _The `String` type, which is provided by Rust's standard library rather than
> encoded into the core language, is a growable, mutable, owned, UTF-8 encoded
> string type._

General ideas:
- It's not possible to access a string by index, like a vector, but we can
  create slices (`&s[0..5]`).

Language syntax:
- We can create a string with `String::new`, `String::from` or with the method
  `to_string` from types with the `Display` trait.
- The `push_str` is used to append a string slice.
- We can concatenate strings with `+`, but the first string becomes invalid.
- It's also possible to use the `format!` macro to combine strings.
- The `chars` method returns the individual characters of a string.

## Storing Keys with Associated Values in Hash Maps

Language syntax:
- `HashMap::new()` creates a new hash map.
- We can add new entries to a hash map with `insert()`.
- `get()` returns an `Option<&V>`.
- By calling `copied()` after `get()`, we get a `Option<V>`.
- `unwrap_or()` set a default value if an entry isn't found.
- `or_insert()` on an `Entry` returns a mutable reference to the value. If the
  value doesn't exist, it inserts it first.