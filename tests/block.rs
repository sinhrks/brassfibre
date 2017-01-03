use std::borrow::Cow;

extern crate brassfibre;
use brassfibre::prelude::*;

#[test]
fn test_block_creation_from_col_vec() {
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let b = Block::from_col_vec(values,
                                vec!["A", "BB", "CC", "D", "EEE"],
                                vec!["X", "YYY", "ZZ"]);
    assert_eq!(b.len(), 5);

    let exp_index: Indexer<&str> = Indexer::new(vec!["A", "BB", "CC", "D", "EEE"]);
    let exp_columns: Indexer<&str> = Indexer::new(vec!["X", "YYY", "ZZ"]);
    assert_eq!(b.index, Cow::Owned(exp_index));
    assert_eq!(b.columns, Cow::Owned(exp_columns));

    let c = b.get(&"X");
    let exp: Series<i64, &str> = Series::new(vec![1, 2, 3, 4, 5],
                                             vec!["A", "BB", "CC", "D", "EEE"]);
    assert_eq!(c, exp);

    let c = b.get(&"YYY");
    let exp: Series<i64, &str> = Series::new(vec![6, 7, 8, 9, 10],
                                             vec!["A", "BB", "CC", "D", "EEE"]);
    assert_eq!(c, exp);

    let c = b.get(&"ZZ");
    let exp: Series<i64, &str> = Series::new(vec![11, 12, 13, 14, 15],
                                             vec!["A", "BB", "CC", "D", "EEE"]);
    assert_eq!(c, exp);
}

#[test]
fn test_block_creation_from_row_vec() {
    let values = vec![1, 6, 11, 2, 7, 12, 3, 8, 13, 4, 9, 14, 5, 10, 15];
    let b = Block::from_row_vec(values,
                                vec!["A", "BB", "CC", "D", "EEE"],
                                vec!["X", "YYY", "ZZ"]);
    assert_eq!(b.len(), 5);

    let exp_index: Indexer<&str> = Indexer::new(vec!["A", "BB", "CC", "D", "EEE"]);
    let exp_columns: Indexer<&str> = Indexer::new(vec!["X", "YYY", "ZZ"]);
    assert_eq!(b.index, Cow::Owned(exp_index));
    assert_eq!(b.columns, Cow::Owned(exp_columns));

    let c = b.get(&"X");
    let exp: Series<i64, &str> = Series::new(vec![1, 2, 3, 4, 5],
                                             vec!["A", "BB", "CC", "D", "EEE"]);
    assert_eq!(c, exp);

    let c = b.get(&"YYY");
    let exp: Series<i64, &str> = Series::new(vec![6, 7, 8, 9, 10],
                                             vec!["A", "BB", "CC", "D", "EEE"]);
    assert_eq!(c, exp);

    let c = b.get(&"ZZ");
    let exp: Series<i64, &str> = Series::new(vec![11, 12, 13, 14, 15],
                                             vec!["A", "BB", "CC", "D", "EEE"]);
    assert_eq!(c, exp);
}

#[test]
fn test_block_creation_from_vec() {
    let values = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15]];
    let b = Block::from_vec(values,
                            vec!["A", "BB", "CC", "D", "EEE"],
                            vec!["X", "YYY", "ZZ"]);
    assert_eq!(b.len(), 5);

    let exp_index: Indexer<&str> = Indexer::new(vec!["A", "BB", "CC", "D", "EEE"]);
    let exp_columns: Indexer<&str> = Indexer::new(vec!["X", "YYY", "ZZ"]);
    assert_eq!(b.index, Cow::Owned(exp_index));
    assert_eq!(b.columns, Cow::Owned(exp_columns));

    let c = b.get(&"X");
    let exp: Series<i64, &str> = Series::new(vec![1, 2, 3, 4, 5],
                                             vec!["A", "BB", "CC", "D", "EEE"]);
    assert_eq!(c, exp);

    let c = b.get(&"YYY");
    let exp: Series<i64, &str> = Series::new(vec![6, 7, 8, 9, 10],
                                             vec!["A", "BB", "CC", "D", "EEE"]);
    assert_eq!(c, exp);

    let c = b.get(&"ZZ");
    let exp: Series<i64, &str> = Series::new(vec![11, 12, 13, 14, 15],
                                             vec!["A", "BB", "CC", "D", "EEE"]);
    assert_eq!(c, exp);
}

