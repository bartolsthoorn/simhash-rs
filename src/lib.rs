// Rust Simhash
// Implemented by Bart Olsthoorn on 12/08/2014
// With the help of http://matpalm.com/resemblance/simhash/

#![license = "MIT"]

pub mod simhash {
  use std::hash;

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
}

#[test]
fn simhash_test() {
  assert!(simhash::hash("The cat sat on the mat") == 2595200813813010837);
  assert!(simhash::hash("Why the lucky stiff") == 1155526875459215761);
}
