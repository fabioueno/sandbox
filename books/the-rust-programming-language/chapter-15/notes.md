# Notes

General ideas:

- References borrow data, but smart pointers own the data.
- Smart pointers implement `Deref` and `Drop`.

## Using Box<T> to Point to Data on the Heap

> _Boxes allow you to store data on the heap rather than the stack._

Language syntax:

- To define a box, we need to specify the type (`Box<T>`).
- We create boxes with `Box::new`.

## Treating Smart Pointers Like Regular References with Deref

General ideas:

- The `Deref` trait customize the dereference operator.
- Deref coersion automatically converts a reference to a type that implements
  `Deref` into another type.

> _Deref coersion can convert `&String` ot `&str` because `String` implements
> the `Deref` trait such that it returns `&str`._

## Running Code on Cleanup with the Drop Trait

General ideas:

- `Drop` customize what happens when a value is about to be dropped (go out of
  scope).

Language syntax:

- To drop a variable, we need to call `std::mem::drop`.

> _Variables are dropped in the reverse order of their creation._

## Rc<T>, the Reference Counted Smart Pointer

General ideas:

- `Rc<T>` allows multiple ownership and keeps track of the number of
  references. It can only be used in single-threaded cases.

## RefCell<T> and the Interior Mutability Pattern

General ideas:

- Interior mutability allows us to mutate data even if there are immutable
  references to it.
- `RefCell<T>` enforces reference constraints in runtime rather than compile.
- With `RefCell<T>` and `Rc<T>`, we can have multiple owners and mutability
  together.

## Reference Cycles Can Leak Memory

General ideas:

- Weak references don't affect cleaning.

Language syntax:

- `Rc::downgrade` creates a weak reference (`Weak<T>`).