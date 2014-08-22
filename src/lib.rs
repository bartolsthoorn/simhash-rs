// Rust Simhash
// Implemented by Bart Olsthoorn on 12/08/2014
// With the help of http://matpalm.com/resemblance/simhash/

#![license = "MIT"]

use std::hash;

/// Convert string into `u64` simhash
pub fn hash(subject: &str) -> u64 {
  let mut v: [int, ..64]  = [0, ..64];
  let mut simhash: u64 = 0;

  for feature in subject.words() {
    let feature_hash: u64 = hash::hash(&feature);

    for i in range(0, 64) {
      let bit = (feature_hash >> i) & 1;
      if bit == 1 {
        v[i] = v[i] + 1;
      } else {
        v[i] = v[i] - 1;
      }
    }
  }

  for q in range(0, 64) {
    if v[q] > 0 {
      simhash |= 1 << q;
    }
  }
  simhash
}

/// Bitwise hamming distance of two `u64` hashes
pub fn hamming_distance(x: u64, y: u64) -> u64 {
  (x ^ y).count_ones()
}

/// Calculate similarity as `f64` of two hashes
/// 0.0 means no similarity, 1.0 means identical
pub fn hash_similarity(hash1: u64, hash2: u64) -> f64 {
  let distance: f64 = hamming_distance(hash1, hash2) as f64;
  1.0 - (distance / 64.0)
}

/// Calculate similarity of two string slices by simhash
pub fn similarity(x: &str, y: &str) -> f64 {
  hash_similarity(hash(x), hash(y))
}

#[test]
fn simhash_test() {
  assert_eq!(hash("The cat sat on the mat"), 2595200813813010837);
  assert_eq!(hash("The cat sat under the mat"), 2595269945604666783);
  assert_eq!(hash("Why the lucky stiff"), 1155526875459215761);
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
  assert!(similarity("Hocus pocus", "Hocus pocus pilatus pas") > 0.9);
  assert!(similarity("Peanut butter", "Strawberry cocktail") < 0.6);
}
