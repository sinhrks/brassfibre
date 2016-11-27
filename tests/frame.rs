#[macro_use]
extern crate brassfibre;
use brassfibre::*;


#[test]
fn test_block_creation_from_vec() {
    let values = vec![Array::Int64Array(vec![1, 2, 3, 4, 5]),
                      Array::Float64Array(vec![6., 7., 8., 9., 10.]),
                      Array::Int64Array(vec![11, 12, 13, 14, 15])];
    let df = DataFrame::from_vec(values,
                                 vec!["A", "BB", "CC", "D", "EEE"],
                                 vec!["X", "YYY", "ZZ"]);
    assert_eq!(df.len(), 5);
}

#[test]
fn test_block_add_columns() {
    let values = vec![Array::Int64Array(vec![1, 2, 3]),
                      Array::Float64Array(vec![4., 5., 6.])];
    let mut df = DataFrame::from_vec(values,
                                     vec!["A", "BB", "CC"],
                                     vec!["X", "Y"]);
    assert_eq!(df.len(), 3);
    df.insert(Array::Int64Array(vec![10, 11, 12]), "Z");

    let exp_values = vec![Array::Int64Array(vec![1, 2, 3]),
                          Array::Float64Array(vec![4., 5., 6.]),
                          Array::Int64Array(vec![10, 11, 12])];
    let exp = DataFrame::from_vec(exp_values,
                                  vec!["A", "BB", "CC"],
                                  vec!["X", "Y", "Z"]);
    assert_eq!(df.values, exp.values);
    assert_eq!(df.index, exp.index);
    assert_eq!(df.columns, exp.columns);
}

#[test]
fn test_block_slice_locs() {
    let values = vec![Array::Int64Array(vec![1, 2, 3, 4, 5]),
                      Array::Float64Array(vec![6., 7., 8., 9., 10.]),
                      Array::Int64Array(vec![11, 12, 13, 14, 15])];
    let df = DataFrame::from_vec(values,
                                 vec!["A", "BB", "CC", "D", "EEE"],
                                 vec!["X", "YYY", "ZZ"]);
    assert_eq!(df.len(), 5);

    let res = df.locs(&vec!["A", "D", "CC"]);
    let exp_values = vec![Array::Int64Array(vec![1, 4, 3]),
                          Array::Float64Array(vec![6., 9., 8.]),
                          Array::Int64Array(vec![11, 14, 13])];
    let exp = DataFrame::from_vec(exp_values,
                                  vec!["A", "D", "CC"],
                                  vec!["X", "YYY", "ZZ"]);
    assert_eq!(res.values, exp.values);
    assert_eq!(res.index, exp.index);
    assert_eq!(res.columns, exp.columns);
}

#[test]
fn test_block_slice_ilocs() {
    let values = vec![Array::Int64Array(vec![1, 2, 3, 4, 5]),
                      Array::Float64Array(vec![6., 7., 8., 9., 10.]),
                      Array::Int64Array(vec![11, 12, 13, 14, 15])];
    let df = DataFrame::from_vec(values,
                                 vec!["A", "BB", "CC", "D", "EEE"],
                                 vec!["X", "YYY", "ZZ"]);
    assert_eq!(df.len(), 5);

    let res = df.ilocs(&vec![0, 3, 2]);
    let exp_values = vec![Array::Int64Array(vec![1, 4, 3]),
                          Array::Float64Array(vec![6., 9., 8.]),
                          Array::Int64Array(vec![11, 14, 13])];
    let exp = DataFrame::from_vec(exp_values,
                                  vec!["A", "D", "CC"],
                                  vec!["X", "YYY", "ZZ"]);
    assert_eq!(res.values, exp.values);
    assert_eq!(res.index, exp.index);
    assert_eq!(res.columns, exp.columns);
}

#[test]
fn test_block_columns_slice() {
    let values = vec![Array::Int64Array(vec![1, 2, 3, 4, 5]),
                      Array::Float64Array(vec![6., 7., 8., 9., 10.]),
                      Array::Int64Array(vec![11, 12, 13, 14, 15])];
    let b = DataFrame::from_vec(values,
                                vec!["A", "BB", "CC", "D", "EEE"],
                                vec!["X", "YYY", "ZZ"]);

    let exp_values = vec![Array::Float64Array(vec![6., 7., 8., 9., 10.]),
                          Array::Int64Array(vec![1, 2, 3, 4, 5])];
    let exp = DataFrame::from_vec(exp_values,
                                  vec!["A", "BB", "CC", "D", "EEE"],
                                  vec!["YYY", "X"]);
    let res = b.gets(&vec!["YYY", "X"]);
    assert_eq!(res.values, exp.values);
    assert_eq!(res.index, exp.index);
    assert_eq!(res.columns, exp.columns);

    let res = b.igets(&vec![1, 0]);
    assert_eq!(res.values, exp.values);
    assert_eq!(res.index, exp.index);
    assert_eq!(res.columns, exp.columns);
}