# Types in memory
Type is a way to interpret piece of memory

## Alignment
Pointer point to bytes and not bits. Type needs to be at least size of byte. On x64 cpu momory is accessed in chunks of 8 bytes - cpu word size. Accesses in the middle of word size block called misaligned and hit performance. Simple types aligned to their size, complex types aligned to largest type they contain???.

## Layout
A way type is laid in memory. Built-in ways of layout:
- repr(C) - same way as C, in the order of source code, lays paddings for type to be at least byte aligned. Struct aligned to multiple of largest alignment 
- repr(Rust) - reorder fields
- repr(packed) - without padding, misaligned memory accesses
- repr(align(n)) - specify alignment size https://rust-lang.github.io/rfcs/1358-repr-align.html
- repr(tranparent) - for single field structs, alignment is exactly the same as underlying field

## Wide pointers (fat pointers)
Dynamically sized types - size cannot be known at compile time (slice, iterator). Types need to be sized for struct fields, function arguments, return values, variable types, and array types. To pass dynamically sized value, you need wide pointer - pointer with additional word sized info like length of slice. Wide pointer is sized. Automatically constructed when taking reference to dynamically sized type

# Traits and trait bounds

## Dispatch (отправка)

### Static dispatch
For each generic type T functions being copied and compiled. Only happens for T that use particular generic. For each generic type T generic function is being transformed into separate object in memory. This called "monomorphisation". CPU instruction cache is less effective. Piece of generic function that does not rely on generic type T, can be extracted to helper function - it will be compiled once. "non generic inner function"

What is CPU instruction cache? 

### Dynamic dispatch
Allows use one compiled generic function for multiple types.

Vtable - virtual method table, contains addresses to methods of an object, that are required by trait definition
Trait object - combination of type and vtable.

dyn Trait keyword tells that you want dynamic dispatch. Gives vtable. If it is an argument to function, it needs to be a reference, because size of dynamic is unknown. Gives wide pointer. Clone and Extend can not by dynamic because they are not object-safe

Object-safe means, that method signature does not depend on the type it is called on

Self: Sized trait bound require that trait or method can not be accessed through dynamic dispatch

 Improves CPU instuction cache efficiency, but compiler can not optimize method for specific type. Guidance: do not use dynamic dispatch in libraries, users will decide for themselves. In binary application decide for yourself.

https://geo-ant.github.io/blog/2023/rust-dyn-trait-objects-fat-pointers/
https://alschwalm.com/blog/static/2017/03/07/exploring-dynamic-dispatch-in-rust/

## Generic traits
### Associated types
```rust
trait Contains {
    type A;
    type B;

    // Updated syntax to refer to these new types generically.
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
}
```
Use when expect only one implementation of trait for given type

Do not allow multiple implementations

### Generic type parameters
```rust
trait Foo<T>
```
Use when you expect several implementation of trait for given type. Hard to maintain. 

Allow multiple implementations. For example, you can implement PartialEq against multiple right-hand side types for your type

## Coherence
For each type and method there can be only one correct implementation. This is facilitated by Orphan rule. you can implement trait for type only if trait or type is local to your crate. But there are exceptions
- Blanket implementations - `impl<T>` can only be done by crate local to trait
- Fundamental types - marked with #[fundamental], allow anyone to implement trait on them (&, &mut, Box). They are erased before orphan rule is checked
- Covered implementations - you can implement foreign trait on foreign type if trait is generic over local type

## Trait bounds
Can be in form of type restrictions. You can bound associated generic types of generic type.

Derive trait macros expands to `impl Trait for Foo<T> where T: Trait`. It is not always what you want, sometimes makes unnecessary bounds, when using complex types, that contain other generic types.

## Marker traits
Do not implement anything. Just tells compiler, that type can be used in a particular way. For example Send trait tells compiler, that type can be safely send across threads. Send, Sync, Copy, Sized, Unpin. Compiler seem to implement them somehow

Marker types - `struct Mymarker;` make it impossible to misuse API.

# Existential types
Compiler can perform type inference, if type was not specified. Infers type for variables and closure arguments and return types.

Return type `impl Trait` and `async fn` have existential return type. They do not specify exact type returned. Compiler infers their return type.