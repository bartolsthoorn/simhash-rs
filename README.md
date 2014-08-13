# Simhash for Rust
![travis-ci status](https://travis-ci.org/bartolsthoorn/rust-simhash.svg?branch=master)

Simhash algorithm (developed by [Moses Charikar](http://www.cs.princeton.edu/courses/archive/spring04/cos598B/bib/CharikarEstim.pdf)) implemented in Rust. It generates 64 bit simhashes and can calculate similarities of two subjects using the [Hamming distance](http://en.wikipedia.org/wiki/Hamming_distance).

~~~rust
extern crate simhash;

// use simhash::hash("This is an example sentence") to simhash a sentence
// use simhash::similarity("Sentence A", "Sentence B") to get a 0.0 - 1.0 similarity score

// check src/lib.rs for hamming_distance and hash_similarity functions if you need those
~~~
