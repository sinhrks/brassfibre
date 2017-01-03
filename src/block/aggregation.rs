use num::{Num, Zero, ToPrimitive};
use std::borrow::{Borrow, Cow};
use std::hash::Hash;
use std::ops::{Add, Sub, Div};

use super::Block;
use algos::computation::{Aggregation, NanMinMax};
use indexer::Indexer;
use series::Series;
use traits::{Apply, BasicAggregation, NumericAggregation, ComparisonAggregation, Description};

impl<'v, 'i, 'c, V, I, C> BasicAggregation<'c> for Block<'v, 'i, 'c, V, I, C>
    where V: 'c + Clone + Zero + Add,
          I: Clone + Eq + Hash,
          C: 'c + Clone + Eq + Hash
{
    type Kept = Series<'c, 'c, V, C>;
    type Counted = Series<'c, 'c, usize, C>;

    fn sum(&'c self) -> Self::Kept {
        self.apply(&Aggregation::vec_sum)
    }

    fn count(&'c self) -> Self::Counted {
        self.apply(&Aggregation::vec_count)
    }
}

impl<'v, 'i, 'c, V, I, C> NumericAggregation<'c> for Block<'v, 'i, 'c, V, I, C>
    where V: 'c + Clone + Zero + Add + Sub + Div + ToPrimitive,
          I: Clone + Eq + Hash,
          C: 'c + Clone + Eq + Hash
{
    type Coerced = Series<'c, 'c, f64, C>;

    fn mean(&'c self) -> Self::Coerced {
        self.apply(&Aggregation::vec_mean)
    }

    fn var(&'c self) -> Self::Coerced {
        self.apply(&Aggregation::vec_var)
    }

    fn unbiased_var(&'c self) -> Self::Coerced {
        self.apply(&Aggregation::vec_unbiased_var)
    }

    fn std(&'c self) -> Self::Coerced {
        self.apply(&Aggregation::vec_std)
    }

    fn unbiased_std(&'c self) -> Self::Coerced {
        self.apply(&Aggregation::vec_unbiased_std)
    }
}

impl<'v, 'i, 'c, V, I, C> ComparisonAggregation<'c> for Block<'v, 'i, 'c, V, I, C>
    where V: 'c + Clone + NanMinMax<V>,
          I: Clone + Eq + Hash,
          C: Clone + Eq + Hash
{
    type Kept = Series<'c, 'c, V, C>;

    fn min(&'c self) -> Self::Kept {
        self.apply(&Aggregation::vec_min)
    }

    fn max(&'c self) -> Self::Kept {
        self.apply(&Aggregation::vec_max)
    }
}

impl<'v, 'i, 'c, V, I, C> Description<'c> for Block<'v, 'i, 'c, V, I, C>
    where V: 'c + Clone + Zero + Add + Sub + Div + ToPrimitive + NanMinMax<V>,
          I: Clone + Eq + Hash,
          C: Clone + Eq + Hash
{
    type Described = Block<'c, 'c, 'c, f64, &'c str, C>;

    fn describe(&'c self) -> Self::Described {
        let new_index: Vec<&str> = vec!["count", "mean", "std", "min", "max"];

        let describe = |x: &Vec<V>| {
            vec![Aggregation::vec_count(x) as f64,
                 Aggregation::vec_mean(x),
                 Aggregation::vec_std(x),
                 ToPrimitive::to_f64(&Aggregation::vec_min(x)).unwrap(),
                 ToPrimitive::to_f64(&Aggregation::vec_max(x)).unwrap()]
        };
        let new_values: Vec<Cow<Vec<f64>>> = self.values
            .iter()
            .map(|x| Cow::Owned(describe(x)))
            .collect();
        Block::from_cow(new_values,
                        Cow::Owned(Indexer::new(new_index)),
                        Cow::Borrowed(self.columns.borrow()))
    }
}
