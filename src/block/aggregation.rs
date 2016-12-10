use num::{Num, Zero, ToPrimitive};
use std::hash::Hash;
use std::ops::{Add, Sub, Div};

use super::Block;
use super::super::computations;
use super::super::series::Series;
use super::super::traits::{Apply, BasicAggregation, NumericAggregation,
                           ComparisonAggregation};

impl<'v, 'i, 'c, V, I, C> BasicAggregation<'c> for Block<'v, 'i, 'c, V, I, C>
    where V: 'c + Clone + Zero + Add,
          I: Clone + Eq + Hash,
          C: 'c + Clone + Eq + Hash {

    type Kept = Series<'c, 'c, V, C>;
    type Counted = Series<'c, 'c, usize, C>;

    fn sum(&'c self) -> Self::Kept {
        self.apply(&computations::vec_sum)
    }

    fn count(&'c self) -> Self::Counted {
        self.apply(&computations::vec_count)
    }
}

impl<'v, 'i, 'c, V, I, C> NumericAggregation<'c> for Block<'v, 'i, 'c, V, I, C>
    where V: 'c + Clone + Zero + Add + Sub + Div + ToPrimitive,
          I: Clone + Eq + Hash,
          C: 'c + Clone + Eq + Hash {

    type Coerced = Series<'c, 'c, f64, C>;

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

impl<'v, 'i, 'c, V, I, C> Block<'v, 'i, 'c, V, I, C>
    where V: Clone + computations::NanMinMax<V>,
          I: Clone + Eq + Hash,
          C: Clone + Eq + Hash {

    pub fn min(&'c self) -> Series<'c, 'c, V, C> {
        self.apply(&computations::vec_min)
    }

    pub fn max(&'c self) -> Series<'c, 'c, V, C> {
        self.apply(&computations::vec_max)
    }
}
