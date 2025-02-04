# Unsurprising

API should not surprise user. If API can behave exactly the same way user expects, it should 

## Naming practices
Commonly used method names should behave the same, as their well known counterparts. Method iter should take &self and give iterator, type Something error should implement error::Error

## Common traits for types
Types should implement common traits, because user will expect them to be there, and he will not be able to implement them himself, because of coherence rule. Common traits to implement:
- Debug for println
- Send for multithreading
- Sync
- Clone
- Default for providing default value
- PartialEq for assert_eq
- PartialOrd for items in collection
- Hash for use as keys in hashmap
- serde Serialize and Deserialize (make optional for user)

Copy implementation is not expected, should not implement. Removal of Copy is backward incompatible change.

https://oswalt.dev/2023/12/copy-and-clone-in-rust/
https://blog.logrocket.com/disambiguating-rust-traits-copy-clone-dynamic/

## Ergonomic trait implementation
Blanket implement traits for references, mutable references and Box references. User expects to be able to use type method on reference to type

```rust
impl<T> Speaker for &T
where
    T: Speaker,
{
    fn speak(&self) {
        return (**self).speak();
    }
}
```

https://www.judy.co.uk/blog/rust-traits-and-references/

## Wrapper types
If your type is a transparent wrapper to some other type you should implement `Deref, AsRef, From<InnerType>, Into<InnerType>`
If type is equivalent to something, like str and String, it should implement Borrow

https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch15-02-deref.html
https://doc.rust-lang.org/std/borrow/trait.Borrow.html

### Deref
Type that allows to call methods on inner type, should avoid methods, that take self. Instead use static method T::frobnicate(t: T)

# Flexible
Code is a contract with promises and requirements

Requirements - argument types and trait bounds. Avoid unnecessary requirements

Promises - trait implementations and return types. Make promises you can keep.

## Generic arguments
Use generic argument with trait bound to allow user to supply type, that satisfies what your function actually needs. Make an argument generic if you
can think of other types a user might reasonably and frequently want to use
instead of the concrete type

Don't use dynamic dispatch, if you thing your function will be used in performance sensitive setting, as user will not be able to change dynamic dispatch. Dynamic dispatch does not allow to use complicated trait bounds

Changing concrete types to generic types is not always backward compatible
## Object safety
Try to provide object safety. Even if it comes at a cost of ergonomics. Object-safety provides new ways to use your trait

Drop also should be object-safe. Object-safety is part of promise, change is not backward compatible
## Borrowed vs owned
If function does not need to own the data, it should operate on reference. If function needs own data it should make user to provide owned data.

## Fallible and blocking destructors
Some types need to perform cleanup before they are dropped. It is bad idea to perform this process in drop, because if there is an error in cleanup process, type cannot communicate it, as drop does not return anything

We should provide explicit destructor that takes self and returns Result. Drop cannot call destructor, because drop does not own self, but &mut self. We can std::mem::take self and destruct it inside drop

ManuallyDrop inhibits compiler from automatically all destructor, unsafe

# Obvious
Make your interface as easy as possible to understand and as hard as possible to misuse.

Adding precaution in naming might help for example `dangerous_` in method name
## Documentation
Document everything your code might do besides function signature. For example panics. Document in which condition code panics

For unsafe code write, what user needs to guarantee to make code safe.

Provide end-to-end usage example on crate and module level. Helps to understand, how your module components fit together.

Organise documentation. Group related items to modules. Use intra-documentation links between related items. Use `#[doc(hidden)]` for private items. 

Enrich documentation with external links, resources, whitepapers. Use
`#[doc(cfg(..))]` to highlight items that are available only under certain configurations. Use `#doc(alias = "..")` to allow user to find function with other name.
## Type system guidance
Semantic typing - types, that represent meaning. For example use enum with variants DryRun::Yes and ::No instead of boolean flag dry_run: bool. 

Zero size types to indicate something. Introduce generic parameter to struct, that represents state of struct, use generic implementation to implement methods only relevant to particular state.

If you have two parameters - value and flag on weather to ignore this value, use enum with variants value and none

Warn user to handle a return with `#[must_use]`
# Constrained
Assume that user uses everything that is publicly exposed in your interface. Changing behaviour of anything public is a breaking change.
## Type modifications
Have as few public types as possible, because, you will not be able to easily change them later. Use pub(crate) and pub(in path).

Adding new private field to public type is a breaking change. Use `#[non_exaustive]` to restrict usage of constructor from public fields.

https://doc.rust-lang.org/reference/visibility-and-privacy.html#pubin-path-pubcrate-pubsuper-and-pubself

## Trait implementation
Trait implementation is sometimes a breaking change, because user could have implemented methods with same names

Sealed traits allow to mitigate most of incompatibility issues. They restrict user from implementing trait, but allow to use

https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/

## Hidden contracts
Change in code sometimes break contract elsewhere in interface.
### Re-exports
You should not expose types from foreign crates, because then breaking change in foreign crate will break your interface. Use newtype wrapper pattern to hide foreign type from interface

Semver trick - if change in type is not a breaking change, and you want to make it available in previous version, you may create new minor version of previous major version and export this type from you current major version
### Auto traits
Sometimes you unexpectedly break traits that are auto-derived by compiler. To check for it, you can introduce additional test
```rust
fn is_normal<T: Sized + Send + Sync + Unpin>() {}
#[test]
fn normal_types() {
	is_normal::<MyType>();
}
```

If you hide item from documentation, but leave it public, you may consider change in it non-breaking change. But you still need take into account public effect of your changes