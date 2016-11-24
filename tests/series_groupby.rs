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
    let exp_values: Vec<f64> = vec![1., 2., 3.];
    let exp_index: Indexer<usize> = Indexer::new(vec![0, 1, 2]);
    assert_eq!(s1.values, exp_values);
    assert_eq!(s1.index, exp_index);

    let s2 = sg.get_group(&2);
    let exp_values: Vec<f64> = vec![4., 5., 6.];
    let exp_index: Indexer<usize> = Indexer::new(vec![3, 4, 5]);
    assert_eq!(s2.values, exp_values);
    assert_eq!(s2.index, exp_index);
}

#[test]
fn test_series_agg_sum_integer_grouper() {
    let values: Vec<i64> = vec![1, 2, 3, 4, 5];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let s = Series::<i64, i64>::new(values, index);

    let sg = s.groupby(vec![1, 1, 1, 2, 2]);
    let sum = sg.sum();

    let exp_values: Vec<i64> = vec![6, 9];
    let exp_index: Indexer<i64> = Indexer::new(vec![1, 2]);
    assert_eq!(sum.values, exp_values);
    assert_eq!(sum.index, exp_index);
}

#[test]
fn test_series_agg_sum_str_grouper() {
    let values: Vec<i64> = vec![1, 2, 3, 4, 5];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let s = Series::<i64, i64>::new(values, index);
    let sg = s.groupby(vec!["A", "A", "A", "B", "B"]);
    let sum = sg.sum();

    let exp_values: Vec<i64> = vec![6, 9];
    let exp_index: Indexer<&str> = Indexer::new(vec!["A", "B"]);
    assert_eq!(sum.values, exp_values);
    assert_eq!(sum.index, exp_index);
}

#[test]
fn test_series_agg_mean_integer_grouper() {
    let values: Vec<i64> = vec![1, 2, 3, 4, 5];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let s = Series::<i64, i64>::new(values, index);

    let sg = s.groupby(vec![1, 1, 1, 2, 2]);
    let sum = sg.mean();

    let exp_values: Vec<f64> = vec![2.0, 4.5];
    let exp_index: Indexer<i64> = Indexer::new(vec![1, 2]);
    assert_eq!(sum.values, exp_values);
    assert_eq!(sum.index, exp_index);
}