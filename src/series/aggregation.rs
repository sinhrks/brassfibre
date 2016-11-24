extern crate num;

use num::{Num, Zero, ToPrimitive};

use std::hash::Hash;

use super::Series;
use super::super::algos::counter::Counter;
use super::super::computations;
use super::super::traits::{Applicable, Aggregator};


impl<T, U> Aggregator for Series<T, U>
    where T: Copy + Num + Zero + ToPrimitive,
          U: Copy + Eq + Hash {

    type Kept = T;
    type Counted = usize;
    type Coerced = f64;

    fn sum(&self) -> T {
        self.apply(&computations::vec_sum)
    }

    fn count(&self) -> usize {
        self.apply(&computations::vec_count)
    }

    fn mean(&self) -> f64 {
        self.apply(&computations::vec_mean)
    }

    fn var(&self) -> f64 {
        self.apply(&computations::vec_var)
    }

    fn unbiased_var(&self) -> f64 {
        self.apply(&computations::vec_unbiased_var)
    }

    fn std(&self) -> f64 {
        self.apply(&computations::vec_std)
    }

    fn unbiased_std(&self) -> f64 {
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