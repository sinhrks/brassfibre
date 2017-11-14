#[macro_use]
extern crate brassfibre;
use brassfibre::prelude::*;

#[test]
fn test_frame_agg() {
    let values: Vec<Array> = vec![array![1i64, 2, 3, 4, 5], array![6.0f64, 7., 8., 9., 10.]];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let columns: Vec<&str> = vec!["X", "Y"];
    let df = DataFrame::from_vec(values, index, columns);

    let exp: Series<Scalar, &str> =
        Series::new(vec![Scalar::i64(15), Scalar::f64(40.)], vec!["X", "Y"]);
    // assert_eq!(df.sum(), exp);

    let exp: Series<usize, &str> = Series::new(vec![5, 5], vec!["X", "Y"]);
    assert_eq!(df.count(), exp);

    let exp: Series<f64, &str> = Series::new(vec![3., 8.], vec!["X", "Y"]);
    assert_eq!(df.mean(), exp);

    let exp: Series<f64, &str> = Series::new(vec![2., 2.], vec!["X", "Y"]);
    assert_eq!(df.var(), exp);

    let exp: Series<f64, &str> = Series::new(vec![2.5, 2.5], vec!["X", "Y"]);
    assert_eq!(df.unbiased_var(), exp);

    let exp: Series<f64, &str> =
        Series::new(vec![1.4142135623730951, 1.4142135623730951], vec!["X", "Y"]);
    assert_eq!(df.std(), exp);

    let exp: Series<f64, &str> =
        Series::new(vec![1.5811388300841898, 1.5811388300841898], vec!["X", "Y"]);
    assert_eq!(df.unbiased_std(), exp);

    let exp: Series<f64, &str> = Series::new(vec![1., 6.], vec!["X", "Y"]);
    // assert_eq!(df.min(), exp);

    let exp: Series<f64, &str> = Series::new(vec![5., 10.], vec!["X", "Y"]);
    // assert_eq!(df.max(), exp);
}

#[test]
fn test_frame_agg_non_numerics() {
    let values: Vec<Array> = vec![
        array![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ],
        array![1i64, 7, 3, -2, 5],
        array![true, false, true, false, true],
        array![3.1f64, 7.5, 8., 9.0, 10.],
    ];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let columns: Vec<&str> = vec!["A", "B", "C", "D"];
    let df = DataFrame::from_vec(values, index, columns);

    let exp: Series<Scalar, &str> = Series::new(
        vec![Scalar::i64(14), Scalar::f64(37.600000000000001)],
        vec!["B", "D"],
    );
    // assert_eq!(df.sum(), exp);

    let exp: Series<usize, &str> = Series::new(vec![5, 5], vec!["B", "D"]);
    assert_eq!(df.count(), exp);

    let exp: Series<f64, &str> =
        Series::new(vec![2.7999999999999998, 7.5200000000000005], vec!["B", "D"]);
    assert_eq!(df.mean(), exp);

    let exp: Series<f64, &str> =
        Series::new(vec![9.7599999999999998, 5.621599999999999], vec!["B", "D"]);
    assert_eq!(df.var(), exp);

    let exp: Series<f64, &str> =
        Series::new(vec![12.199999999999999, 7.0269999999999992], vec!["B", "D"]);
    assert_eq!(df.unbiased_var(), exp);

    let exp: Series<f64, &str> =
        Series::new(vec![3.1240998703626617, 2.3709913538433662], vec!["B", "D"]);
    assert_eq!(df.std(), exp);

    let exp: Series<f64, &str> =
        Series::new(vec![3.4928498393145961, 2.6508489206290125], vec!["B", "D"]);
    assert_eq!(df.unbiased_std(), exp);

    let exp: Series<Scalar, &str> =
        Series::new(vec![Scalar::i64(-2), Scalar::f64(3.1)], vec!["B", "D"]);
    // assert_eq!(df.min(), exp);

    let exp: Series<Scalar, &str> =
        Series::new(vec![Scalar::i64(7), Scalar::f64(10.)], vec!["B", "D"]);
    // assert_eq!(df.max(), exp);
}

#[test]
fn test_frame_describe() {
    let values: Vec<Array> = vec![
        array![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ],
        array![1, 3, 2, 5, 8],
        array![true, false, true, false, true],
        array![1.1, 2.5, 3.2, 1.6, 0.8],
    ];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let columns: Vec<&str> = vec!["A", "B", "C", "D"];
    let df = DataFrame::from_vec(values, index, columns);

    let exp_values: Vec<Array> = vec![
        array![5., 3.8, 2.4819347291981715, 1., 8.],
        array![5., 1.8400000000000003, 0.8912911982062878, 0.8, 3.2],
    ];
    let exp = DataFrame::from_vec(
        exp_values,
        vec!["count", "mean", "std", "min", "max"],
        vec!["B", "D"],
    );
    assert_eq!(df.describe(), exp);
}
