# Simhash for Rust
[![CI](https://github.com/bartolsthoorn/simhash-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/bartolsthoorn/simhash-rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/simhash.svg)](https://crates.io/crates/simhash)

Simhash algorithm (developed by [Moses Charikar](http://www.cs.princeton.edu/courses/archive/spring04/cos598B/bib/CharikarEstim.pdf)) implemented in Rust. It generates 64-bit simhashes and can calculate similarities using the [Hamming distance](http://en.wikipedia.org/wiki/Hamming_distance).

## Usage

Add `simhash` to your `Cargo.toml`:
```toml
[dependencies]
simhash = "0.3"
```

```rust
fn main() {
    let hash = simhash::simhash("The cat sat on the mat");
    println!("{hash}");

    let similarity = simhash::similarity("The cat sat on the mat", "The cat sat under the mat");
    println!("{similarity}");
}
```

## Features

By default, simhash uses [SipHash](https://crates.io/crates/siphasher) as its internal hasher. You can switch to [FNV](https://crates.io/crates/fnv) instead:
```toml
[dependencies]
simhash = { version = "0.3", default-features = false, features = ["hasher-fnv"] }
```

## License

MIT
