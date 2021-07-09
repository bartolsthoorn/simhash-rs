# Simhash for Rust
[![Build Status](https://travis-ci.org/bartolsthoorn/simhash-rs.svg?branch=master)](https://travis-ci.org/bartolsthoorn/simhash-rs)

Simhash algorithm (developed by [Moses Charikar](http://www.cs.princeton.edu/courses/archive/spring04/cos598B/bib/CharikarEstim.pdf)) implemented in Rust. It generates 64 bit simhashes and can calculate similarities using the [Hamming distance](http://en.wikipedia.org/wiki/Hamming_distance).

To use simhash append the following lines to your `Cargo.toml` file.
```toml
[dependencies]
simhash = "0.3.0"
```

You can now use it in your project.
```rust
fn main() {
  let h: u64 = simhash::hash("The cat sat on the mat");
  println!("{}", h);
}
```
