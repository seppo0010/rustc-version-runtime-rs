# rustc-version-runtime-rs

Simple library for getting the version information of a `rustc`
compiler in runtime.

The goal of this crate is to provide debug information to developers,
not to branch application logic based on compiler version. Please, don't
do that.

## Getting Started

rustc-version-runtime-rs is available on
[crates.io](https://crates.io/crates/rustc_version_runtime).
Add the following dependency to your Cargo manifest:

```toml
[dependencies]
rustc_version_runtime = "0.1.*"
```


## Example

```rust
extern crate rustc_version_runtime;
use rustc_version_runtime::version;

println!("This was compiled using {:?}", version());
```
