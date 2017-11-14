use num::{Zero, ToPrimitive};
use std::hash::Hash;
use std::ops::{Add, Sub, Div};

use super::Series;
use algos::computation::{Aggregation, NanMinMax};
use algos::counter::Counter;
use traits::{Apply, BasicAggregation, NumericAggregation, ComparisonAggregation, Description};


impl<'v, 'i, V, I> BasicAggregation<'i> for Series<'v, 'i, V, I>
where
    V: Clone + Zero + Add,
    I: Clone + Eq + Hash,
{
    type Kept = V;
    type Counted = usize;

    fn sum(&'i self) -> Self::Kept {
        self.apply(&Aggregation::vec_sum)
    }

    fn count(&'i self) -> Self::Counted {
        self.apply(&Aggregation::vec_count)
    }
}


impl<'v, 'i, V, I> NumericAggregation<'i> for Series<'v, 'i, V, I>
where
    V: Clone
        + Zero
        + Add
        + Sub
        + Div
        + ToPrimitive,
    I: Clone + Eq + Hash,
{
    type Coerced = f64;

    fn mean(&'i self) -> Self::Coerced {
        self.apply(&Aggregation::vec_mean)
    }

    fn var(&'i self) -> Self::Coerced {
        self.apply(&Aggregation::vec_var)
    }

    fn unbiased_var(&'i self) -> Self::Coerced {
        self.apply(&Aggregation::vec_unbiased_var)
    }

    fn std(&'i self) -> Self::Coerced {
        self.apply(&Aggregation::vec_std)
    }

    fn unbiased_std(&'i self) -> Self::Coerced {
        self.apply(&Aggregation::vec_unbiased_std)
    }
}

impl<'v, 'i, V, I> ComparisonAggregation<'i> for Series<'v, 'i, V, I>
where
    V: Clone + NanMinMax<V>,
    I: Clone + Eq + Hash,
{
    type Kept = V;

    fn min(&'i self) -> Self::Kept {
        self.apply(&Aggregation::vec_min)
    }

    fn max(&'i self) -> Self::Kept {
        self.apply(&Aggregation::vec_max)
    }
}

impl<'v, 'i, V, I> Description<'i> for Series<'v, 'i, V, I>
where
    V: Clone
        + Zero
        + Add
        + Sub
        + Div
        + ToPrimitive
        + NanMinMax<V>,
    I: Clone + Eq + Hash,
{
    type Described = Series<'i, 'i, f64, &'i str>;

    fn describe(&'i self) -> Self::Described {
        let new_index: Vec<&str> = vec!["count", "mean", "std", "min", "max"];
        let count: f64 = Aggregation::vec_count(&self.values) as f64;

        let min = ToPrimitive::to_f64(&self.min()).unwrap();
        let max = ToPrimitive::to_f64(&self.max()).unwrap();

        let new_values: Vec<f64> = vec![count, self.mean(), self.std(), min, max];
        Series::new(new_values, new_index)
    }
}

// Other

impl<'v, 'i, V, I> Series<'v, 'i, V, I>
where
    V: Clone + Eq + Hash + Ord,
    I: Clone + Eq + Hash,
{
    pub fn value_counts<'a>(&self) -> Series<'a, 'a, usize, V> {
        let c = Counter::new(&self.values);
        let (keys, counts) = c.get_results();
        Series::new(counts, keys)
    }
}
