# Notes

## Packages and Crates

General ideas:
- Crates can contain modules.
- A crate can be a binary crate or library crate.
- Binary crates have a `main` function and are executable.
- Library crates don't have a `main` function and instead of compiling to an
  executable, it defines shared functionality.
- Packages are bundles of one or more crates.
- A package can have multiple crates, but only one library crate.

## Defining Modules to Control Scope and Privacy

General ideas:
- Everything inside a module is private by default.

Language syntax:
- `mod` defines modules.

## Paths for Referring to an Item in the Module Tree

> _Adding the `pub` keyword in front of `mod hosting` makes the module public.
> [...] But the contents of `hosting` are still private; making the module
> doesn't make its contents public. The `pub` keyword on a module only lets
> code in its ancestor modules refer to it, not access its inner code. Because
> modules are containers, there's not much we can do by only making the module
> public; we need to go further and choose to make one or more of the items
> within the module public as well._

General ideas:
- A public module still has everything private by default.

Language syntax:
- `pub` makes a module public.

## Bringing Paths into Scope with the use Keyword

Language syntax:
- We can use `use` to bring a path to scope.
- By using `as` we can add an alias for a type.
- `pub use` brings a path to scope and re-exports it to other modules.

## Separating Modules into Different Files

General ideas:
- Ideally, each module has its own file.