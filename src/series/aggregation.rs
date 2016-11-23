extern crate num;

use num::{Num, Zero, ToPrimitive};

use std::hash::Hash;

use super::Series;
use super::super::algos::counter::Counter;
use super::super::computations;
use super::super::eval::Applicable;


impl<T, U> Series<T, U>
    where T: Copy + Num + Zero + ToPrimitive,
          U: Copy + Eq + Hash {

    pub fn sum(&self) -> T {
        self.apply(&computations::vec_sum)
    }

    pub fn count(&self) -> usize {
        self.apply(&computations::vec_count)
    }

    pub fn mean(&self) -> f64 {
        self.apply(&computations::vec_mean)
    }

    pub fn var(&self) -> f64 {
        self.apply(&computations::vec_var)
    }

    pub fn unbiased_var(&self) -> f64 {
        self.apply(&computations::vec_unbiased_var)
    }

    pub fn std(&self) -> f64 {
        self.apply(&computations::vec_std)
    }

    pub fn unbiased_std(&self) -> f64 {
        self.apply(&computations::vec_unbiased_std)
    }
}

impl<T, U> Series<T, U>
    where T: Copy + Num + Zero + ToPrimitive + computations::NanMinMax<T>,
          U: Copy + Eq + Hash {

    pub fn min(&self) -> T {
        self.apply(&computations::vec_min)
    }

    pub fn max(&self) -> T {
        self.apply(&computations::vec_max)
    }

    pub fn describe(&self) -> Series<f64, &str> {
        let new_index: Vec<&str> = vec!["count", "mean", "std", "min", "max"];
        let count_f64 = computations::vec_count_as_f64(&self.values);

        let min = ToPrimitive::to_f64(&self.min()).unwrap();
        let max = ToPrimitive::to_f64(&self.max()).unwrap();

        let new_values: Vec<f64> = vec![count_f64,
                                        self.mean(),
                                        self.std(),
                                        min,
                                        max];
        Series::new(new_values, new_index)
    }
}

// Other

impl<T, U> Series<T, U>
    where T: Copy + Eq + Hash + Ord,
          U: Copy + Eq + Hash {

    pub fn value_counts(&self) -> Series<usize, T> {
        let c = Counter::new(&self.values);
        let (keys, counts) = c.get_results();
        Series::new(counts, keys)
    }
}

#[cfg(test)]
mod tests {

    use super::super::Series;

    #[test]
    fn test_series_agg_int() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];

        let s = Series::<i64, i64>::new(values, index);

        assert_eq!(&s.sum(), &15);
        assert_eq!(&s.min(), &1);
        assert_eq!(&s.max(), &5);
        assert_eq!(&s.count(), &5);
        assert_eq!(&s.mean(), &3.0);
        assert_eq!(&s.var(), &2.0);
        assert_eq!(&s.unbiased_var(), &2.5);

        let values: Vec<i64> = vec![2, 2, 2, 3, 3];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];

        let s = Series::<i64, i64>::new(values, index);
        assert_eq!(&s.mean(), &2.4);

        let values: Vec<i64> = vec![11, 12, 11, 14, 12];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<i64, i64>::new(values, index);

        assert_eq!(&s.var(), &1.2);
        assert_eq!(&s.unbiased_var(), &1.5);

        assert_eq!(&s.std(), &1.0954451150103321);
        assert_eq!(&s.unbiased_std(), &1.2247448713915889);
    }

    #[test]
    fn test_series_agg_float() {
        let values: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<f64, i64>::new(values, index);

        assert_eq!(&s.sum(), &15.);
        assert_eq!(&s.min(), &1.);
        assert_eq!(&s.max(), &5.);
        assert_eq!(&s.count(), &5);
        assert_eq!(&s.mean(), &3.);
        assert_eq!(&s.var(), &2.0);
        assert_eq!(&s.unbiased_var(), &2.5);

        let values: Vec<f64> = vec![11., 12., 11., 14., 12.];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<f64, i64>::new(values, index);

        assert_eq!(&s.var(), &1.2);
        assert_eq!(&s.unbiased_var(), &1.5);

        assert_eq!(&s.std(), &1.0954451150103321);
        assert_eq!(&s.unbiased_std(), &1.2247448713915889);
    }

    #[test]
    fn test_series_describe_int() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let s = Series::<i64, i64>::from_vec(values);

        let d = s.describe();
        let exp_values: Vec<f64> = vec![10., 5.5, 2.8722813232690143, 1., 10.];
        let exp_index: Vec<&str> = vec!["count", "mean", "std", "min", "max"];
        assert_eq!(&d.values, &exp_values);
        assert_eq!(&d.index.values, &exp_index);
    }

    #[test]
    fn test_series_describe_float() {
        let values: Vec<f64> = vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
        let s = Series::<f64, i64>::from_vec(values);

        let d = s.describe();
        let exp_values: Vec<f64> = vec![10., 5.5, 2.8722813232690143, 1., 10.];
        let exp_index: Vec<&str> = vec!["count", "mean", "std", "min", "max"];
        assert_eq!(&d.values, &exp_values);
        assert_eq!(&d.index.values, &exp_index);
    }

    #[test]
    fn test_series_value_counts_int() {
        let values: Vec<i64> = vec![1, 1, 3, 4, 2, 1, 1, 2, 3, 3];
        let s = Series::<i64, usize>::from_vec(values);

        let d = s.value_counts();
        let exp: Series<usize, i64> = Series::new(vec![4, 3, 2, 1], vec![1, 3, 2, 4]);
        assert_eq!(&d, &exp);
    }

    #[test]
    fn test_series_value_counts_str() {
        let values: Vec<&str> = vec!["a", "bb", "bb", "c", "a", "a"];
        let s = Series::<&str, usize>::from_vec(values);

        let d = s.value_counts();
        let exp: Series<usize, &str> = Series::new(vec![3, 2, 1], vec!["a", "bb", "c"]);
        assert_eq!(&d, &exp);
    }
}