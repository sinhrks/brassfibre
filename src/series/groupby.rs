
use num::{Num, Zero, ToPrimitive};
use std::cmp::Ord;
use std::hash::Hash;

use super::Series;
use super::super::algos::grouper::{Grouper};
use super::super::groupby::GroupBy;
use super::super::traits::{Applicable, Aggregator};

////////////////////////////////////////////////////////////////////////////////
// Apply
////////////////////////////////////////////////////////////////////////////////

impl<'v, 'i, V, I, G, W> Applicable<'i, W>
    for GroupBy<'i, Series<'v, 'i, V, I>, G>

    where V: 'v + Clone,
          I: Clone + Eq + Hash,
          G: 'i + Clone + Eq + Hash + Ord,
          W: 'i + Clone {

    type In = Series<'v, 'i, V, I>;
    type FOut = W;
    // ToDo: use 'n lifetime for value
    type Out = Series<'i, 'i, W, G>;

    /// Apply passed function to each group
    fn apply<'f>(&'i self, func: &'f Fn(&Series<'v, 'i, V, I>) -> W) -> Series<'i, 'i, W, G> {

        let mut new_values: Vec<W> = Vec::with_capacity(self.grouper.len());

        let groups = self.groups();
        for g in groups.iter() {
            let s = self.get_group(&g);
            new_values.push(func(&s));
        }
        Series::new(new_values, groups)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Aggregation
////////////////////////////////////////////////////////////////////////////////

impl<'v, 'i, V, I, G> Aggregator<'i> for GroupBy<'i, Series<'v, 'i, V, I>, G>
    where V: Clone + Eq + Hash + Num + Zero + ToPrimitive,
          I: Clone + Eq + Hash,
          G: 'i + Clone + Eq + Hash + Ord {

    // result can have different lifetime
    // ToDo: use 'n lifetime for value
    type Kept = Series<'i, 'i, V, G>;
    type Counted = Series<'i, 'i, usize, G>;
    type Coerced = Series<'i, 'i, f64, G>;

    fn sum(&'i self) -> Self::Kept {
        self.apply(&|x: &Series<V, I>| x.sum())
    }

    fn count(&'i self) -> Self::Counted {
        self.apply(&|x: &Series<V, I>| x.count())
    }

    fn mean(&'i self) -> Self::Coerced {
        self.apply(&|x: &Series<V, I>| x.mean())
    }

    fn var(&'i self) -> Self::Coerced {
        self.apply(&|x: &Series<V, I>| x.var())
    }

    fn unbiased_var(&'i self) -> Self::Coerced {
        self.apply(&|x: &Series<V, I>| x.unbiased_var())
    }

    fn std(&'i self) -> Self::Coerced {
        self.apply(&|x: &Series<V, I>| x.std())
    }

    fn unbiased_std(&'i self) -> Self::Coerced {
        self.apply(&|x: &Series<V, I>| x.unbiased_std())
    }
}

#[cfg(test)]
mod tests {

    use std::borrow::Cow;
    use super::super::Series;
    use super::super::super::indexer::Indexer;
    use super::super::super::groupby::GroupBy;
    use super::super::super::Aggregator;

    #[test]
    fn test_series_get_group() {
        let values: Vec<f64> = vec![1., 2., 3., 4., 5., 6.];
        let s = Series::<f64, usize>::from_vec(values);

        // Instanciate directly method
        let sg = GroupBy::<Series<f64, usize>, i64>::new(&s, vec![1, 1, 1, 2, 2, 2]);
        assert_eq!(sg.groups().len(), 2);

        let s1 = sg.get_group(&1);
        let exp_values: Vec<f64> = vec![1., 2., 3.];
        let exp_index: Indexer<usize> = Indexer::new(vec![0, 1, 2]);
        assert_eq!(s1.values, Cow::Borrowed(&exp_values));
        assert_eq!(s1.index, Cow::Owned(exp_index));

        let s2 = sg.get_group(&2);
        let exp_values: Vec<f64> = vec![4., 5., 6.];
        let exp_index: Indexer<usize> = Indexer::new(vec![3, 4, 5]);
        assert_eq!(s2.values, Cow::Borrowed(&exp_values));
        assert_eq!(s2.index, Cow::Owned(exp_index));
    }

    #[test]
    fn test_series_agg_sum_integer_grouper() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<i64, i64>::new(values, index);

        let sg = GroupBy::<Series<i64, i64>, i64>::new(&s, vec![1, 1, 1, 2, 2]);
        let sum = sg.sum();

        let exp: Series<i64, i64> = Series::new(vec![6, 9], vec![1, 2]);
        assert_eq!(sum, exp);
    }

    #[test]
    fn test_series_agg_sum_str_grouper() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<i64, i64>::new(values, index);
        let sg = GroupBy::<Series<i64, i64>, &str>::new(&s, vec!["A", "A", "A", "B", "B"]);
        let sum = sg.sum();

        let exp: Series<i64, &str> = Series::new(vec![6, 9], vec!["A", "B"]);
        assert_eq!(sum, exp);
    }

    #[test]
    fn test_series_agg_mean_integer_grouper() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<i64, i64>::new(values, index);

        let sg = GroupBy::<Series<i64, i64>, i64>::new(&s, vec![1, 1, 1, 2, 2]);
        let sum = sg.mean();

        let exp: Series<f64, i64> = Series::new(vec![2.0, 4.5], vec![1, 2]);
        assert_eq!(sum, exp);
    }
}