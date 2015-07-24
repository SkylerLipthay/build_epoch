# build_epoch

A simple Rust compiler plugin macro for generating an `i64` specifying, in seconds, the time that the target crate was built in relation to epoch (`1970-01-01T00:00:00Z`).

```rust
#![feature(plugin)]
#![plugin(build_epoch)]

fn main() {
    // At the time of writing, this prints "1437740236":
    println!("{}", build_epoch!());
}
```

The static value of `build_epoch!()` is determined when the compiler expands its first occurrence.
