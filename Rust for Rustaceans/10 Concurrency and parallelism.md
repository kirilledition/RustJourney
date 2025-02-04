# The trouble with concurrency

## Correctness
Multiple thread write access is called data race. Data race considered undefined behaviour. 
## Performance
Scalability - program scales performance per number of cores. Linear scalability is ideal. Sublinear scalability - there are diminishing returns on number of cores you can add. Negative scalability - when threads contend for one shared resource, and program becomes slower. It is called contention
### Mutual exclusion
There is a critical piece of application that only one thread can access. Mutex lock. Amdahl law computes theoretical loss of performance due to introdution of mutex.

### Shared resource exhaustion
Your resources are limited and they limit scaling. For example limit of OS to send tcp requests.

### False sharing
Your threads access different parts of mutex. Sometimes it is possible to split mutex. One example is in the processor, core can only use one 64 bit cache line, so if you have whole array in one cache line, only one core can access it. You can pad values so they fall on different cache lines.

### Cost of scalability
Compiler can optimize single threaded code very well. If you introduce multithreading, you can make your code slower. Read: https://www.frankmcsherry.org/assets/COST.pdf
# Concurrency models
Rust has three models
## Shared memory
Threads operate on single shared source. Good for situations where threads need to jointly update state in a way that does not commute. You need to carefully select data structure to store shared state. For example read optimized hash map, reader/writer lock. There are lock-free algorithms
## Worker pools
There is a shared job queue and threads draw jobs from it. Usually you need some shared resource to manage how threads take jobs and return results. There is work stealing, thread that finished earlier can still job that was not started, but was assigned to other thread. Good for situations where threads perform same work on different data.
## Actors
You have separate job queues that feed particular actor. Actor owns some resource and receives messages from other actors to do something with it. Each actor may not be its own thread. Suitable for situations when you have many resources that can be operated independently and there is little opportunity for concurrency in each resource

# Asynchrony and parallelism
Async code uses only one thread, you can use more, but then your futures need to be Send and you need to specify, how you split futures to tasks.

Sync primitives like Mutex are faster then async ones, sometimes you can use them in async code, if you ensure, that it is safe.
# Low level concurrency
Atomic types provide access to CPU primitives, they are useful for light cooperation between threads. High level primitives like mutexes are built with them. They define behaviour for what happens, when multiple threads are trying to access them.

## Memory operations
CPU instructions on read and write can be changed to optimize code, for example reordered. It is usually fine, but can break multithreaded programs. CPU provides instructions with different guarantees, so you can use them safely. Rust provides access to these instructions with atomic types

## Atomic types
Atomic type variable is written all at once and will never be written by multiple stores. You cannot load variable until full variable is written into memory. CPU can access variables of certain sizes and only types with these sizes are in atomic module.
## Memory ordering
 When several threads are trying to write to one memory cpu will decide order itself. We can dictate order of memory access for given atomic type.

### Relaxed ordering
No order is preserved or guaranteed, threads can access variables in any order. Fine for cases like counter.

### Acquire/Release ordering
Makes relationship between store and load of particular value. When you load with Acquire, no other loads or stores will happen after. When you store with Release, no loads or stores can happen before - no actions before release

### Sequentially consistent ordering
Threads have same ordering as each other. You can setup order of loads and stores and other threads will respect this order.

## Compare and exchange
Conditionally replace value. Can use different memory ordering for success and failure cases. Can be scalability bottleneck, because it requires cpu to have exclusive access to memory location. There is no native operation on ARM, so program needs to loop to perform this operation. If you already have a loop, you can use compare_exchange_weak.

## The fetch methods
Perform read and store in a single step. Always performs operation on current value and returns value after operation.

# Sane concurrency

## Start simple
Start with simple algorithm, performance test to find bottlenecks, optimise bottleneck. 
## Write stress tests
Stress tests in concurrent programs try to touch as many parts of your code as possible, and hopefully will reveal bugs.
## Use concurrency testing tools
Because order of threads execution matters, it is hard to test.
### Model checking with loom
Loom checks all possible executions of a code block (all sequences of operations?). You need to pass a closure, where you replace types with special loom types. For small cases.
### Runtime checking with ThreadSanitizer
Checks for any race conditions by reordering memory accesses. You can run using rust compiler. For larger cases.

Heisenbugs - bugs that disappear, when you try to debug them. For example happens with print debugging, because print affects concurrency. 