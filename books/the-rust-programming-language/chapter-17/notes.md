# Notes

## Using Trait Objects That Allow for Values of Different Types

General ideas:
- Rust doesn't have objects like traditional OOP languages, but it has
  trait objects, i.e. structs that implement a given trait. It gives
  flexibility similar to polymorphism.
- Trait objects force Rust to use dynamic dispatch instead of static. This
  flexibility comes with a cost: because the compiler can't know all the types
  that might be used, it can't inline the code, or optimize it.

## Implementing an Object-Oriented Design Pattern