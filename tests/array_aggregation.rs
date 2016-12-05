#[macro_use]
extern crate brassfibre;
use brassfibre::*;

#[test]
fn test_aggregation_int64() {
    let arr = Array::Int64Array(vec![1, 2, 3]);
    assert_eq!(arr.sum(), 6.);
    assert_eq!(arr.count(), 3);
    assert_eq!(arr.mean(), 2.);
    assert_eq!(arr.var(), 0.6666666666666666);
    assert_eq!(arr.unbiased_var(), 1.);
    assert_eq!(arr.std(), 0.816496580927726);
    assert_eq!(arr.unbiased_std(), 1.);
}

#[test]
fn test_aggregation_float64() {
    let arr = Array::Float64Array(vec![1.0, 2.0, 3.0]);
    assert_eq!(arr.sum(), 6.);
    assert_eq!(arr.count(), 3);

    assert_eq!(arr.sum(), 6.);
    assert_eq!(arr.count(), 3);
    assert_eq!(arr.mean(), 2.);
    assert_eq!(arr.var(), 0.6666666666666666);
    assert_eq!(arr.unbiased_var(), 1.);
    assert_eq!(arr.std(), 0.816496580927726);
    assert_eq!(arr.unbiased_std(), 1.);
}

#[test]
#[should_panic]
fn test_aggregation_should_panic() {
    let arr = Array::StringArray(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    arr.sum();
}