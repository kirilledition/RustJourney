Invariant - something that must be true for your program to be correct
# The unsafe keyword
Unsafe in front of function tells user that function is unsafe and he should ensure safety,  allows unsafe operations inside keyword (is it still true in modern rust?)
Unsafe block assumes that user has ensured safety, allows unsafe operations in block.
It is best to use both, mark function as unsafe and put unsafe operations into unsafe block.
# Great power
Unsafe allows calling unsafe functions and dereference raw pointers. Also accessing mutable and external static variables, fields of unions.
## Juggling raw pointers
Raw pointers: `*mut T *const T std::ptr::NonNull`
You can safely cast pointer to reference, but casting back is unsafe
Common operations are: `&*ptr and &mut *`
### Unrepresentable lifetimes
Self referential structs cannot represent their own lifetime with rust lifetime system. They can use raw pointer to value and then unsafely cast pointer to reference when needed.
### Pointer arithmetic
.offset, .add, .sub to move pointer by byte. Useful in space optimized environment.
### To pointer and back again
Usually standard types have method to turn raw pointer to them. 
### Playing fast and loose with types
You can cast any raw pointer type to another raw pointer type - interpret underlying bytes as another type. Useful for FFI
## Calling unsafe functions
Usually unsafe functions operate on raw pointers in some way.
### Foreign function interfaces
Using extern blocks you tell the compiler, that final object will appear in final linked binary. Calling foreign functions is never safe.
### I'll pass on safety checks
Some types check safety themselves, like bounds of array. If you want to skip it, you can use unchecked methods with unsafe block. Useful in high performance environment.
### Custom invariants
Functions that perform unsafe operations, but ensure safety somehow by themselves. 
## Implementing unsafe traits
Traits are unsafe to implement, because they rely on correctness of implementation.
### Send and sync 
Safe to send and sync across thread boundaries. If send and sync were safe, some type containing them would become send and sync, which is not always expected by developer, so they are not.
### GlobalAlloc
Allows to implement memory allocation given type layout. Seems like whatever can happen if you deal with memory alignment. 
### Surprisingly not unpin
Implementing unpin for a type does not allow to unpin !Unpin type. Drop is a safe way to unpin you type.
### When to make trait unsafe
When memory unsafety can be caused by not correct implementation of trait.
# Great responsibility
https://rust-lang.github.io/unsafe-code-guidelines/
## What can go wrong
Unsafe code produces undefined behaviour - compiler will do something, but nobody knows what, and different version of compiler can do it differently. Three types of undefined behaviour:
- No visible errors, correct
- Visible errors, incorrect
- No visible errors, incorrect
## Validity
Rules for type's values
### Reference types
- never dangle
- always aligned
- always point to valid value for target type
- never shared and exclusive reference at the same time
- never multiple exclusive references
- never change target value during reference lifetime (except UnsafeCell)

It was not possible to obtain reference to field of uninitialised struct, but now you can with ptr::adr_of!, that uses raw references
### Primitive types
Restricted on values that they can hold. Cannot be constructed from uninitialised memory. 
### Owned pointer types
Types that point memory they own hold exclusive reference to data. Otherwise you cause undefined behaviour 
### Strong invalid values 
You can initialise type with invalid initialised memory with MaybeUninit. For example initialise slice with `[MaybeUninit::<u8>::uninit(); 4096]` instead of zeros to save some performance on initialisation.
## Panics
In unsafe code unwinding of panic may break the program. You need to consider effects of your code, that was not finished executing. Or you can ensure, that no panics going to happen
## Casting
Different types of repr(Rust) are not represented the same way in memory, so you cannot safely cast between them. Even wrapper types around same type does not have same representation. Repr(C) has more guarantees, so it is easier.
## The drop check
If type is generic over some type, that does not implement drop, and first type is provided mutable reference, then first type will certainly not touch reference again?????

Watch: https://www.youtube.com/watch?v=TJOFSMpJdzg

# Coping with fear
Be sure, that there is no safe way to implement
## Manage unsafe boundaries
Do not remove unsafe marker from unsafe methods. Think about where your unsafe code reaches, will the safety be preserved everywhere? If your code relies on not implementing certain trait, do not give public access to code, that can implement the trait.
## Read and write documentation
Document safety invariants on all levels, that it may be important for developer. At any level, where you can break safety. Document each unsafe block about how you know that it is safe.
## Check your work
Use tools to check your code. Use them in CI
Miri - interprets running code and checks for bad things your program is doing. Checks only running code, use on test suit.
Google AddressSanitizer - detects memory errors in compiled code
## House of cards
Other languages also provide a way to use unsafe code. Usually in form of C extensions. Rust allows the same, by allowing you to write extensions in the same language