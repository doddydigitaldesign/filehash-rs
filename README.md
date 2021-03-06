# filehash-rs
A small (3kb), fast library for hashing files written in Rust.

### Usage
See the `examples` directory for some example usages.

### Notes
- The resulting hash may vary depending on the environment

### Dependencies
The crate has some dependencies on the Rust standard library and might not be suited for applications where `no-std` is important.
```rust
use std::{
    collections::hash_map::DefaultHasher,
    fs::File,
    hash::{Hash, Hasher},
    io::{Error, Read},
    path::Path,
};
```