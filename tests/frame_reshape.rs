#[macro_use]
extern crate brassfibre;
use brassfibre::*;

#[test]
fn test_dataframe_append() {
    let values1 = vec![array![1, 2, 3],
                       array![4.1, 5.1, 6.1],
                       array![1, 2, 3]];
    let df1 = DataFrame::from_vec(values1,
                                  vec!["A", "B", "C"],
                                  vec!["X", "Y", "Z"]);

    let values2 = vec![array![4, 5],
                       array![7.1, 8.1],
                       array![4, 5]];
    let df2 = DataFrame::from_vec(values2,
                                  vec!["D", "E"],
                                  vec!["X", "Y", "Z"]);

    let res = df1.append(&df2);

    let exp_values = vec![array![1, 2, 3, 4, 5],
                          array![4.1, 5.1, 6.1, 7.1, 8.1],
                          array![1, 2, 3, 4, 5]];
    let exp = DataFrame::from_vec(exp_values,
                                  vec!["A", "B", "C", "D", "E"],
                                  vec!["X", "Y", "Z"]);
    assert_eq!(res, exp);
}

#[test]
#[should_panic]
fn test_dataframe_append_different_columns() {
    // different columns
    let values1 = vec![array![1, 2, 3],
                       array![4.1, 5.1, 6.1],
                       array![1, 2, 3]];
    let df1 = DataFrame::from_vec(values1,
                                  vec!["A", "B", "C"],
                                  vec!["X", "Y", "Z"]);

    let values2 = vec![array![4, 5],
                       array![7.1, 8.1],
                       array![4, 5]];
    let df2 = DataFrame::from_vec(values2,
                                  vec!["D", "E"],
                                  vec!["XX", "Y", "Z"]);

    df1.append(&df2);
}

#[test]
#[should_panic]
fn test_dataframe_append_different_dtype() {
    // different columns
    let values1 = vec![array![1, 2, 3],
                       array![4.1, 5.1, 6.1],
                       array![1, 2, 3]];
    let df1 = DataFrame::from_vec(values1,
                                  vec!["A", "B", "C"],
                                  vec!["X", "Y", "Z"]);

    let values2 = vec![array![4, 5],
                       array![7.1, 8.1],
                       array![4.1, 5.1]];
    let df2 = DataFrame::from_vec(values2,
                                  vec!["D", "E"],
                                  vec!["X", "Y", "Z"]);

    df1.append(&df2);
}

#[test]
fn test_dataframe_concat() {

    let values1 = vec![array![1, 2, 3],
                       array![4.1, 5.1, 6.1],
                       array![1, 2, 3]];
    let df1 = DataFrame::from_vec(values1,
                                  vec!["A", "B", "C"],
                                  vec!["X", "Y", "Z"]);

    let values2 = vec![array![4, 5, 6],
                       array![7.1, 8.1, 9.1]];
    let df2 = DataFrame::from_vec(values2,
                                  vec!["A", "B", "C"],
                                  vec!["X2", "Y2"]);
    let res = df1.concat(&df2);

    let exp_values = vec![array![1, 2, 3],
                          array![4.1, 5.1, 6.1],
                          array![1, 2, 3],
                          array![4, 5, 6],
                          array![7.1, 8.1, 9.1]];
    let exp = DataFrame::from_vec(exp_values,
                                  vec!["A", "B", "C"],
                                  vec!["X", "Y", "Z", "X2", "Y2"]);
    assert_eq!(res, exp);
}

#[test]
#[should_panic]
fn test_block_concat_panic() {
    // different index
    let values1 = vec![array![1, 2, 3],
                       array![4.1, 5.1, 6.1],
                       array![1, 2, 3]];
    let df1 = DataFrame::from_vec(values1,
                                  vec!["A", "B", "C"],
                                  vec!["X", "Y", "Z"]);

    let values2 = vec![array![4, 5, 6],
                       array![7.1, 8.1, 9.1]];
    let df2 = DataFrame::from_vec(values2,
                                  vec!["A1", "B", "C"],
                                  vec!["X2", "Y2"]);
    df1.concat(&df2);
}

#[test]
fn test_block_join() {
    let values1 = vec![array![1, 2, 3, 4, 5],
                       array![4.1, 5.1, 6.1, 7.1, 8.1],
                       array![1, 2, 3, 4, 5]];
    let df1 = DataFrame::from_vec(values1,
                                  vec!["A", "B", "C", "D", "E"],
                                  vec!["X", "Y", "Z"]);

    let values2 = vec![array![4, 5, 6],
                       array![7.1, 8.1, 9.1]];
    let df2 = DataFrame::from_vec(values2,
                                  vec!["A", "D", "B"],
                                  vec!["X2", "Y2"]);
    let res = df1.join_inner(&df2);

    let exp_values = vec![array![1, 2, 4],
                          array![4.1, 5.1, 7.1],
                          array![1, 2, 4],
                          array![4, 6, 5],
                          array![7.1, 9.1, 8.1]];
    let exp = DataFrame::from_vec(exp_values,
                                  vec!["A", "B", "D"],
                                  vec!["X", "Y", "Z", "X2", "Y2"]);

    assert_eq!(res, exp);
}
