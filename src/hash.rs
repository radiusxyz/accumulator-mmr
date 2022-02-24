use crate::Merge;
use blake2b_rs::{Blake2b, Blake2bBuilder};
use bytes::Bytes;

fn new_blake2b() -> Blake2b {
  Blake2bBuilder::new(32).build()
}

#[derive(Eq, PartialEq, Clone, Debug, Default)]
pub struct StringHash(pub Bytes);

impl From<String> for StringHash {
  fn from(input_string: String) -> Self {
    let mut hasher = new_blake2b();
    let mut hash = [0u8; 32];
    hasher.update(&Bytes::from(input_string));
    hasher.finalize(&mut hash);
    StringHash(hash.to_vec().into())
  }
}

#[derive(Debug)]
pub struct MergeStringHash;

impl Merge for MergeStringHash {
  type Item = StringHash;
  fn merge(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
    let mut hasher = new_blake2b();
    let mut hash = [0u8; 32];
    hasher.update(&lhs.0);
    hasher.update(&rhs.0);
    hasher.finalize(&mut hash);
    StringHash(hash.to_vec().into())
  }
}