#[test]
fn test_block_creation_from_nested_vec() {
    let values = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15]];
    let b = Block::from_nested_vec(values,
                                   vec!["A", "BB", "CC", "D", "EEE"],
                                   vec!["X", "YYY", "ZZ"]);
    assert_eq!(b.len(), 5);

    let exp_values = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15]];
    let exp = Block::from_vec(exp_values,
                              vec!["A", "BB", "CC", "D", "EEE"],
                              vec!["X", "YYY", "ZZ"]);
    assert_eq!(b, exp);
}

#[test]
fn test_block_creation_from_series() {
    let values: Vec<f64> = vec![1., 2., 3.];
    let index: Vec<&str> = vec!["A", "B", "C"];
    let s = Series::<f64, &str>::new(values, index);

    let b = Block::<f64, &str, i64>::from_series(s, 1);
    assert_eq!(b.len(), 3);

    let exp_index: Indexer<&str> = Indexer::new(vec!["A", "B", "C"]);
    let exp_columns: Indexer<i64> = Indexer::new(vec![1]);
    assert_eq!(b.index, Cow::Owned(exp_index));
    assert_eq!(b.columns, Cow::Owned(exp_columns));

    let c = b.get(&1);
    let exp: Series<f64, &str> = Series::new(vec![1., 2., 3.], vec!["A", "B", "C"]);
    assert_eq!(c, exp);
}

#[test]
fn test_block_creation_into() {
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let exp = Block::from_col_vec(values,
                                  vec!["A", "BB", "CC", "D", "EEE"],
                                  vec!["X", "YYY", "ZZ"]);

    let index = Indexer::new(vec!["A", "BB", "CC", "D", "EEE"]);
    let columns = Indexer::new(vec!["X", "YYY", "ZZ"]);
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let b = Block::from_col_vec(values, index, columns);
    assert_eq!(b, exp);

    let index = Indexer::new(vec!["A", "BB", "CC", "D", "EEE"]);
    let columns = Indexer::new(vec!["X", "YYY", "ZZ"]);
    let values = vec![1, 6, 11, 2, 7, 12, 3, 8, 13, 4, 9, 14, 5, 10, 15];
    let b = Block::from_row_vec(values, index, columns);
    assert_eq!(b, exp);

    let index = Indexer::new(vec!["A", "BB", "CC", "D", "EEE"]);
    let columns = Indexer::new(vec!["X", "YYY", "ZZ"]);
    let values = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15]];
    let b = Block::from_nested_vec(values, index, columns);
    assert_eq!(b, exp);
}

#[test]
fn test_block_head_tail() {
    let values = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15]];
    let b = Block::from_nested_vec(values,
                                   vec!["A", "BB", "CC", "D", "EEE"],
                                   vec!["X", "YYY", "ZZ"]);


    let exp_values = vec![vec![1, 2, 3], vec![6, 7, 8], vec![11, 12, 13]];
    let exp = Block::from_vec(exp_values, vec!["A", "BB", "CC"], vec!["X", "YYY", "ZZ"]);
    assert_eq!(b.head(3), exp);

    let exp_values = vec![vec![3, 4, 5], vec![8, 9, 10], vec![13, 14, 15]];
    let exp = Block::from_vec(exp_values, vec!["CC", "D", "EEE"], vec!["X", "YYY", "ZZ"]);
    assert_eq!(b.tail(3), exp);
}

