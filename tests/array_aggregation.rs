#[macro_use]
extern crate brassfibre;
use brassfibre::prelude::*;

#[test]
fn test_aggregation_int64() {
    let arr = array![1i64, 2, 3];
    assert_eq!(arr.sum(), Scalar::i64(6));
    assert_eq!(arr.count(), 3);
    assert_eq!(arr.mean(), Nullable::new(2.0f64));
    assert_eq!(arr.var(), Nullable::new(0.6666666666666666f64));
    assert_eq!(arr.unbiased_var(), Nullable::new(1.0f64));
    assert_eq!(arr.std(), Nullable::new(0.816496580927726f64));
    assert_eq!(arr.unbiased_std(), Nullable::new(1.0f64));

    assert_eq!(arr.min(), Scalar::i64(1));
    assert_eq!(arr.max(), Scalar::i64(3));
}

#[test]
fn test_aggregation_float64() {
    let arr = array![1.0f64, 2.0, 3.0];
    assert_eq!(arr.sum(), Scalar::f64(6.));
    assert_eq!(arr.count(), 3);

    assert_eq!(arr.mean(), Nullable::new(2.0f64));
    assert_eq!(arr.var(), Nullable::new(0.6666666666666666f64));
    assert_eq!(arr.unbiased_var(), Nullable::new(1.0f64));
    assert_eq!(arr.std(), Nullable::new(0.816496580927726f64));
    assert_eq!(arr.unbiased_std(), Nullable::new(1.0f64));

    assert_eq!(arr.min(), Scalar::f64(1.));
    assert_eq!(arr.max(), Scalar::f64(3.));
}

#[test]
#[should_panic]
fn test_aggregation_sum_should_panic() {
    let arr = array!["a".to_string(), "b".to_string(), "c".to_string()];
    arr.sum();
}

#[test]
#[should_panic]
fn test_aggregation_mean_should_panic() {
    let arr = array!["a".to_string(), "b".to_string(), "c".to_string()];
    arr.mean();
}

#[test]
#[should_panic]
fn test_aggregation_min_should_panic() {
    let arr = array!["a".to_string(), "b".to_string(), "c".to_string()];
    arr.min();
}
