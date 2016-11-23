extern crate num;

use num::{Num, Zero, ToPrimitive};
use std::hash::Hash;

use super::Block;
use super::super::computations;
use super::super::eval::Applicable;
use super::super::series::Series;


impl<T, U, V> Block<T, U, V>
    where T: Copy + Num + Zero + ToPrimitive,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash {

    // ToDo: Merge definition to Series
    pub fn sum(&self) -> Series<T, V> {
        self.apply(&computations::vec_sum)
    }

    pub fn count(&self) -> Series<usize, V> {
        self.apply(&computations::vec_count)
    }

    pub fn mean(&self) -> Series<f64, V> {
        self.apply(&computations::vec_mean)
    }

    pub fn var(&self) -> Series<f64, V> {
        self.apply(&computations::vec_var)
    }

    pub fn unbiased_var(&self) -> Series<f64, V> {
        self.apply(&computations::vec_unbiased_var)
    }

    pub fn std(&self) -> Series<f64, V> {
        self.apply(&computations::vec_std)
    }

    pub fn unbiased_std(&self) -> Series<f64, V> {
        self.apply(&computations::vec_unbiased_std)
    }
}

impl<T, U, V> Block<T, U, V>
    where T: Copy + Num + Zero + computations::NanMinMax<T>,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash {

    pub fn min(&self) -> Series<T, V> {
        self.apply(&computations::vec_min)
    }

    pub fn max(&self) -> Series<T, V> {
        self.apply(&computations::vec_max)
    }
}

#[cfg(test)]
mod tests {

    use super::super::Block;
    use super::super::super::series::Series;

    #[test]
    fn test_block_sum() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<i64, i64>::new(values, index);
        let mut b = Block::from_series(s, "X");

        let new_values: Vec<i64> = vec![6, 7, 8, 9, 10];
        b.add_columns(new_values, "Y");

        let sum = b.sum();

        let exp_values: Vec<i64> = vec![15, 40];
        let exp_index: Vec<&str> = vec!["X", "Y"];
        assert_eq!(&sum.values, &exp_values);
        assert_eq!(&sum.index.values, &exp_index);
    }

    #[test]
    fn test_block_mean() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<i64, i64>::new(values, index);
        let mut b = Block::from_series(s, "X");

        let new_values: Vec<i64> = vec![6, 7, 8, 9, 10];
        b.add_columns(new_values, "Y");

        let mean = b.mean();

        let exp_values: Vec<f64> = vec![3., 8.];
        let exp_index: Vec<&str> = vec!["X", "Y"];
        assert_eq!(&mean.values, &exp_values);
        assert_eq!(&mean.index.values, &exp_index);
    }

    #[test]
    fn test_minmax_int() {
        let values = vec![3, 2, 1, 4, 5,
                          7, 6, 8, 10, 10,
                          12, 14, 11, 14, 15];
        let b = Block::from_col_vec(values,
                                    vec!["A", "BB", "CC", "D", "EEE"],
                                    vec!["X", "YYY", "ZZ"]);
        assert_eq!(&b.len(), &5);

        let min = b.min();
        let exp_values: Vec<i64> = vec![1, 6, 11];
        let exp_index: Vec<&str> = vec!["X", "YYY", "ZZ"];
        assert_eq!(&min.values, &exp_values);
        assert_eq!(&min.index.values, &exp_index);

        let min = b.max();
        let exp_values: Vec<i64> = vec![5, 10, 15];
        let exp_index: Vec<&str> = vec!["X", "YYY", "ZZ"];
        assert_eq!(&min.values, &exp_values);
        assert_eq!(&min.index.values, &exp_index);
    }

    #[test]
    fn test_minmax_float() {
        let values = vec![3., 2., 1., 4., 5.,
                          7., 6., 8., 10., 10.,
                          12., 14., 11., 14., 15.];
        let b = Block::from_col_vec(values,
                                    vec!["A", "BB", "CC", "D", "EEE"],
                                    vec!["X", "YYY", "ZZ"]);
        assert_eq!(&b.len(), &5);

        let min = b.min();
        let exp_values: Vec<f64> = vec![1., 6., 11.];
        let exp_index: Vec<&str> = vec!["X", "YYY", "ZZ"];
        assert_eq!(&min.values, &exp_values);
        assert_eq!(&min.index.values, &exp_index);

        let min = b.max();
        let exp_values: Vec<f64> = vec![5., 10., 15.];
        let exp_index: Vec<&str> = vec!["X", "YYY", "ZZ"];
        assert_eq!(&min.values, &exp_values);
        assert_eq!(&min.index.values, &exp_index);
    }
}
