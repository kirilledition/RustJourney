# Features
A way to customise project. Usually in three ways
- Enable optional dependecies
- Include additional components
- Augment behaviour of code 

Features should only add functionality, not remove or change existing. No conflicting features. If developer adds something, feature or new dependency, it should not break existing code

Configure CI to check, if crate compiles with all combinations of features - cargo-hack
## Defining and including features
Feature is a array of other features or optional dependencies that it enables. 

You can name feature default and it will be allways compiled with feature. When installing dependency, you can turn it off with default-features=false

Every optional dependency is a feature

## Using features in your crate
Using `#[cfg(feature=)]` can annotate items in code, that will compile if feature is turned on. `cfg!(feature = "some-feature")` is equivalent to if true only if the derive feature is enabled

Modifying public API with features make them non additive and non backward compatible. If you add attribute to struct with feature, annotate it with `#[non_exhaustive]`.  

If you expect users to use only small piece of your big library, make larger pieces features, so users don't pay unnecessary compilation cost

# Workspaces
Compiling whole project on every change can be expensive, so you can split your crate to sub-crates, and make them depend on each other.  Workspace in cargo.toml contains paths to sub-crates, each of subcrates has its own cargo.toml

Sub crates should have prefix with main crate name tokio, tokio-test, tokio-macros

If sub-crates are published, you should specify published version of crate instead of path

# Project configuration
Useful options in Crate.toml
## Crate metadata
Useful to include readme, keywords, categories.

Include and exclude allow to manage, what files to include in package while publishing. Can publish to alternatives to crates.io. I found only lib.rs, maybe can publish to both? 

List of all options - https://doc.rust-lang.org/cargo/reference/manifest.html
## Build configuration
`[patch]` - allows to substitute dependency with local path, or different git branch
`[profile]` - additional options to rust compiler. 
opt-level - optimisation level 1 to 3, "s" for binary size. codegen-units - splits crate to pieces for independent compilation, misses optimisation options, the most optimised is 1.
lto - optimisation of linker, that links codegen-units together. linker-plugin-lto allows to optimise across FFI
There are options that help for debugging - overflow-checks, debug-assertions
`[profile.*.panic]` - what to do during panic
unwind - unwinds stack and forcefully returns function after panic, allows to cleanup resources after panic, misses some optimisations, increases binary size. abort and immediate_abort to exit immediately without doing anything

Can override profile settings for dependencies.

# Conditional compilation
Using `#[cfg()]` can annotate items in code, that will compile if condition is satisfied.  Can use logical operators any, all Options include:
- Feature options - only if feature
- OS options - target_family with values macos, windows, linux and short unix and window
- Context options - for example test profile, doc, doctest profile
- Tool options - for example `#[cfg_attr(miri, ignore)]`
- Architecture options - target_arch with values x86, mips, aarch64; target_feature with values avs, sse2, target_endian, target_poiner_width
- Compiler options - target_env with values gnu, msvc, musl

Can this options for optional dependencies
```
[target.'cfg(windows)'.dependencies]
winrt = "0.7"
[target.'cfg(unix)'.dependencies]
nix = "0.17"
```

Recommend to set CI to audit dependencies using cargo-deny and cargo-audit
Can pass options to compiler with --cfg=myoption, useful for running in different environments
# Versioning
Semantic versioning
Rust specification of versioning - RFC 1105

## Minimum supported rust version
Users in enterprise usually constrained to some older version of rust. To satisfy them, you should update your minimum supported rust version only if really necessary.

You can introduce MSRV policy, that guarantees something. For example - crate will always compile with any stable release of rust from last 6 or 12 months. 

If MSRV changes, you should introduce new minor package version. You may consider it breaking and change major version.

https://github.com/foresterre/cargo-msrv - check MSRV auutomatically
https://rust-for-linux.com/rust-version-policy - linux policy
## Minimal dependency versions
You should keep your required package version as low as possible. Can use cargo -Zminimal-versions flag to check, if crate compiles with specified minimal versions and increase them, if not
## Changelogs
Keep manual changelog for users to find out, what has changed. Recommended format is - https://keepachangelog.com/
## Unreleased versions
Some users may rely on your git version instead of published version. Keep version in your git repository as -alpha.1