#[test]
fn test_block_columns_get() {
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let b = Block::from_col_vec(values,
                                vec!["A", "BB", "CC", "D", "EEE"],
                                vec!["X", "YYY", "ZZ"]);

    let res = b.get(&"YYY");
    let exp: Series<i64, &str> = Series::new(vec![6, 7, 8, 9, 10],
                                             vec!["A", "BB", "CC", "D", "EEE"]);
    assert_eq!(res, exp);

    let res = b.iget(&1);
    let exp: Series<i64, &str> = Series::new(vec![6, 7, 8, 9, 10],
                                             vec!["A", "BB", "CC", "D", "EEE"]);
    assert_eq!(res, exp);
}

#[test]
fn test_block_columns_slice() {
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let b = Block::from_col_vec(values,
                                vec!["A", "BB", "CC", "D", "EEE"],
                                vec!["X", "YYY", "ZZ"]);

    let exp = Block::from_col_vec(vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5],
                                  vec!["A", "BB", "CC", "D", "EEE"],
                                  vec!["YYY", "X"]);
    let res = b.gets(&vec!["YYY", "X"]);
    assert_eq!(res, exp);

    let res = b.igets(&vec![1, 0]);
    assert_eq!(res, exp);
}

#[test]
fn test_insert() {
    let values: Vec<f64> = vec![1., 2., 3.];
    let index: Vec<&str> = vec!["A", "B", "C"];
    let s = Series::<f64, &str>::new(values, index);

    let mut b = Block::<f64, &str, i64>::from_series(s, 1);

    assert_eq!(b.len(), 3);
    let exp_index: Indexer<&str> = Indexer::new(vec!["A", "B", "C"]);
    let exp_columns: Indexer<i64> = Indexer::new(vec![1]);
    assert_eq!(b.index, Cow::Owned(exp_index));
    assert_eq!(b.columns, Cow::Owned(exp_columns));

    // add columns
    let values2: Vec<f64> = vec![4., 5., 6.];
    b.insert(3, values2);
    assert_eq!(b.len(), 3);
    let exp_index: Indexer<&str> = Indexer::new(vec!["A", "B", "C"]);
    let exp_columns: Indexer<i64> = Indexer::new(vec![1, 3]);
    assert_eq!(b.index, Cow::Owned(exp_index));
    assert_eq!(b.columns, Cow::Owned(exp_columns));

    assert_eq!(b.columns.get_loc(&1), 0);
    assert_eq!(b.columns.get_loc(&3), 1);
    let c = b.get(&1);
    let exp: Series<f64, &str> = Series::new(vec![1., 2., 3.], vec!["A", "B", "C"]);
    assert_eq!(c, exp);

    let c = b.get(&3);
    let exp: Series<f64, &str> = Series::new(vec![4., 5., 6.], vec!["A", "B", "C"]);
    assert_eq!(c, exp);
}

#[test]
fn test_slice_ilocs() {
    let values: Vec<f64> = vec![1., 2., 3., 4., 5., 6.];
    let index: Vec<&str> = vec!["A", "B", "C"];
    let b = Block::<f64, &str, i64>::from_col_vec(values, index, vec![1, 3]);
    assert_eq!(b.len(), 3);

    // slice
    let sliced = b.ilocs(&vec![0, 2]);
    let exp =
        Block::<f64, &str, i64>::from_col_vec(vec![1., 3., 4., 6.], vec!["A", "C"], vec![1, 3]);
    assert_eq!(sliced, exp);

    // compare columns
    let c = sliced.get(&1);
    let exp: Series<f64, &str> = Series::new(vec![1., 3.], vec!["A", "C"]);
    assert_eq!(c, exp);
    let c = sliced.get(&3);
    let exp: Series<f64, &str> = Series::new(vec![4., 6.], vec!["A", "C"]);
    assert_eq!(c, exp);
}

