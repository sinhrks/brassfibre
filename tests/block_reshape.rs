extern crate brassfibre;
use brassfibre::*;

#[test]
fn test_block_append() {
    let b1 = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                 vec!["A", "B", "C"],
                                 vec!["X", "Y"]);
    let b2 = Block::from_col_vec(vec![7., 8., 9., 10., 11., 12.],
                                 vec!["D", "E", "F"],
                                 vec!["X", "Y"]);

    let res = b1.append(&b2);

    let exp = Block::from_col_vec(vec![1., 2., 3., 7., 8., 9.,
                                       4., 5., 6., 10., 11., 12.],
                                 vec!["A", "B", "C", "D", "E", "F"],
                                 vec!["X", "Y"]);
    assert_eq!(res, exp);
}

#[test]
#[should_panic]
fn test_block_append_failure() {
    // different columns
    let b1 = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                 vec!["A", "B", "C"],
                                 vec!["X", "Z"]);
    let b2 = Block::from_col_vec(vec![7., 8., 9., 10., 11., 12.],
                                 vec!["D", "E", "F"],
                                 vec!["X", "Y"]);

    b1.append(&b2);
}

#[test]
fn test_block_concat() {
    let b1 = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                 vec!["A", "B", "C"],
                                 vec!["X1", "Y1"]);
    let b2 = Block::from_col_vec(vec![7., 8., 9., 10., 11., 12.],
                                 vec!["A", "B", "C"],
                                 vec!["X2", "Y2"]);

    let res = b1.concat(&b2);

    let exp = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.,
                                       7., 8., 9., 10., 11., 12.],
                                 vec!["A", "B", "C"],
                                 vec!["X1", "Y1", "X2", "Y2"]);
    assert_eq!(res, exp);
}

#[test]
#[should_panic]
fn test_block_concat_panic() {
    // different index
    let b1 = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                 vec!["A", "B", "C"],
                                 vec!["X1", "Y1"]);
    let b2 = Block::from_col_vec(vec![7., 8., 9., 10., 11., 12.],
                                 vec!["A", "B", "D"],
                                 vec!["X2", "Y2"]);

    b1.concat(&b2);
}

#[test]
fn test_block_join() {
    let b1 = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                 vec!["A", "B", "C"],
                                 vec!["X1", "Y1"]);
    let b2 = Block::from_col_vec(vec![7., 8., 9., 10., 11., 12.],
                                 vec!["A", "B", "C"],
                                 vec!["X2", "Y2"]);

    let res = b1.join_inner(&b2);
    let exp = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.,
                                       7., 8., 9., 10., 11., 12.],
                                 vec!["A", "B", "C"],
                                 vec!["X1", "Y1", "X2", "Y2"]);
    assert_eq!(res, exp);

    let b3 = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                 vec!["D", "B", "A"],
                                 vec!["X3", "Y3"]);
    let res = b1.join_inner(&b3);
    let exp = Block::from_col_vec(vec![1., 2., 4., 5.,
                                       3., 2., 6., 5.],
                                 vec!["A", "B"],
                                 vec!["X1", "Y1", "X3", "Y3"]);
    assert_eq!(res, exp);
}
