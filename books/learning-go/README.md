# Learning Go

![Go](https://img.shields.io/badge/go-%2300ADD8.svg?style=for-the-badge&logo=go&logoColor=white)

These are my personal notes from **Learning Go (2nd edition)**. I'm
approaching this as an experienced software engineer exploring Go's approach
to simplicity, concurrency, and systems programming. My notes focus on what's
_different_ from other languages, _essential_ Go idioms, and _conceptually
important_ patterns like goroutines/channels, interfaces, error handling, and
Go's unique approach to composition over inheritance.

## Table of Contents

- [Chapter 01: Setting Up Your Go Environment](#chapter-01-setting-up-your-go-environment)
- [Chapter 02: Predeclared Types and Declarations](#chapter-02-predeclared-types-and-declarations)
- [Chapter 03: Composite Types](#chapter-03-composite-types)
- [Chapter 04: Blocks, Shadows, and Control Structures](#chapter-04-blocks-shadows-and-control-structures)
- [Chapter 05: Functions](#chapter-05-functions)
- [Chapter 06: Pointers](#chapter-06-pointers)
- [Chapter 07: Types, Methods, and Interfaces](#chapter-07-types-methods-and-interfaces)
- [Chapter 08: Generics](#chapter-08-generics)
- [Chapter 09: Errors](#chapter-09-errors)
- [Chapter 10: Modules, Packages, and Imports](#chapter-10-modules-packages-and-imports)
- [Chapter 11: Go Tooling](#chapter-11-go-tooling)
- [Chapter 12: Concurrency in Go](#chapter-12-concurrency-in-go)
- [Chapter 13: The Standard Library](#chapter-13-the-standard-library)
- [Chapter 14: The Context](#chapter-14-the-context)
- [Chapter 15: Writing Tests](#chapter-15-writing-tests)
- [Chapter 16: Here Be Dragons: Reflect, Unsafe, and Cgo](#chapter-16-here-be-dragons-reflect-unsafe-and-cgo)
- [Next Steps](#next-steps)

---

## Chapter 01: Setting Up Your Go Environment

[Go](https://go.dev) programs compile to **one** binary and don't need any
dependencies to execute them.

The development tools are accessed with `go`:
- `go build`: Code Compiler.
- `go fmt`: Code formatter.
- `go vet`: Code scanner.
- `go mod`: Dependency manager.
- `go test`: Test runner.

---

## Chapter 02: Predeclared Types and Declarations

Variables can be declared in many ways:
- Using `var` and explicit type: `var x int = 10`.
- Using `var` without type: `var x = 10`.
- Using `:=` **inside functions**: `x := 10`.
- Without assignment: `var x int`.

- It's a compilation error to have unused variables.
- Variables without assignment get **zero values** (`0`, `false`, `""`, `nil`).
  This makes code safer by default.
- **Visibility is determined by capitalization**: `Name` is exported (public),
  `name` is unexported (private to the package).

---

## Chapter 03: Composite Types

Arrays aren't used directly, because they're too rigid. Its size is part
of its type, which means we can't resize them or convert to other arrays
directly. Instead, we use slices.

A slice is similar to an array, but more flexible:
- Slices have `length` and `capacity`. The former are the actual elements,
  while the latter is the reserved memory.
- `make` creates a slice with zero-values.
- `append` adds elements to the end of the slice. When capacity is exceeded,
  Go allocates a new array and copies the data.
- **Subslices can produce confusing behavior, because they share part of the
  memory.**

Go also has `Map`:
- Maps are defined with `map[keyType]valueType`.
- Maps' keys must be comparable.
- Maps return zero-value for missing keys.

---

## Chapter 04: Blocks, Shadows, and Control Structures

Go has **shadowing variables**:
- A shadowing variable is a variable with same name in a containing block.
- It's not possible to access the shadow variable while the shadowing exists.

It's possible add variables scoped to an `if/else` block:
```go
if n := rand.Intn(10); n == 0 {
    ...
} else if n > 5 {
    ...
} else {
    ...
}
```

We can create a `for` loop in four ways:
1. C-style: `for i := 0; i < 10; i++ { ... }`
   - In this case, the variable has to be declared using `:=`.
2. `while`-like: `for i < 10 { ... }`
   - We have to declare `i` outside of the loop, and control inside.
3. Infinite loop: `for { ... }`
   - We use `break/continue` to control the loop.
4. `for-range`: `for i, v := range values { ... }`
   - Similar to Python's `for i in range()`, and Java's "for each".
   - The index (first variable) can be ignored: `for _, v := ...`.
   - We can also just use the indexes, ignoring the value: `for i := ...`.

Iterating over maps produce random order by design.

`switch` statements don't fallthrough automatically, removing the need for
`break`ing:
- We can `switch` on any comparable.
- Cases can be constants or expressions.

---

## Chapter 05: Functions

There are no named or optional parameters in Go functions, but it does support
**variadic parameters**:
- Variadic parameters are the last parameters of a function, and have `...`
  before the type: `func sum(first int, second int, rest ...int) int { ... }`.
- It's possible to return a tuple.
- Functions can return an error as the last value of a tuple.
- We must assign all returned values to variables (destructure), but we can
  ignore some using `_`.
- We can return **named returned values**. These are predeclared variables
  that are initialized to their zero-values.
  - It can shadow other variables.
  - We don't actually have to return them.
  - Writing `return` without values (naked return), return the last assigned
    values to the named returned values.

Go functions are values:
- Functions can be assigned to variables, passed as parameters to other
  functions, and returned as result from other functions. 
- We can define function types (`type func...`).
- It's possible to create anonymous functions.
- Inner functions can reference variables in the outer function (closure).

We can automatically delay/schedule a function to be run until the end of a
function execution using `defer`:
- Arguments are evaluated immediately, when using `defer`.
- We can defer multiple functions (LIFO order).

> [!DANGER]
> ```go
> func main() {
>     i := 0
>     defer fmt.Println(i)
>     i++
> }
> ```
> 
> **Output is 0, not 1, because of the immediate evaluation of the argument.**

---

## Chapter 06: Pointers

Pointers hold the memory location of a value:
- `&` is the address operator and returns the address of the variable:
  `&variable`.
- `*` is the indirection operator and returns the value of the pointer:
  `*pointer`.
- `new` creates a pointer to a zero-valued type.
- The difference in performance between passing a pointer and a value is, in
  most cases, irrelevant.
- We can't take pointers to constants and literals: `&42` and `&"hello"`.

Variables are always copied as values by default, which means updates to the
copies aren't reflected in the original variables:
- Maps are implemented as pointers to a struct. When we pass it to a function,
  the produce a copy to the pointer, which in turn updates the actual values
  in memory.
- Slices are defined as length, capacity and a pointer to the array. When
  passed to a function, the slice header is copied (pass-by-value), but the
  pointer inside still references the same underlying array, so contents are
  mutable, but length/capacity changes don't affect the caller's slice.

Go GC favors lower latency over high throughput:
- We should avoid using pointers to reduce GC work.

---

## Chapter 07: Types, Methods, and Interfaces

Methods are like functions, but they have a `receiver`:
```go
func (r Receiver) Name(params []string) (result string)
```

Receivers can be either a pointer or a value:
- If the method modifies the receiver, we must use a pointer receiver.
- If the method needs to handle `nil`, we must use a pointer receiver.
- If the method doesn't modify the receiver, we can use a value receiver.
- **If a type has any pointer receiver methods, convention dictates that all
  methods use pointer receiver.**

We can declare types in terms of other types: `type Percentage int`.
- Go doesn't have inheritance. We can't use `int` and `Percentage`
  interchangeably.

Go has weird enumerations with `iota`:
1. First we define a type based on `int`: `type MailCategory int`.
2. Then we create a constants block:
   ```go
   const (
       Uncategorized MailCategory = iota
	   Personal
       Spam
       Social
	   Advertisements
   )
   ```

We can add fields in `struct`s without adding a name. This makes the field an
**embedded field**, and promotes all fields and methods on the embedded type,
and they can be invoked directly:
- This is still not inheritance!

Interfaces are defined using the `interface` keyword:
- Go uses **duck typing** instead of a declaration of implementation.
- A `nil` pointer is not a `nil` interface. An interface is only `nil` if both
  type and value are `nil`.
- An empty interface means a type that implements zero or more methods (every
  type). `any` is a type alias for `interface{}`.
- When using interfaces, it's common to use a `type switch`. It looks just like
  a regular `switch` but we use `.(type)`: `switch j := i.(type)`.
- It's possible to implement interfaces on functions.

> "Accept interfaces, return structs."
> -- Jack Lindamood

```go
// Example of function implementing an interface

type Handler interface {
	ServeHTTP(http.ResponseWriter, *http.Request)
}

type HandlerFunc func(http.ResponseWriter, *http.Request)

func (f HandlerFunc) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	f(w, r)
}
```

---

## Chapter 08: Generics

---

## Chapter 09: Errors

---

## Chapter 10: Modules, Packages, and Imports

---

## Chapter 11: Go Tooling

---

## Chapter 12: Concurrency in Go

---

## Chapter 13: The Standard Library

---

## Chapter 14: The Context

---

## Chapter 15: Writing Tests

---

## Chapter 16: Here Be Dragons: Reflect, Unsafe, and Cgo

---

## Next Steps