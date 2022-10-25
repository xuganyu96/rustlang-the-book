1. `std::env::args()` return an iterator `Args` that returns `String` values (note it's not `&String`).

```rust
use std::env;

for arg in env::args() {
    println!("{}", arg);
}
```

2. Recall from [Packages and Crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) that a crate is a single rust source code file, a package is a collection of crate. A package can build to one or more binaries, but only one library
