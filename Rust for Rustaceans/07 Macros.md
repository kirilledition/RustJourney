Automatic code substitution where you define rules of substitution
# Declarative macros
Function-like macros
Defined using macro_rules!
macro_rules! and format_args! are not declarative macros
! indicates that compiler will replace macro to other source code
macros are not allowed where identifier of match arm is expected
"declare" how to transform input to output
## When to use them
Instead of writing repetitive code
tests: when you have same test for slightly different configuration like different types
traits: implement trait with trivial implementation for several types. Can use blanket implementation, but then other crates' types will need to use same implementation
Rule of thumb: if code changes based on types, use generics, otherwise use macros
## How they work
Compiler parses source code into sequence of tokens and then to abstract syntax tree (ast). 
Macros are transformed to replacement tokens and then to new ast. 
First compiler parses macro input into ast, which means that input must be parsable.  
Declarative macros cannot generate invalid rust code, they wont compile.

## How to write declarative macros
Macro consists of matchers and correponding transcribers. Can have several pairs, compiler will find the one, that matches from the first to last
### Matchers
Token tree template that compiler tries to bend to match input token tree.
Fragment types - :ident, :expr, :ty, :tt and more
($()) - repeated match

### Transcribers
Rule to generate code from matcher
Matcher defines metavariables, that later will be substituted inside transcriber.
For repeated matchers, mirror the same repeated pattern.

### Hygiene
Macros does not interfere with outside variable names. Generated code variables exist in their own namespace. Everything should be explicitly passed inside. This only applies to variable identifiers
Macros can declare new functions or new implementations for existing types (not hygenic).
You need to assume that every type in your macro can be already defined differently in the outside. For example custom result type imported. Should specify fully. 
You can share identifiers between outer code and macro, you need to pass variable name as metavariable

Macros must be declared before using. `#[macro_export]` makes macro public and moves to the root of program.
# Procedural macros
Function that runs at compile time. Compiler passes sequence of tokens to macro and outputs new sequence of tokens. You define how to generate code

https://blog.jetbrains.com/rust/2022/03/18/procedural-macros-under-the-hood-part-i/
## Types of procedural macros
Specialised on different use cases. Differ in how incoked and how input is handled. 
### Function like macros
Like declarative but does not require hygiene. You need to specify which identifiers are external and with are internal by Span::call_site, Span::mixed_site. 
### Attribute macros
Replaces whole annotated item, takes tokens from attribute itself and item.
### Derive macros
Seems the same as Attribute, but does not replace original code, appends code after your definition 
## Cost of procedurial macros
Eats compilation time by heavy dependencies and by generating huge amounts of boilerplate code, that needs to be optimized by compiler.
## So you think you want a macro
### When to use derive macros
Automate trait implementation when automation is possible. Use for cases with obvious implementation and for cases, when trait needs to be implemented a lot of times.
### When to use function like macros
- Replace hard to maintain declarative macro
- Need to execute actual function at compile time
### When to use attribute like macros
- Test generation
- Framework boilerplate like `#[get("/<name>")]`
- Middleware that adds feature, but does not change functionality
- Type transformation
## How do they work
TokenStream type that can be iterated and analyzed. Valid rust syntax can be parsed to token stream, can be parsed from string. Each token has a span - where token is originate from. You can mark token to make it resolved from surrounding scope or from macro definition scope.