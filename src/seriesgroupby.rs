extern crate multimap;

use multimap::MultiMap;
use num::{Num, Zero, ToPrimitive};
use std::cmp::Ord;
use std::hash::Hash;

use super::series::Series;

pub struct SeriesGroupBy<T, U: Hash, G: Hash> {
    /// Grouped Series
    /// T: type of Series values
    /// U: type of Series indexer
    /// V: type of Group indexer

    pub series: Series<T, U>,
    pub grouper: MultiMap<G, usize>,
}

impl<T: Copy, U: Copy + Eq + Hash, G: Copy + Eq + Hash + Ord> SeriesGroupBy<T, U, G> {

    pub fn new(series: Series<T, U>, indexer: Vec<G>) -> SeriesGroupBy<T, U, G>{

        if series.len() != indexer.len() {
            panic!("Series and Indexer length are different");
        }

        let mut mapper = MultiMap::new();

        for (loc, label) in indexer.iter().enumerate() {
            mapper.insert(*label, loc);
        }

        SeriesGroupBy {
            series: series,
            grouper: mapper,
        }
    }

    pub fn get_group(&self, group: &G) -> Series<T, U> {

        if let Some(locs) = self.grouper.get_vec(group) {
            let values = locs.iter().map(|x| *x).collect();
            return self.series.slice_by_index(&values);
        } else {
            panic!("Group not found!");
        }
    }

    pub fn groups(&self) -> Vec<G> {
        let mut keys = vec![];
        for key in self.grouper.keys() {
            keys.push(*key)
        }
        // key is returned arbitrary order
        keys.sort();
        return keys;
    }
}

// Aggregation


impl<T: Copy + Num + Zero + ToPrimitive,
     U: Copy + Eq + Hash,
     G: Copy + Eq + Hash + Ord>

     SeriesGroupBy<T, U, G> {

    pub fn apply(&self, func: &Fn(&Series<T, U>) -> T) -> Series<T, G> {
        /*
        Apply passed function to each group.
        */
        let mut new_index: Vec<G> = vec![];
        let mut new_values: Vec<T> = vec![];

        for g in self.groups() {
            let s = self.get_group(&g);
            new_index.push(g);
            new_values.push(func(&s));
        }
        return Series::new(new_values, new_index);
    }

    fn apply_f64(&self, func: &Fn(&Series<T, U>) -> f64) -> Series<f64, G> {
        // ToDo: can't use generics to specify new return type?
        let mut new_index: Vec<G> = vec![];
        let mut new_values: Vec<f64> = vec![];

        for g in self.groups() {
            let s = self.get_group(&g);
            new_index.push(g);
            new_values.push(func(&s));
        }
        return Series::new(new_values, new_index);
    }

    pub fn sum(&self) -> Series<T, G> {
        return self.apply(&|s: &Series<T, U>| s.sum());
    }

    pub fn count(&self) -> Series<usize, G> {
        // ToDo: merge the logic to apply
        let mut new_index: Vec<G> = vec![];
        let mut new_values: Vec<usize> = vec![];

        for g in self.groups() {
            let s = self.get_group(&g);
            new_index.push(g);
            new_values.push(s.count());
        }
        return Series::new(new_values, new_index);
    }

    pub fn mean(&self) -> Series<f64, G> {
        return self.apply_f64(&|s: &Series<T, U>| s.mean());
    }

    pub fn var(&self) -> Series<f64, G> {
        return self.apply_f64(&|s: &Series<T, U>| s.var());
    }

    pub fn unbiased_var(&self) -> Series<f64, G> {
        return self.apply_f64(&|s: &Series<T, U>| s.unbiased_var());
    }

    pub fn std(&self) -> Series<f64, G> {
        return self.apply_f64(&|s: &Series<T, U>| s.std());
    }

    pub fn unbiased_std(&self) -> Series<f64, G> {
        return self.apply_f64(&|s: &Series<T, U>| s.unbiased_std());
    }
}

#[cfg(test)]
mod tests {

    use super::super::series::Series;
    use super::SeriesGroupBy;

    #[test]
    fn test_series_get_group() {
        let values: Vec<f64> = vec![1., 2., 3., 4., 5., 6.];
        let s = Series::<f64, i64>::from_vec(values);

        // Instanciate directly method
        let sg = SeriesGroupBy::<f64, i64, i64>::new(s, vec![1, 1, 1, 2, 2, 2]);
        assert_eq!(&sg.groups().len(), &2);

        let s1 = sg.get_group(&1);
        let exp_values: Vec<f64> = vec![1., 2., 3.];
        let exp_index: Vec<i64> = vec![0, 1, 2];
        assert_eq!(&s1.values, &exp_values);
        assert_eq!(&s1.index.values, &exp_index);

        let s2 = sg.get_group(&2);
        let exp_values: Vec<f64> = vec![4., 5., 6.];
        let exp_index: Vec<i64> = vec![3, 4, 5];
        assert_eq!(&s2.values, &exp_values);
        assert_eq!(&s2.index.values, &exp_index);
    }


    #[test]
    fn test_series_groupby_method() {
        let values: Vec<f64> = vec![1., 2., 3., 4., 5., 6.];
        let s = Series::<f64, i64>::from_vec(values);

        // Use Series method
        let sg = s.groupby(vec![1, 1, 1, 2, 2, 2]);
        assert_eq!(&sg.groups().len(), &2);

        let s1 = sg.get_group(&1);
        let exp_values: Vec<f64> = vec![1., 2., 3.];
        let exp_index: Vec<i64> = vec![0, 1, 2];
        assert_eq!(&s1.values, &exp_values);
        assert_eq!(&s1.index.values, &exp_index);

        let s2 = sg.get_group(&2);
        let exp_values: Vec<f64> = vec![4., 5., 6.];
        let exp_index: Vec<i64> = vec![3, 4, 5];
        assert_eq!(&s2.values, &exp_values);
        assert_eq!(&s2.index.values, &exp_index);
    }

    #[test]
    fn test_series_agg_sum_integer_grouper() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<i64, i64>::new(values, index);

        let sg = SeriesGroupBy::<i64, i64, i64>::new(s, vec![1, 1, 1, 2, 2]);
        let sum = sg.sum();

        let exp_values: Vec<i64> = vec![6, 9];
        let exp_index: Vec<i64> = vec![1, 2];
        assert_eq!(&sum.values, &exp_values);
        assert_eq!(&sum.index.values, &exp_index);
    }

    #[test]
    fn test_series_agg_sum_str_grouper() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<i64, i64>::new(values, index);
        let sg = SeriesGroupBy::<i64, i64, &str>::new(s, vec!["A", "A", "A", "B", "B"]);
        let sum = sg.sum();

        let exp_values: Vec<i64> = vec![6, 9];
        let exp_index: Vec<&str> = vec!["A", "B"];
        assert_eq!(&sum.values, &exp_values);
        assert_eq!(&sum.index.values, &exp_index);
    }

    #[test]
    fn test_series_agg_mean_integer_grouper() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<i64, i64>::new(values, index);

        let sg = SeriesGroupBy::<i64, i64, i64>::new(s, vec![1, 1, 1, 2, 2]);
        let sum = sg.mean();

        let exp_values: Vec<f64> = vec![2.0, 4.5];
        let exp_index: Vec<i64> = vec![1, 2];
        assert_eq!(&sum.values, &exp_values);
        assert_eq!(&sum.index.values, &exp_index);
    }

}