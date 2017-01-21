use std::borrow::Cow;

#[macro_use]
extern crate brassfibre;
use brassfibre::prelude::*;

#[test]
fn test_frame_creation_from_vec() {
    let values =
        vec![array![1, 2, 3, 4, 5], array![6., 7., 8., 9., 10.], array![11, 12, 13, 14, 15]];
    let df = DataFrame::from_vec(values,
                                 vec!["A", "BB", "CC", "D", "EEE"],
                                 vec!["X", "YYY", "ZZ"]);
    assert_eq!(df.len(), 5);
}

#[test]
fn test_frame_add_columns() {
    let values = vec![array![1, 2, 3], array![4., 5., 6.]];
    let mut df = DataFrame::from_vec(values, vec!["A", "BB", "CC"], vec!["X", "Y"]);
    assert_eq!(df.len(), 3);
    df.insert(array![10, 11, 12], "Z");

    let exp_values = vec![array![1, 2, 3], array![4., 5., 6.], array![10, 11, 12]];
    let exp = DataFrame::from_vec(exp_values, vec!["A", "BB", "CC"], vec!["X", "Y", "Z"]);
    assert_eq!(df.values, exp.values);
    assert_eq!(df.index, exp.index);
    assert_eq!(df.columns, exp.columns);
}

#[test]
fn test_frame_slice_locs() {
    let values = vec![array![1i64, 2, 3, 4, 5],
                      array![6.0f64, 7., 8., 9., 10.],
                      array![11i64, 12, 13, 14, 15]];
    let df = DataFrame::from_vec(values,
                                 vec!["A", "BB", "CC", "D", "EEE"],
                                 vec!["X", "YYY", "ZZ"]);
    assert_eq!(df.len(), 5);

    let res = df.locs(&vec!["A", "D", "CC"]);
    let exp_values = vec![array![1i64, 4, 3], array![6.0f64, 9., 8.], array![11i64, 14, 13]];
    let exp = DataFrame::from_vec(exp_values, vec!["A", "D", "CC"], vec!["X", "YYY", "ZZ"]);
    assert_eq!(res.values, exp.values);
    assert_eq!(res.index, exp.index);
    assert_eq!(res.columns, exp.columns);
}

#[test]
#[should_panic]
fn test_frame_slice_locs_panic() {
    let values = vec![array![1i64, 2, 3, 4, 5],
                      array![6.0f64, 7., 8., 9., 10.],
                      array![11i64, 12, 13, 14, 15]];
    let df = DataFrame::from_vec(values,
                                 vec!["A", "BB", "CC", "D", "EEE"],
                                 vec!["X", "YYY", "ZZ"]);

    df.locs(&vec!["A", "D", "X"]);
}

#[test]
fn test_frame_slice_ilocs() {
    let values = vec![array![1i64, 2, 3, 4, 5],
                      array![6.0f64, 7., 8., 9., 10.],
                      array![11i64, 12, 13, 14, 15]];
    let df = DataFrame::from_vec(values,
                                 vec!["A", "BB", "CC", "D", "EEE"],
                                 vec!["X", "YYY", "ZZ"]);
    assert_eq!(df.len(), 5);

    let res = df.ilocs(&vec![0, 3, 2]);
    let exp_values = vec![array![1i64, 4, 3], array![6.0f64, 9., 8.], array![11i64, 14, 13]];
    let exp = DataFrame::from_vec(exp_values, vec!["A", "D", "CC"], vec!["X", "YYY", "ZZ"]);
    assert_eq!(res.values, exp.values);
    assert_eq!(res.index, exp.index);
    assert_eq!(res.columns, exp.columns);
}

#[test]
#[should_panic]
fn test_frame_slice_ilocs_panic() {
    let values = vec![array![1i64, 2, 3, 4, 5],
                      array![6.0f64, 7., 8., 9., 10.],
                      array![11i64, 12, 13, 14, 15]];
    let df = DataFrame::from_vec(values,
                                 vec!["A", "BB", "CC", "D", "EEE"],
                                 vec!["X", "YYY", "ZZ"]);
    df.ilocs(&vec![0, 5, 2]);
}

#[test]
fn test_frame_columns_slice() {
    let values = vec![array![1i64, 2, 3, 4, 5],
                      array![6.0f64, 7., 8., 9., 10.],
                      array![11i64, 12, 13, 14, 15]];
    let df = DataFrame::from_vec(values,
                                 vec!["A", "BB", "CC", "D", "EEE"],
                                 vec!["X", "YYY", "ZZ"]);


    let exp_values = vec![array![6.0f64, 7., 8., 9., 10.], array![1i64, 2, 3, 4, 5]];
    let exp = DataFrame::from_vec(exp_values,
                                  vec!["A", "BB", "CC", "D", "EEE"],
                                  vec!["YYY", "X"]);
    let res = df.gets(&vec!["YYY", "X"]);
    assert_eq!(res.values, exp.values);
    assert_eq!(res.index, exp.index);
    assert_eq!(res.columns, exp.columns);

    let res = df.igets(&vec![1, 0]);
    assert_eq!(res.values, exp.values);
    assert_eq!(res.index, exp.index);
    assert_eq!(res.columns, exp.columns);
}


#[test]
fn test_frame_into_iter() {
    let values = vec![array![1i64, 2, 3], array![6.0f64, 7., 8.]];
    let df = DataFrame::from_vec(values, vec!["A", "BB", "CC"], vec!["X", "YYY"]);
    let mut it = df.into_iter();
    assert_eq!(it.next(), Some(Cow::Owned(array![1i64, 2, 3])));
    assert_eq!(it.next(), Some(Cow::Owned(array![6.0f64, 7., 8.])));
    assert_eq!(it.next(), None);
}

#[test]
fn test_frame_iter() {
    let values = vec![array![1i64, 2, 3], array![6.0f64, 7., 8.]];
    let df = DataFrame::from_vec(values, vec!["A", "BB", "CC"], vec!["X", "YYY"]);
    let mut it = df.iter();
    assert_eq!(it.next(), Some(&Cow::Owned(array![1i64, 2, 3])));
    assert_eq!(it.next(), Some(&Cow::Owned(array![6.0f64, 7., 8.])));
    assert_eq!(it.next(), None);
}

#[test]
fn test_frame_properties() {
    let values: Vec<Array> = vec![array!["a".to_string(),
                                         "b".to_string(),
                                         "c".to_string(),
                                         "d".to_string(),
                                         "e".to_string()],
                                  array![1, 2, 3, 4, 5],
                                  array![true, false, true, false, true],
                                  array![6., 7., 8., 9., 10.]];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let columns: Vec<&str> = vec!["A", "B", "C", "D"];
    let df = DataFrame::from_vec(values, index, columns);

    let exp: Vec<String> =
        vec!["str".to_string(), "i32".to_string(), "bool".to_string(), "f64".to_string()];
    assert_eq!(df.dtypes(), exp);

    let exp: Vec<bool> = vec![false, true, false, true];
    assert_eq!(df.is_numeric(), exp);
}
