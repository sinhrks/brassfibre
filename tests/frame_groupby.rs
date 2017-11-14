#[macro_use]
extern crate brassfibre;
use brassfibre::prelude::*;

#[test]
fn test_frame_get_group() {
    let values = vec![
        array![1, 2, 3, 4, 5],
        array![6., 7., 8., 9., 10.],
        array![11, 12, 13, 14, 15],
    ];
    let df = DataFrame::from_vec(
        values,
        vec!["A", "BB", "CC", "D", "EEE"],
        vec!["X", "YYY", "ZZ"],
    );
    assert_eq!(df.len(), 5);

    let dg = df.groupby(vec![1, 2, 1, 1, 2]);
    assert_eq!(&dg.groups().len(), &2);

    let df1 = dg.get_group(&1);
    let exp_values = vec![array![1, 3, 4], array![6., 8., 9.], array![11, 13, 14]];
    let exp = DataFrame::from_vec(exp_values, vec!["A", "CC", "D"], vec!["X", "YYY", "ZZ"]);
    assert_eq!(df1.values, exp.values);
    assert_eq!(df1.index, exp.index);
    assert_eq!(df1.columns, exp.columns);
}
