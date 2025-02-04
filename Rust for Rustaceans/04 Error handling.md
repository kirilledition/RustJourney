# Representing errors
Need to consider, if user really needs to know exactly what happened, or just, that something went wrong
## Enumeration
If user needs to distinguish between different error cases, you can use enumeration. Make a enum, that wraps around error type.

When making own error type, error should implement this traits:

std::error::Error trait with Error::source method

Display and Debug traits - Display should provide one line lowercase description without trailing punctuation, Debug should provide more information including auxiliary information for example what port and what file

Send, Sync trait to make it possible to use error in concurrent setting

Error should be 'static
## Opaque errors
In cases, where it is not important, what went wrong. Extreme example is `Box<dyn Error + Send + Sync + 'static>`. 

They do not add a lot of weight, they are easy to compose from different sources

'static on error allows access to downcasing - casting item to more specific type. You can use Error::downcast_ref to get underlying dyn Error
## Special error cases
If you can not return meaningful error, return `Result<T, ()>`. It is almost the same as `Option<T>`,  but with different semantics. Option means, that there is nothing to return, Result means, that something went wrong

You can use never type ! if function never returns an error, but needs to return Result - `Result<T, !>`

std::thread::Result does not contain Error, but Any, because it happens in case of panic in thread, so there is nothing to do about it
# Propagating errors

? operator - unwrap and return early. It performs conversion through From trait
Syntax sugar for unstable trait Try
### From and into traits
They are there for complicated historical reasons. Implement From and use Into in bounds, because From auto implements Into