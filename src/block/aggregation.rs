extern crate num;

use num::{Num, Zero, ToPrimitive};
use std::hash::Hash;

use super::Block;
use super::super::computations;
use super::super::series::Series;
use super::super::traits::{Applicable, Aggregator};

impl<T, U, V> Aggregator for Block<T, U, V>
    where T: Copy + Num + Zero + ToPrimitive,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash {

    type Kept = Series<T, V>;
    type Counted = Series<usize, V>;
    type Coerced = Series<f64, V>;

    fn sum(&self) -> Series<T, V> {
        self.apply(&computations::vec_sum)
    }

    fn count(&self) -> Series<usize, V> {
        self.apply(&computations::vec_count)
    }

    fn mean(&self) -> Series<f64, V> {
        self.apply(&computations::vec_mean)
    }

    fn var(&self) -> Series<f64, V> {
        self.apply(&computations::vec_var)
    }

    fn unbiased_var(&self) -> Series<f64, V> {
        self.apply(&computations::vec_unbiased_var)
    }

    fn std(&self) -> Series<f64, V> {
        self.apply(&computations::vec_std)
    }

    fn unbiased_std(&self) -> Series<f64, V> {
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
