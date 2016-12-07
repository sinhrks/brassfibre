#[macro_use]
extern crate brassfibre;
use brassfibre::*;

#[test]
fn test_creation() {
    let i = Scalar::i64(1);
    assert_eq!(i.dtype(), "i64");
}

#[test]
fn test_clone() {
    let i = Scalar::i64(1);
    assert_eq!(i, i.clone());
}
