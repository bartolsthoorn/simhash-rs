/**
Rust Simhash
Implemented by Bart Olsthoorn on 12/08/2014
Ported to Rust 1.16.0 by Jakub Pastuszek on 29/05/2017
With the help of http://matpalm.com/resemblance/simhash/
*/
use std::hash::{Hash, Hasher};

// Note: stdlib no longer exposes SipHasher directly
#[cfg(all(feature = "hasher-sip", not(feature = "hasher-fnv")))]
fn hash_feature<T: Hash>(t: &T) -> u64 {
    use siphasher::sip::SipHasher;
    let mut s = SipHasher::default();
    t.hash(&mut s);
    s.finish()
}

#[cfg(all(feature = "hasher-fnv", not(feature = "hasher-sip")))]
fn hash_feature<T: Hash>(t: &T) -> u64 {
    use fnv::FnvHasher;
    let mut s = FnvHasher::default();
    t.hash(&mut s);
    s.finish()
}

/// Calculate `u64` simhash from stream of `&str` words
pub fn simhash_stream<'w, W>(words: W) -> u64
where
    W: Iterator<Item = &'w str>,
{
    let mut v = [0i32; 64];
    let mut simhash: u64 = 0;

    for feature in words {
        let feature_hash: u64 = hash_feature(&feature);

        for i in 0..64 {
            let bit = (feature_hash >> i) & 1;
            if bit == 1 {
                v[i] = v[i].saturating_add(1);
            } else {
                v[i] = v[i].saturating_sub(1);
            }
        }
    }

    for q in 0..64 {
        if v[q] > 0 {
            simhash |= 1 << q;
        }
    }
    simhash
}

/// Calculate `u64` simhash from `&str` split by whitespace
pub fn simhash(text: &str) -> u64 {
    simhash_stream(text.split_whitespace())
}

/// Calculate `u64` simhash from `&str` split by whitespace
#[deprecated(since = "0.2.0", note = "please use `simhash` instead")]
pub fn hash(text: &str) -> u64 {
    simhash_stream(text.split_whitespace())
}

/// Bitwise hamming distance of two `u64` hashes
pub fn hamming_distance(x: u64, y: u64) -> u32 {
    (x ^ y).count_ones()
}

/// Calculate similarity as `f64` of two hashes
/// 0.0 means no similarity, 1.0 means identical
pub fn hash_similarity(hash1: u64, hash2: u64) -> f64 {
    let distance: f64 = hamming_distance(hash1, hash2) as f64;
    1.0 - (distance / 64.0)
}

/// Calculate similarity of two streams of string slices by simhash
pub fn similarity_streams<'w1, 'w2, W1, W2>(words1: W1, words2: W2) -> f64
where
    W1: Iterator<Item = &'w1 str>,
    W2: Iterator<Item = &'w2 str>,
{
    hash_similarity(simhash_stream(words1), simhash_stream(words2))
}

/// Calculate similarity of two string slices split by whitespace by simhash
pub fn similarity(text1: &str, text2: &str) -> f64 {
    similarity_streams(text1.split_whitespace(), text2.split_whitespace())
}

#[test]
#[cfg(all(feature = "hasher-sip", not(feature = "hasher-fnv")))]
fn simhash_test() {
    assert_eq!(simhash("The cat sat on the mat"), 2595200813813010837);
    assert_eq!(simhash("The cat sat under the mat"), 2595269945604666783);
    assert_eq!(simhash("Why the lucky stiff"), 1155526875459215761);
}

#[test]
#[cfg(all(feature = "hasher-fnv", not(feature = "hasher-sip")))]
fn simhash_test() {
    assert_eq!(simhash("The cat sat on the mat"), 1245844084707839512);
    assert_eq!(simhash("The cat sat under the mat"), 102053001967980056);
    assert_eq!(simhash("Why the lucky stiff"), 369356755250006160);
}

#[test]
fn hamming_distance_test() {
    assert_eq!(hamming_distance(0b0000000u64, 0b0000000u64), 0);
    assert_eq!(hamming_distance(0b1111111u64, 0b0000000u64), 7);
    assert_eq!(hamming_distance(0b0100101u64, 0b1100110u64), 3);
}

#[test]
fn hash_similarity_test() {
    assert_eq!(hash_similarity(0u64, 0u64), 1.0);
    assert_eq!(hash_similarity(!0u64, 0u64), 0.0);
    assert_eq!(hash_similarity(!0u32 as u64, 0u64), 0.5);
}

#[test]
fn similarity_test() {
    assert_eq!(similarity("Stop hammertime", "Stop hammertime"), 1.0);
    assert!(similarity("Hocus pocus", "Hocus pocus pilatus pas") > 0.7);
    assert!(similarity("Peanut butter", "Strawberry cocktail") < 0.6);
}
