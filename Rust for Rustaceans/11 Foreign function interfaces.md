# Crossing boundaries with extern
Accessing bytes that originate from outside rust code. 

## Symbols
Name, that points to location of function or static variable in compiled binary. Generic functions have multiple symbols. They are used internally by compiler and have semi-random names, as they are not used, in your code. We need a way to point compiler to right symbol, as random names wont work and particular names may be duplicated. Every symbol can only point to one memory (definition), but can be reffered to multiple times (declaration)
### An aside on compilation and linking
Compilation consists of three phases.
Compilation - optimizing a code and generating low level representation of code
Code generation - generating machine code in form of binary artefacts, object files
Linker - stiches together several object files to one binary program, replacing every symbol reference with final location in resulting file. Linker does not care, where object file originates from, as long, as it has same symbols. Linking can be static and dynamic. Static is used for rust code, dynamic for ffi. 
### Using extern
You can force rust to not use random name with no_mangle attribute. Specify particular name for symbol with export_name. Inside extern block you can define variable of function. Definition will be pulled during linking from another object file. You can specify location of symbol with link(name = ""), rename symbol with link_name. Acessing functions and variables from extern is unsafe.
### Link kinds
Can be specified with link attribute kind argumant.
Dynamic linking - smaller binary, user can upgrade library independently of your code. 
Static linking - ease of destribution
## Calling conventions
Tells assembly code, how to invoke a function. Default for extern is C. Default for every other function is Rust. Unwinding only works with regular function, to use in C, you can use C-unwind. There are also system and stdcall.
## Other binary artefacts
When you publish to cargo, you publish code. If you want to distribute binary library, compile it as dynamic of static library object. You can specify it in lib in cargo.toml. Rust compiles dependencies to binary library artefacts called rlibs. 
# Types across language boundaries
Type layout must be the same between two languages in FFI.
## Type matching
You need to define type on both sides of FFI. To define C type in rust, you can use repr(C) attribute. Use c_int, c_char etc. for primitive types, CStr and CString for strings. Vec::into_raw_parts will help to constrict vector. Enum has different representations for enum with and without data.
## Allocations
Memory can only be freed by entity, that allocated it. There are two ways of managing memory. External library defines functions for allocations and destruction. Calling library allocated memory and passes it to external function, that way calling function has control over memory. Second way is preferred.
## Callbacks
You can define dunction in calling library and pass it to external library, that is called callbacks. Functions with panic in external library are undefined behaivour, use std::panic::catch_unwind.
## Safety
Calling ffi is unsafe, so you need to provide safe interface for it. 
### References and lifetimes
If external code may modify data, make sure to have exclusive reference on that in calling function. Use rust lifetimes to enforce requirements of ffi. 
### Send and sync
Only implement them if external library explicetely say, that they are safe to use across different threads. If several functions must be called only on the same thread, you can bind them in one struct.
### Pointer confusion
If interface does not want you to act on some pointer, you can define it as void pointer. To not confuse different types of void pointers, you can wrap them in different rust types.
# bindgen and build scripts
bindgen is a tool that generates rust code from c header files. You can invoke it standalone or as part of build of your crate. 
You can use build script in form of rust file, that runs before compilation. It can be either called build.rs, or specified in cargo.toml. 
Generated bindings should be in separate crate.
If you want other languages to use your rust code, you should provide c header file for your library. It can be generated with cbindgen.

C++ need to be bound differently then C. bindgen supports generating bindings for C++, you can use cxx crate.