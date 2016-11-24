
use num::{Num, Zero, ToPrimitive};
use std::cmp::Ord;
use std::hash::Hash;


use super::Series;
use super::super::algos::groupby::{GroupBy, HashGroupBy};
use super::super::traits::{RowIndexer, Applicable, Aggregator};

pub struct SeriesGroupBy<'a, T: 'a, U: 'a + Hash, G: 'a + Hash> {
    /// Grouped Series
    /// T: type of Series values
    /// U: type of Series indexer
    /// V: type of Group indexer

    pub series: &'a Series<T, U>,
    pub grouper: HashGroupBy<G>,
}

impl<'a, T, U, G> SeriesGroupBy<'a, T, U, G>
    where T: Copy,
          U: Copy + Eq + Hash,
          G: Copy + Eq + Hash + Ord {

    pub fn new(series: &Series<T, U>, indexer: Vec<G>) -> SeriesGroupBy<T, U, G> {

        if series.len() != indexer.len() {
            panic!("Series and Indexer length are different");
        }

        let grouper: HashGroupBy<G> = HashGroupBy::groupby(&indexer);

        SeriesGroupBy {
            series: series,
            grouper: grouper,
        }
    }

    pub fn get_group(&self, group: &G) -> Series<T, U> {
        if let Some(locs) = self.grouper.get(group) {
            self.series.ilocs(&locs)
        } else {
            panic!("Group not found!");
        }
    }

    pub fn groups(&self) -> Vec<G> {
        let mut keys: Vec<G> = self.grouper.keys();
        keys.sort();
        keys
    }
}

////////////////////////////////////////////////////////////////////////////////
// Apply
////////////////////////////////////////////////////////////////////////////////

impl<'a, T, U, G, W> Applicable<Series<T, U>, W, Series<W, G>>
    for SeriesGroupBy<'a, T, U, G>

    where T: Copy,
          U: Copy + Eq + Hash,
          G: Copy + Eq + Hash + Ord,
          W: Copy {

    /// Apply passed function to each group
    fn apply(&self, func: &Fn(&Series<T, U>) -> W) -> Series<W, G> {

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

impl<'a, T, U, G> Aggregator for SeriesGroupBy<'a, T, U, G>
    where T: Copy + Eq + Hash + Num + Zero + ToPrimitive,
          U: Copy + Eq + Hash,
          G: Copy + Eq + Hash + Ord {

    type Kept = Series<T, G>;
    type Counted = Series<usize, G>;
    type Coerced = Series<f64, G>;

    fn sum(&self) -> Series<T, G> {
        self.apply(&|x: &Series<T, U>| x.sum())
    }

    fn count(&self) -> Series<usize, G> {
        self.apply(&|x: &Series<T, U>| x.count())
    }

    fn mean(&self) -> Series<f64, G> {
        self.apply(&|x: &Series<T, U>| x.mean())
    }

    fn var(&self) -> Series<f64, G> {
        self.apply(&|x: &Series<T, U>| x.var())
    }

    fn unbiased_var(&self) -> Series<f64, G> {
        self.apply(&|x: &Series<T, U>| x.unbiased_var())
    }

    fn std(&self) -> Series<f64, G> {
        self.apply(&|x: &Series<T, U>| x.std())
    }

    fn unbiased_std(&self) -> Series<f64, G> {
        self.apply(&|x: &Series<T, U>| x.unbiased_std())
    }
}

#[cfg(test)]
mod tests {


    use super::SeriesGroupBy;
    use super::super::Series;
    use super::super::super::indexer::Indexer;
    use super::super::super::Aggregator;

    #[test]
    fn test_series_get_group() {
        let values: Vec<f64> = vec![1., 2., 3., 4., 5., 6.];
        let s = Series::<f64, usize>::from_vec(values);

        // Instanciate directly method
        let sg = SeriesGroupBy::<f64, usize, i64>::new(&s, vec![1, 1, 1, 2, 2, 2]);
        assert_eq!(sg.groups().len(), 2);

        let s1 = sg.get_group(&1);
        let exp_values: Vec<f64> = vec![1., 2., 3.];
        let exp_index: Indexer<usize> = Indexer::new(vec![0, 1, 2]);
        assert_eq!(s1.values, exp_values);
        assert_eq!(s1.index, exp_index);

        let s2 = sg.get_group(&2);
        let exp_values: Vec<f64> = vec![4., 5., 6.];
        let exp_index: Indexer<usize> = Indexer::new(vec![3, 4, 5]);
        assert_eq!(s2.values, exp_values);
        assert_eq!(s2.index, exp_index);
    }

    #[test]
    fn test_series_agg_sum_integer_grouper() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<i64, i64>::new(values, index);

        let sg = SeriesGroupBy::<i64, i64, i64>::new(&s, vec![1, 1, 1, 2, 2]);
        let sum = sg.sum();

        let exp_values: Vec<i64> = vec![6, 9];
        let exp_index: Indexer<i64> = Indexer::new(vec![1, 2]);
        assert_eq!(sum.values, exp_values);
        assert_eq!(sum.index, exp_index);
    }

    #[test]
    fn test_series_agg_sum_str_grouper() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<i64, i64>::new(values, index);
        let sg = SeriesGroupBy::<i64, i64, &str>::new(&s, vec!["A", "A", "A", "B", "B"]);
        let sum = sg.sum();

        let exp_values: Vec<i64> = vec![6, 9];
        let exp_index: Indexer<&str> = Indexer::new(vec!["A", "B"]);
        assert_eq!(sum.values, exp_values);
        assert_eq!(sum.index, exp_index);
    }

    #[test]
    fn test_series_agg_mean_integer_grouper() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<i64, i64>::new(values, index);

        let sg = SeriesGroupBy::<i64, i64, i64>::new(&s, vec![1, 1, 1, 2, 2]);
        let sum = sg.mean();

        let exp_values: Vec<f64> = vec![2.0, 4.5];
        let exp_index: Indexer<i64> = Indexer::new(vec![1, 2]);
        assert_eq!(sum.values, exp_values);
        assert_eq!(sum.index, exp_index);
    }
}