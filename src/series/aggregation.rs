use num::{Num, Zero, ToPrimitive};

use std::hash::Hash;

use super::Series;
use super::super::algos::counter::Counter;
use super::super::computations;
use super::super::traits::{Applicable, Aggregator};


impl<'i, V, I> Aggregator<'i> for Series<'i, V, I>
    where V: Copy + Num + Zero + ToPrimitive,
          I: Clone + Eq + Hash {

    type Kept = V;
    type Counted = usize;
    type Coerced = f64;

    fn sum(&'i self) -> Self::Kept {
        self.apply(&computations::vec_sum)
    }

    fn count(&'i self) -> Self::Counted {
        self.apply(&computations::vec_count)
    }

    fn mean(&'i self) -> Self::Coerced {
        self.apply(&computations::vec_mean)
    }

    fn var(&'i self) -> Self::Coerced {
        self.apply(&computations::vec_var)
    }

    fn unbiased_var(&'i self) -> Self::Coerced {
        self.apply(&computations::vec_unbiased_var)
    }

    fn std(&'i self) -> Self::Coerced {
        self.apply(&computations::vec_std)
    }

    fn unbiased_std(&'i self) -> Self::Coerced {
        self.apply(&computations::vec_unbiased_std)
    }
}

impl<'i, V, I> Series<'i, V, I>
    where V: Copy + Num + Zero + ToPrimitive + computations::NanMinMax<V>,
          I: Clone + Eq + Hash {

    pub fn min(&'i self) -> V {
        self.apply(&computations::vec_min)
    }

    pub fn max(&'i self) -> V {
        self.apply(&computations::vec_max)
    }

    pub fn describe<'a>(&self) -> Series<'a, f64, &str> {
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

impl<'i, V, I> Series<'i, V, I>
    where V: Copy + Eq + Hash + Ord,
          I: Clone + Eq + Hash {

    pub fn value_counts<'a>(&self) -> Series<'a, usize, V> {
        let c = Counter::new(&self.values);
        let (keys, counts) = c.get_results();
        Series::new(counts, keys)
    }
}