#[test]
fn test_slice_locs() {
    let values: Vec<f64> = vec![1., 2., 3., 4., 5., 6.];
    let index: Vec<&str> = vec!["A", "B", "C"];
    let b = Block::<f64, &str, i64>::from_col_vec(values, index, vec![1, 3]);
    assert_eq!(b.len(), 3);

    // slice
    let sliced = b.locs(&vec!["B", "C"]);

    let exp =
        Block::<f64, &str, i64>::from_col_vec(vec![2., 3., 5., 6.], vec!["B", "C"], vec![1, 3]);
    assert_eq!(sliced, exp);

    // compare columns
    let c = sliced.get(&1);
    let exp: Series<f64, &str> = Series::new(vec![2., 3.], vec!["B", "C"]);
    assert_eq!(c, exp);
    let c = sliced.get(&3);
    let exp: Series<f64, &str> = Series::new(vec![5., 6.], vec!["B", "C"]);
    assert_eq!(c, exp);
}

#[test]
fn test_block_reindex() {
    let values = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15]];
    let b = Block::from_nested_vec(values,
                                   vec!["A", "BB", "CC", "D", "EEE"],
                                   vec!["X", "YYY", "ZZ"]);
    let res = b.reindex(&vec!["BB", "D", "A"]);

    let values = vec![vec![2, 4, 1], vec![7, 9, 6], vec![12, 14, 11]];
    let exp = Block::from_nested_vec(values, vec!["BB", "D", "A"], vec!["X", "YYY", "ZZ"]);
    assert_eq!(res, exp);
}

#[test]
#[should_panic]
fn test_block_reindex_panic() {
    let values = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15]];
    let b = Block::from_nested_vec(values,
                                   vec!["A", "BB", "CC", "D", "EEE"],
                                   vec!["X", "YYY", "ZZ"]);
    b.reindex(&vec!["BB", "D", "X"]);
}

#[test]
fn test_block_reindex_by_index() {
    let values = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15]];
    let b = Block::from_nested_vec(values,
                                   vec!["A", "BB", "CC", "D", "EEE"],
                                   vec!["X", "YYY", "ZZ"]);
    let res = b.reindex_by_index(&vec![1, 3, 0]);

    let values = vec![vec![2, 4, 1], vec![7, 9, 6], vec![12, 14, 11]];
    let exp = Block::from_nested_vec(values, vec!["BB", "D", "A"], vec!["X", "YYY", "ZZ"]);
    assert_eq!(res, exp);
}

#[test]
#[should_panic]
fn test_block_reindex_by_index_panic() {
    let values = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15]];
    let b = Block::from_nested_vec(values,
                                   vec!["A", "BB", "CC", "D", "EEE"],
                                   vec!["X", "YYY", "ZZ"]);
    b.reindex_by_index(&vec![1, 3, 5]);
}

#[test]
fn test_block_transpose() {
    let b1 = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                 vec!["A", "B", "C"],
                                 vec!["X", "Y"]);
    let res = b1.transpose();

    let exp = Block::from_row_vec(vec![1., 2., 3., 4., 5., 6.],
                                  vec!["X", "Y"],
                                  vec!["A", "B", "C"]);
    assert_eq!(res, exp);
}

#[test]
fn test_block_into_iter() {
    let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                vec!["A", "B", "C"],
                                vec!["X", "Y"]);
    let mut it = b.into_iter();
    assert_eq!(it.next(), Some(Cow::Owned(vec![1., 2., 3.])));
    assert_eq!(it.next(), Some(Cow::Owned(vec![4., 5., 6.])));
    assert_eq!(it.next(), None);
}

#[test]
fn test_block_iter() {
    let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                vec!["A", "B", "C"],
                                vec!["X", "Y"]);
    let mut it = b.iter();
    assert_eq!(it.next(), Some(&Cow::Owned(vec![1., 2., 3.])));
    assert_eq!(it.next(), Some(&Cow::Owned(vec![4., 5., 6.])));
    assert_eq!(it.next(), None);
}
