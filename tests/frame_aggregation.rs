#[macro_use]
extern crate brassfibre;
use brassfibre::*;

#[test]
fn test_frame_agg() {
    let values: Vec<Array> = vec![array![1, 2, 3, 4, 5],
                                  array![6., 7., 8., 9., 10.]];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let columns: Vec<&str> = vec!["X", "Y"];
    let df = DataFrame::from_vec(values, index, columns);

    let sum = df.sum();
    let exp: Series<f64, &str> = Series::new(vec![15., 40.], vec!["X", "Y"]);
    assert_eq!(sum, exp);

    let mean = df.count();
    let exp: Series<usize, &str> = Series::new(vec![5, 5], vec!["X", "Y"]);
    assert_eq!(mean, exp);

    let mean = df.mean();
    let exp: Series<f64, &str> = Series::new(vec![3., 8.], vec!["X", "Y"]);
    assert_eq!(mean, exp);

    let mean = df.var();
    let exp: Series<f64, &str> = Series::new(vec![2., 2.], vec!["X", "Y"]);
    assert_eq!(mean, exp);

    let mean = df.unbiased_var();
    let exp: Series<f64, &str> = Series::new(vec![2.5, 2.5], vec!["X", "Y"]);
    assert_eq!(mean, exp);

    let mean = df.std();
    let exp: Series<f64, &str> = Series::new(vec![1.4142135623730951, 1.4142135623730951], vec!["X", "Y"]);
    assert_eq!(mean, exp);

    let mean = df.unbiased_std();
    let exp: Series<f64, &str> = Series::new(vec![1.5811388300841898, 1.5811388300841898], vec!["X", "Y"]);
    assert_eq!(mean, exp);
}

#[test]
fn test_frame_agg_non_numerics() {
    let values: Vec<Array> = vec![array!["a", "b", "c", "d", "e"],
                                  array![1, 7, 3, -2, 5],
                                  array![true, false, true, false ,true],
                                  array![3.1, 7.5, 8., 9.0, 10.]];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let columns: Vec<&str> = vec!["A", "B", "C", "D"];
    let df = DataFrame::from_vec(values, index, columns);

    let sum = df.sum();
    let exp: Series<f64, &str> = Series::new(vec![14., 37.600000000000001], vec!["B", "D"]);
    assert_eq!(sum, exp);

    let mean = df.count();
    let exp: Series<usize, &str> = Series::new(vec![5, 5], vec!["B", "D"]);
    assert_eq!(mean, exp);

    let mean = df.mean();
    let exp: Series<f64, &str> = Series::new(vec![2.7999999999999998, 7.5200000000000005], vec!["B", "D"]);
    assert_eq!(mean, exp);

    let mean = df.var();
    let exp: Series<f64, &str> = Series::new(vec![9.7599999999999998, 5.621599999999999], vec!["B", "D"]);
    assert_eq!(mean, exp);

    let mean = df.unbiased_var();
    let exp: Series<f64, &str> = Series::new(vec![12.199999999999999, 7.0269999999999992], vec!["B", "D"]);
    assert_eq!(mean, exp);

    let mean = df.std();
    let exp: Series<f64, &str> = Series::new(vec![3.1240998703626617, 2.3709913538433662], vec!["B", "D"]);
    assert_eq!(mean, exp);

    let mean = df.unbiased_std();
    let exp: Series<f64, &str> = Series::new(vec![3.4928498393145961, 2.6508489206290125], vec!["B", "D"]);
    assert_eq!(mean, exp);
}
