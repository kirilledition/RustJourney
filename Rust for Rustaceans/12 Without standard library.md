# Opting out of the standard library
std library basically consists of core and alloc libraries.
core only depends on rust itself, not on operating system.
alloc contains functionality about dynamic memory allocation.
You can add no_std crate attribute to turn off dependency on std library, it switches std prelude to core prelude. 
You can add std as a feature to your library. 
Types such as Box, Iterator, Option are imported from std::prelude::rust_2021 automatically.
# Dynamic memory allocation
Heap is managed by allocator. Allocator usually is from C standard library. You can override allocator with GlobalAlloc trait and global_allocator attribute. To do this you need to implement alloc and dealloc methods. no_std excludes all types that rely on dynamic memory allocation. But you can still use them from alloc crate, because dynamic memory allocation does not require os. 
You still can implement this types yourself without heap, if you want to
# The Rust runtime
Rust has no high-level runtime with garbage collection, but there is one, that runs before main and in special conditions.
## The panic handler
In no std environment you need to implement panic handler yourself. Make function with signature fn(&PanicInfo) -> ! and decorate it with panic_handler.
## Program initialization
Before main rust runs lang_start that initialized program, for example saves command line arguments, sets up panic handling. You can opt out of it with no_main macro and write all needed initialization code yourself.
## The out-of-memory handler
In environment with no memory allocation, you need to write your own handler for cases, when allocator fails. Write handler with lang == "oom" attribute.
# Low-level memory accesses
Some devices use memory mapping - tell state of device in some predefined regions of memory. Device may implement some behavior when memory is read, so read has side-effects. There are volatile memory operations, that cannot be reordered
You can include assembly code in rust in nightly, or generate assembly object file in build script.
# Misuse resistant hardware abstraction
Write code that is impossible for misuse. Use static boolean, if you only can accept one instance of a struct.
# Cross-compilation
You an compile program for target machine on different host machine. Target is specified as macine-vendor-os
You can compile your own std for different target
