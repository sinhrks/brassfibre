extern crate num;

use num::{Num, Zero, ToPrimitive};
use std::hash::Hash;

use super::Block;
use super::super::computations;
use super::super::series::Series;
use super::super::traits::{Applicable, Aggregator};

impl<'i, 'c, V, I, C> Aggregator<'c, 'c> for Block<'i, 'c, V, I, C>
    where V: Copy + Num + Zero + ToPrimitive,
          I: Copy + Eq + Hash,
          C: 'c + Copy + Eq + Hash {

    type Kept = Series<'c, V, C>;
    type Counted = Series<'c, usize, C>;
    type Coerced = Series<'c, f64, C>;

    fn sum(&'c self) -> Self::Kept {
        self.apply(&computations::vec_sum)
    }

    fn count(&'c self) -> Self::Counted {
        self.apply(&computations::vec_count)
    }

    fn mean(&'c self) -> Self::Coerced {
        self.apply(&computations::vec_mean)
    }

    fn var(&'c self) -> Self::Coerced {
        self.apply(&computations::vec_var)
    }

    fn unbiased_var(&'c self) -> Self::Coerced {
        self.apply(&computations::vec_unbiased_var)
    }

    fn std(&'c self) -> Self::Coerced {
        self.apply(&computations::vec_std)
    }

    fn unbiased_std(&'c self) -> Self::Coerced {
        self.apply(&computations::vec_unbiased_std)
    }
}

impl<'i, 'c, V, I, C> Block<'i, 'c, V, I, C>
    where V: Copy + Num + Zero + computations::NanMinMax<V>,
          I: Copy + Eq + Hash,
          C: Copy + Eq + Hash {

    pub fn min(&'c self) -> Series<'c, V, C> {
        self.apply(&computations::vec_min)
    }

    pub fn max(&'c self) -> Series<'c, V, C> {
        self.apply(&computations::vec_max)
    }
}
