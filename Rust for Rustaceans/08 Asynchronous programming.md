# Asynchrony
CPU mostly waits for other tasks
## Synchronous interfaces
Also called blocking, code executes one line at a time, easy to reason about.
## Multithreading
Thread is responsible for executing independent sequence of block operations. Communicate through synchronous instrument like lock or channel. Can introduce concurrency or parallelism. Starting new threads is expensive. Communicating between threads and coordinating them is complex. 
## Asynchronous interfaces
Interface can either tell that its ready or ask to come better for result. This allows to do something else while waiting.
## Standartized polling
There is standard way to poll (ask for readyness): Future trait. It implements poll function and Output type. 
Fused future - future that can be called even after its ready
# Ergonomic futures
Future code operates like state machine
## Async / await
Better way to specify state machine, implemented through generators
### Generators
Function that can preserve state. Yeilds rather then returns - returns value and saves state. Currently unstable, used internally to implement async / await. Local variables in generator are read from associated data structure rather then from stack, code for that is injected when you create async function. 

Generator must hold state, so if you have large futures with lots of big variables, this can be expensive. You can detect them with profiling, clippy. You can move future to the heap with Box::pin
## Pin / unpin
Future holds data and reference to data. When future itself is moved, data is moved, but references stay in place and become invalid. This is called self-referencial data structure. They are supported with Pin type and unpin trait.
### Pin
Prevents type from being moved. Contract that value behind Pin will never move again. Takes pointer to data. Safety guaranteed through reliance on safe methods of underlying traits. Pointer type Deref, DerefMut and Drop must not move value.
### Unpin
Unpin trait promises that type is safe to move and we will not need to use Pin safety gurantees and guarantees are safe to break. Auto-trait. Type need to explicitly opt out of Unpin with !Unpin
### Ways of obtaining a pin
Pin to heap: place value on the heap and place pointer inside pin: Box::pin
Pin to stack: use macro to overshadow value to ensure that it cannot be dropped.
# Going to sleep
Executor tries to check when future has made progress. Instead of cycling trying to check future for readiness, executor goes to sleep until woken up.
## Waking up
Executor constructs Waker instance that wakes executor when progress has been made. Weker is passed to the future inside contex. Waker wake method is called when future has made progress. Executor decides how particular waker acts by manually implementing vtable.
## Fulfilling the poll contract
Future returns Poll::Pending, when some other future inside it returned Poll::Pending. If there is no internal future, then this is leaf future. It stores waker somewhere, where it sill be triggered in case of event. Executor registers all resources, that that future is waiting for and calls wake when event occurs. In case of os calls, epoll system call is used
## Waking as a misnomer
A misnomer is a name that is incorrectly or unsuitably applied. Waker is already awake, it signals executor, that future is runnable. 
## Tasks and subexecutors

# Tying all together with spawn

