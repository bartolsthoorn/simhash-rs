# Simhash for Rust
![travis-ci status](https://travis-ci.org/bartolsthoorn/rust-simhash.svg?branch=master)

Simhash algorithm (developed by [Moses Charikar](http://www.cs.princeton.edu/courses/archive/spring04/cos598B/bib/CharikarEstim.pdf)) implemented in Rust

~~~rust
extern crate simhash;
// now use simhash::hash
~~~

### TODO
- Calculate hamming distance / similarity
- (Optional) Support 32 bit hashing instead of 64
