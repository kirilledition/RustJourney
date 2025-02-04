# Memory

## Variables
value - byte representation of element of specific type
variable - location "slot" on a stack that holds value
pointer - holds location to region of memory, where value is stored

When we assign string to variable, we really get a pointer to start of string on a heap

### High level model
Think about variable as a name of value. Program consists of flows, tracing lifetime of a value. Compiler can check all possible traces and restrict possibility of two parallel mutable accesses.
### Low level model
Think of variables as a slot for value in memory that has room for type.

## Memory regions
### The stack
Stores function calls as frames. Frame is a segment of memory that stores all variables inside function and arguments. When function returns frame disappear

### The heap
Values are kept until they explicitly deallocated.
You can explicitly allocate a piece of memory on a heap and get pointer to start of this piece and length of piece. 
To place value on a heap use `Box::new(value)`, get pointer in return. When pointer is dropped, memory is freed
If value was not deallocated, memory is leaking. You can explicitly leak a value with `Box::leak` and get 'static reference

### Static
One name for several regions
Contains binary code for program
Contains variables with 'static lifetime
Contains string literals
`static` keyword defines value in static region
`const` keyword calculates value during compilation - name for a value, not a place

# Ownership
Owner is responsible for deallocating/dropping value. Automatic dropping happens, when variable is no longer in scope. It is not possible to drop value several times. Types drop values recursively. Later values dropped first, because they may contain references to earlier values, nested values like values in tuple or array dropped in source-code order (from first element)

Value always has a single owner. When value changes owner, you cannot access it from previous owner. If value implements `Copy` trait, new owner gets copy of instance, and previous owner remains ownership of original value. 

# Borrowing
## Shared reference
Pointer `&T` may be shared. References themselves are copied, not moved. Value can not be changed, and compiler uses it for optimisation - reads value once and uses it in place of all references
## Mutable reference
Pointer `&mut T` can be used to change underlying value. If there is one mutable reference, there can be no more references (exclusive). Compiler uses it for optimisation - if you have two references in one code and one of them is mutable, they are definitely not pointing to the same place.  If you have mutable reference to value, you cannot move value to new owner. You can, if you give another value to previous owner.
## Interior mutability
Some types allow to change value with shared reference. They usually provide safety in two ways:
- give mutable reference through shared reference (Mutex, RefCall)
- give methods to change underlying value (Cell, integer atomics)
Cell methods either returns copy of value, either replaces underlying value
# Lifetimes
Named regions of code that reference must be valid for. It can be path of execution, and reference must be valid for the whole path. 
## Borrow checker
When reference with certain lifetime is used, it checks, if lifetime still alive to this point, by following whole path of lifetime. When branching, borrow checker checks paths in branches, they do not conflict, if paths do not intersect further. If borrow checker too conservative and can not understand, why what you are doing is safe, you use unsafe rust.
## Generic lifetimes
Type definition generic. Dropping type is a use of generic lifetimes if type implements Drop, otherwise it is not a use. Multiple generic lifetimes are requred in rare and complicated cases, when methods return references, tied to different lifetimes.

## Lifetime variance
Variance - subtype can be used in place of supertype or vice versa. For lifetimes if 'b outlives 'a, b is a subtype of a.
Three types of variance:
- covariance - subtype instead of type
- invariance - provide exactly type
- contravariance - supertype instead of types, usually in function arguments


