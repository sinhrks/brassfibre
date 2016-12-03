extern crate brassfibre;
use brassfibre::*;

// Some tests are under series::groupby which directly uses SeriesGroupBy (private internal)

#[test]
fn test_series_get_group() {
    let values: Vec<f64> = vec![1., 2., 3., 4., 5., 6.];
    let s = Series::<f64, usize>::from_vec(values);

    // Use Series method
    let sg = s.groupby(vec![1, 1, 1, 2, 2, 2]);
    assert_eq!(sg.groups().len(), 2);

    let s1 = sg.get_group(&1);
    let exp: Series<f64, usize> = Series::new(vec![1., 2., 3.], vec![0, 1, 2]);
    assert_eq!(s1, exp);

    let s2 = sg.get_group(&2);
    let exp: Series<f64, usize> = Series::new(vec![4., 5., 6.], vec![3, 4, 5]);
    assert_eq!(s2, exp);
}

#[test]
fn test_series_agg_sum_integer_grouper() {
    let values: Vec<i64> = vec![1, 2, 3, 4, 5];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let s = Series::<i64, i64>::new(values, index);

    let sg = s.groupby(vec![1, 1, 1, 2, 2]);
    let sum = sg.sum();

    let exp: Series<i64, i64> = Series::new(vec![6, 9], vec![1, 2]);
    assert_eq!(sum, exp);
}

#[test]
fn test_series_agg_sum_str_grouper() {
    let values: Vec<i64> = vec![1, 2, 3, 4, 5];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let s = Series::<i64, i64>::new(values, index);
    let sg = s.groupby(vec!["A", "A", "A", "B", "B"]);
    let sum = sg.sum();

    let exp: Series<i64, &str> = Series::new(vec![6, 9], vec!["A", "B"]);
    assert_eq!(sum, exp);
}

#[test]
fn test_series_agg_mean_integer_grouper() {
    let values: Vec<i64> = vec![1, 2, 3, 4, 5];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let s = Series::<i64, i64>::new(values, index);

    let sg = s.groupby(vec![1, 1, 1, 2, 2]);
    let sum = sg.mean();

    let exp: Series<f64, i64> = Series::new(vec![2.0, 4.5], vec![1, 2]);
    assert_eq!(sum, exp);
}