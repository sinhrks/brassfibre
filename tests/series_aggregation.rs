extern crate brassfibre;
use brassfibre::*;

#[test]
fn test_series_agg_int() {
    let values: Vec<i64> = vec![1, 2, 3, 4, 5];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];

    let s = Series::<i64, i64>::new(values, index);

    assert_eq!(s.sum(), 15);
    assert_eq!(s.min(), 1);
    assert_eq!(s.max(), 5);
    assert_eq!(s.count(), 5);
    assert_eq!(s.mean(), 3.0);
    assert_eq!(s.var(), 2.0);
    assert_eq!(s.unbiased_var(), 2.5);

    let values: Vec<i64> = vec![2, 2, 2, 3, 3];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];

    let s = Series::<i64, i64>::new(values, index);
    assert_eq!(s.mean(), 2.4);

    let values: Vec<i64> = vec![11, 12, 11, 14, 12];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let s = Series::<i64, i64>::new(values, index);

    assert_eq!(s.var(), 1.2);
    assert_eq!(s.unbiased_var(), 1.5);

    assert_eq!(s.std(), 1.0954451150103321);
    assert_eq!(s.unbiased_std(), 1.2247448713915889);
}

#[test]
fn test_series_agg_float() {
    let values: Vec<f64> = vec![1., 2., 3., 4., 5.];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let s = Series::<f64, i64>::new(values, index);

    assert_eq!(s.sum(), 15.);
    assert_eq!(s.min(), 1.);
    assert_eq!(s.max(), 5.);
    assert_eq!(s.count(), 5);
    assert_eq!(s.mean(), 3.);
    assert_eq!(s.var(), 2.0);
    assert_eq!(s.unbiased_var(), 2.5);

    let values: Vec<f64> = vec![11., 12., 11., 14., 12.];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let s = Series::<f64, i64>::new(values, index);

    assert_eq!(s.var(), 1.2);
    assert_eq!(s.unbiased_var(), 1.5);

    assert_eq!(s.std(), 1.0954451150103321);
    assert_eq!(s.unbiased_std(), 1.2247448713915889);
}

#[test]
fn test_series_describe_int() {
    let values: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let s = Series::<i64, i64>::from_vec(values);

    let d = s.describe();
    let exp: Series<f64, &str> = Series::new(vec![10., 5.5, 2.8722813232690143, 1., 10.],
                                             vec!["count", "mean", "std", "min", "max"]);
    assert_eq!(d, exp);
}

#[test]
fn test_series_describe_float() {
    let values: Vec<f64> = vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
    let s = Series::<f64, i64>::from_vec(values);

    let d = s.describe();
    let exp: Series<f64, &str> = Series::new(vec![10., 5.5, 2.8722813232690143, 1., 10.],
                                             vec!["count", "mean", "std", "min", "max"]);
    assert_eq!(d, exp);
}

#[test]
fn test_series_value_counts_int() {
    let values: Vec<i64> = vec![1, 1, 3, 4, 2, 1, 1, 2, 3, 3];
    let s = Series::<i64, usize>::from_vec(values);

    let d = s.value_counts();
    let exp: Series<usize, i64> = Series::new(vec![4, 3, 2, 1], vec![1, 3, 2, 4]);
    assert_eq!(d, exp);
}

#[test]
fn test_series_value_counts_str() {
    let values: Vec<&str> = vec!["a", "bb", "bb", "c", "a", "a"];
    let s = Series::<&str, usize>::from_vec(values);

    let d = s.value_counts();
    let exp: Series<usize, &str> = Series::new(vec![3, 2, 1], vec!["a", "bb", "c"]);
    assert_eq!(d, exp);
}
