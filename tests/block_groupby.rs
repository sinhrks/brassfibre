extern crate brassfibre;
use brassfibre::*;

#[test]
fn test_block_get_group() {
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let b = Block::from_col_vec(values, vec!["A", "B", "C", "D", "E"], vec!["X", "Y", "Z"]);
    assert_eq!(b.len(), 5);

    let bg = b.groupby(vec![1, 2, 1, 1, 2]);
    assert_eq!(&bg.groups().len(), &2);

    let exp = Block::from_col_vec(vec![1, 3, 4, 6, 8, 9, 11, 13, 14],
                                  vec!["A", "C", "D"],
                                  vec!["X", "Y", "Z"]);
    assert_eq!(bg.get_group(&1), exp);
}

#[test]
fn test_block_agg() {
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let b = Block::from_col_vec(values, vec!["A", "B", "C", "D", "E"], vec!["X", "Y", "Z"]);
    assert_eq!(b.len(), 5);

    let bg = b.groupby(vec![1, 2, 1, 1, 2]);

    let exp = Block::from_col_vec(vec![8, 7, 23, 17, 38, 27], vec![1, 2], vec!["X", "Y", "Z"]);
    assert_eq!(bg.sum(), exp);

    let exp = Block::from_col_vec(vec![8. / 3., 3.5, 23. / 3., 8.5, 38. / 3., 13.5],
                                  vec![1, 2],
                                  vec!["X", "Y", "Z"]);
    assert_eq!(bg.mean(), exp);

    let exp = Block::from_col_vec(vec![1, 2, 6, 7, 11, 12], vec![1, 2], vec!["X", "Y", "Z"]);
    assert_eq!(bg.min(), exp);

    let exp = Block::from_col_vec(vec![4, 5, 9, 10, 14, 15], vec![1, 2], vec!["X", "Y", "Z"]);
    assert_eq!(bg.max(), exp);
}

#[test]
fn test_block_agg2() {
    let values = vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                      vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]];
    let b = Block::from_nested_vec(values, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec!["X", "Y"]);
    assert_eq!(&b.len(), &10);

    let bg = b.groupby(vec![1, 1, 1, 2, 2, 2, 1, 1, 1, 2]);
    let bagg = bg.sum();

    // mean
    let exp = Block::from_col_vec(vec![30, 25, 60, 50], vec![1, 2], vec!["X", "Y"]);
    assert_eq!(bagg, exp);

    // count
    let bagg = bg.count();
    let exp = Block::from_col_vec(vec![6, 4, 6, 4], vec![1, 2], vec!["X", "Y"]);
    assert_eq!(bagg, exp);

    // var
    let bagg = bg.var();
    let exp = Block::from_col_vec(vec![9.666666666666666, 5.1875, 38.666666666666664, 20.75],
                                  vec![1, 2],
                                  vec!["X", "Y"]);
    assert_eq!(bagg, exp);

    // unbiased var
    let bagg = bg.unbiased_var();
    let exp = Block::from_col_vec(vec![11.6, 6.916666666666667, 46.4, 27.666666666666668],
                                  vec![1, 2],
                                  vec!["X", "Y"]);
    assert_eq!(bagg, exp);

    // std
    let bagg = bg.std();
    let exp = Block::from_col_vec(vec![3.1091263510296048,
                                       2.277608394786075,
                                       6.2182527020592095,
                                       4.55521678957215],
                                  vec![1, 2],
                                  vec!["X", "Y"]);
    assert_eq!(bagg, exp);

    // unbiased std
    let bagg = bg.unbiased_std();
    let exp = Block::from_col_vec(vec![3.40587727318528,
                                       2.6299556396765835,
                                       6.81175454637056,
                                       5.259911279353167],
                                  vec![1, 2],
                                  vec!["X", "Y"]);
    assert_eq!(bagg, exp);
}
