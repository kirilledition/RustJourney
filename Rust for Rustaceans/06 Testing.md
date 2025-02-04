# Rust testing mechanisms
cargo test passes --test flag to rustc, produces special test binary that runs all unit tests
## The test harness
Compiler with test option generates special main function, that runs tests and includes default test flags like --test-threads. It is called test harness

Each unit test annotated with `#[test]` is compiled into descriptor and exposed to harness.

Integration tests (in tests directory) compiled to separate crates and have access to main crate public interface. Harness generated for each file but not for subdirectories to allow having shared submodules. You can create a harness for subdirectory, by creating file main.rs inside. Integration tests useful for benchmarks

You can create your own test harness by specifying cargo.toml
```toml
[[test]]
name = "custom"
path = "tests/custom.rs"
harness = false
```

Default harness has lots of useful flags like --nocapture to avoid capturing output, --test-threads - run tests concurently, --skip test, --ignored - to run ignored test, --list tests

## `#[cfg(test)]`
Allows to have code, that is compiled for testing.
### Mocking
Fake object used for testing, for example fake network to test networking functions. Usually mocking requires generics - use mock structure as generic parameter. If generic is inconvenient, genetare in cfg(test). Best library - mockall
### Test-Only APIs
Sometimes you need to test private functions or fields, that are dangerous to touch in production. You can then generate test methods that allow you to access private properties in test module.
### Bookkeeping for test assertions
You can keep track of information related to structs. For example number of reads in buffer. Annotate line that executes logging. While integration testing main crate is being compiled as normal, not as test, so this features will not be available.
```rust
struct BufWriter<T> {
#[cfg(test)]
write_through: usize,
// other fields...
}
impl<T: Write> Write for BufWriter<T> {
fn write(&mut self, buf: &[u8]) -> Result<usize> {
// ...
if self.full() {
#[cfg(test)]
self.write_through += 1;
let n = self.inner.write(&self.buffer[..])?;
// ...
}
}
```
## Doctests
Tests examples in documentation run automatically as test cases. Adds main function around example code, can specify your own for async case. If code uses?, test will add `Result<(), impl Debug>` as return type. If you struggle with type inference, you can disambiguate it by specifying `Ok::<(), T>(())` at the end.

Some code is example is needed for example to run, but you dont want to show it to user. Comment it with # or `/*..*/`. 

Examples can have attributes like compile_fail to signify, that failed compilation is desired behaviour.  

# Additional testing tools

## Linting
Linter checks are almost like tests, should add to CI. Some lints you may not want, then you can skip it with `#[allow(clippy::name_of_lint)]`. Some are disabled, but you may want to enable them as good practice `#![warn(rust_2018_idioms)]`, some of them: missing_docs, missing_debug_implementations, rust-2018-idioms, rust-2021-compatibility, rust-2024-compatibility.
## Test generation
When you write test, it only covers behaviours, that you considering at the time you wrote them. You can use more extensive tests with test generation techniques.
### Fuzzing
Generates random inputs for you function, usually checks for failure or panic. You can use arbitrary create to randomly generate some of your own structs. Implement trait Arbitrary with arbitrary function, can use derive macro. Tools: libfuszzer_sys, arbitrary, cargo-fuzz
### Property based testing
A lot like fuzzing, you define code, that you trust to behave correctly and check, if your code behaves the same way. For example you optimise algorithm or replace standard library functionality. If you want to test sequence of operations you can define `Vec<Operation>`
## Test augmentation
Sometimes your test randomly fails with something like segmentation fault. Usually it can happen due to several reasons: race conditions, undefined behaviour ans unsafe code. You can use tools to catch for this cases. Miri - interpretes rust code and catches all the undefined behaviours in your code, use with cargo test miri. Loom - checks for concurrent operations such as access to mutex from two threads
## Performance testing
If your code runs 100 slower or 100 faster, it is probably a bug, you should test for such cases.
### Performance variance
Performance results may vary because of external reasons. One way to mitigate it - look at the distribution of test results. Crate hdrhistogram helps to look at test statistics. Crate criterion allows to use statistical hypothesis testing and creates report with analysis of outliers and graphics. 
### Compiler optimisations
Sometimes your benchmark need to use redundant operations. If compiler notices it, it can remove it. For such cases use std::hint::black_box - compiler with think, that code is needed and used and will not remove it.

Check what compiler does with godbolt.org or cargo-asm
### I/O overhead measurement
If you add IO related functions in you benchmark loop, it eats performance and may overshadow actual performance of your code. For example - random number, thread spawn, println. Remove them from your performance checking loop. Crate criterion provides timing loops.
