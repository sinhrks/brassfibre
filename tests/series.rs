use std::borrow::Cow;

extern crate brassfibre;
use brassfibre::*;

#[test]
fn test_series_creation_from_vec() {
    let values: Vec<f64> = vec![1., 2., 3.];

    let s = Series::<f64, i64>::from_vec(values);

    let exp_values: Vec<f64> = vec![1., 2., 3.];
    let exp_index: Indexer<usize> = Indexer::new(vec![0, 1, 2]);
    assert_eq!(s.values, exp_values);
    assert_eq!(s.index, Cow::Owned(exp_index));

    assert_eq!(s.len(), 3);
    assert_eq!(s.index.len(), 3);
}

#[test]
fn test_series_creation_from_index() {
    let values: Vec<f64> = vec![1., 2., 3.];
    let index: Vec<i64> = vec![5, 6, 7];

    let s = Series::<f64, i64>::new(values, index);

    let exp_values: Vec<f64> = vec![1., 2., 3.];
    let exp_index: Indexer<i64> = Indexer::new(vec![5, 6, 7]);
    assert_eq!(s.values, exp_values);
    assert_eq!(s.index, Cow::Owned(exp_index));

    assert_eq!(s.len(), 3);
    assert_eq!(s.index.len(), 3);
}

#[test]
fn test_series_creation_from_into_index() {
    let values: Vec<f64> = vec![1., 2., 3.];
    let index: Indexer<i64> = Indexer::new(vec![5, 6, 7]);

    let s = Series::<f64, i64>::new(values, index);

    let exp_values: Vec<f64> = vec![1., 2., 3.];
    let exp_index: Indexer<i64> = Indexer::new(vec![5, 6, 7]);
    assert_eq!(s.values, exp_values);
    assert_eq!(s.index, Cow::Owned(exp_index));

    assert_eq!(s.len(), 3);
    assert_eq!(s.index.len(), 3);
}

#[test]
fn test_series_copy() {
    let values: Vec<f64> = vec![1., 2., 3.];
    let index: Vec<i64> = vec![5, 6, 7];

    let s = Series::<f64, i64>::new(values, index);
    let copied = s.clone();

    let exp_values: Vec<f64> = vec![1., 2., 3.];
    let exp_index: Indexer<i64> = Indexer::new(vec![5, 6, 7]);
    assert_eq!(copied.values, exp_values);
    assert_eq!(copied.index, Cow::Owned(exp_index));

    assert_eq!(copied, s);
}

#[test]
fn test_series_equals() {
    let s1 = Series::<f64, i64>::new(vec![1., 2., 3.], vec![5, 6, 7]);
    let s2 = Series::<f64, i64>::new(vec![1., 2., 3.], vec![9, 6, 7]);;
    let s3 = Series::<f64, i64>::new(vec![1., 2., 3.], vec![5, 6, 7]);;
    let s4 = Series::<f64, i64>::new(vec![1., 2., 4.], vec![5, 6, 7]);;

    assert_eq!(s1 == s2, false);
    assert_eq!(s1 == s3, true);
    assert_eq!(s1 == s4, false);
}

#[test]
fn test_series_slice_locs() {
    let values: Vec<f64> = vec![1., 2., 3., 4., 5.];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];

    let s = Series::new(values, index);

    // test construction
    let exp_values: Vec<f64> = vec![1., 2., 3., 4., 5.];
    let exp_index: Indexer<i64> = Indexer::new(vec![10, 20, 30, 40, 50]);
    assert_eq!(s.values, exp_values);
    assert_eq!(s.index, Cow::Owned(exp_index));

    // test label slice
    let res = s.locs(&vec![20, 30, 50]);
    let exp: Series<f64, i64> = Series::new(vec![2., 3., 5.], vec![20, 30, 50]);
    assert_eq!(res, exp);
}

#[test]
fn test_series_slice_ilocs() {
    let values: Vec<f64> = vec![1., 2., 3., 4., 5.];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];

    let s = Series::<f64, i64>::new(values, index);

    // test construction
    let exp_values: Vec<f64> = vec![1., 2., 3., 4., 5.];
    let exp_index: Indexer<i64> = Indexer::new(vec![10, 20, 30, 40, 50]);
    assert_eq!(s.values, exp_values);
    assert_eq!(s.index, Cow::Owned(exp_index));

    // test index slice
    let res = s.ilocs(&vec![0, 2, 4]);

    let exp: Series<f64, i64> = Series::new(vec![1., 3., 5.], vec![10, 30, 50]);
    assert_eq!(res, exp);
}

#[test]
fn test_series_slice_blocs() {
    let values: Vec<f64> = vec![1., 2., 3., 4., 5.];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];

    let s = Series::<f64, i64>::new(values, index);

    // test construction
    let exp_values: Vec<f64> = vec![1., 2., 3., 4., 5.];
    let exp_index: Indexer<i64> = Indexer::new(vec![10, 20, 30, 40, 50]);
    assert_eq!(s.values, exp_values);
    assert_eq!(s.index, Cow::Owned(exp_index));

    // test bool slice
    let res = s.blocs(&vec![true, false, false, true, true]);

    let exp: Series<f64, i64> = Series::new(vec![1., 4., 5.], vec![10, 40, 50]);
    assert_eq!(res, exp);
}

#[test]
fn test_series_reindex() {
    let s: Series<&str, &str> = Series::new(vec!["a", "b", "c", "d"],
                                                vec!["A", "B", "C", "D"]);
    let res = s.reindex(&vec!["D", "C", "A"]);
    let exp: Series<&str, &str> = Series::new(vec!["d", "c", "a"],
                                              vec!["D", "C", "A"]);

    assert_eq!(res, exp);
}

#[test]
fn test_series_reindex_by_index() {
    let s: Series<&str, &str> = Series::new(vec!["a", "b", "c", "d"],
                                            vec!["A", "B", "C", "D"]);
    let res = s.reindex_by_index(&vec![1, 3, 0]);
    let exp: Series<&str, &str> = Series::new(vec!["b", "d", "a"],
                                              vec!["B", "D", "A"]);

    assert_eq!(res, exp);
}

#[test]
fn test_series_append() {
    let values: Vec<f64> = vec![1., 2., 3., 4., 5.];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];

    let s1 = Series::new(values, index);

    let values: Vec<f64> = vec![11., 12., 13., 14., 15.];
    let index: Vec<i64> = vec![110, 120, 130, 140, 150];

    let s2 = Series::new(values, index);

    let res = s1.append(&s2);
    let exp: Series<f64, i64> = Series::new(vec![1., 2., 3., 4., 5., 11., 12., 13., 14., 15.],
                                            vec![10, 20, 30, 40, 50, 110, 120, 130, 140, 150]);
    assert_eq!(res, exp);
}

#[test]
fn test_series_into_iter() {
    let s: Series<i64, i64> = Series::new(vec![1, 2, 3], vec![10, 20, 30]);
    let mut it = s.into_iter();
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), Some(3));
    assert_eq!(it.next(), None);
}

#[test]
fn test_series_iter() {
    let s: Series<i64, i64> = Series::new(vec![1, 2, 3], vec![10, 20, 30]);
    let mut it = s.iter();
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), None);
}