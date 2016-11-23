extern crate itertools;
extern crate num;

use itertools::Zip;
use std::hash::Hash;

use super::algos::sort::Sorter;
use super::eval::Applicable;
use super::indexer::{Indexer, IndexerIndexer};
use super::traits::RowIndexer;

mod aggregation;
mod convert;
mod formatting;
mod groupby;
mod ops;
mod sort;

#[derive(Clone)]
pub struct Series<T, U: Hash> {
    pub values: Vec<T>,
    pub index: Indexer<U>,
}

////////////////////////////////////////////////////////////////////////////////
// Indexing
////////////////////////////////////////////////////////////////////////////////

impl<T, U> RowIndexer<U> for Series<T, U>
    where T: Copy,
          U: Copy + Eq + Hash {

    fn reindex(&mut self, labels: &Vec<U>) -> Self {
        let locs = self.index.get_locs(labels);
        let new_values = Sorter::reindex(&self.values, &locs);
        Series::new(new_values, labels.to_owned())
    }

    fn reindex_by_index(&self, locations: &Vec<usize>) -> Self {
        let new_index = self.index.reindex(&locations);
        let new_values = Sorter::reindex(&self.values, &locations);
        Series::new(new_values, new_index)
    }

    /// Slice using given Vec<bool> (slice by Bool LOCationS)
    fn blocs(&self, flags: &Vec<bool>) -> Self {

        if self.len() != flags.len() {
            panic!("Values and Indexer length are different");
        }

        let mut new_values: Vec<T> = Vec::with_capacity(self.len());
        let mut new_index: Vec<U> = Vec::with_capacity(self.len());

        // ToDo: remove itertools
        for (&flag, &v, &i) in Zip::new((flags, &self.values,
                                         &self.index.values)) {
            if flag {
                new_values.push(v);
                new_index.push(i);
            }
        }
        Series::new(new_values, new_index)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////

impl<T, U> Series<T, U>
    where T: Copy,
          U: Copy + Eq + Hash {

    pub fn from_vec(values: Vec<T>) -> Series<T, usize> {
        let index: Indexer<usize> = Indexer::<usize>::from_len(values.len());

        Series {
            values: values,
            index: index,
        }
    }

    pub fn new<I>(values: Vec<T>, index: I) -> Self
        where I: Into<Indexer<U>> {

        let index: Indexer<U> = index.into();

        if values.len() != index.len() {
            panic!("Length mismatch!");
        }
        Series {
            values: values,
            index: index,
        }
    }

    fn assert_binop(&self, other: &Series<T, U>) {
        if self.index != other.index {
            panic!("index must be the same!");
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Return single value corresponding to given label
    pub fn get_by_label(&mut self, label: &U) -> T {
        let loc = self.index.get_loc(&label);
        self.get_by_index(&loc)
    }

    /// Return single value corresponding to given location
    pub fn get_by_index(&self, location: &usize) -> T {
        self.values[*location]
    }

    pub fn append(&self, other: &Series<T, U>) -> Self {
        let mut new_values: Vec<T> = self.values.clone();
        let mut new_index: Vec<U> = self.index.values.clone();
        new_values.append(&mut other.values.clone());
        new_index.append(&mut other.index.values.clone());

        Series::new(new_values, new_index)
    }

    pub fn groupby<G>(&self, other: Vec<G>) -> groupby::SeriesGroupBy<T, U, G>
        where G: Copy + Eq + Hash + Ord {
        groupby::SeriesGroupBy::new(&self, other)
    }
}

impl<T, U, R> Applicable<T, R, R> for Series<T, U>
    where T: Copy,
          U: Copy + Eq + Hash {

    fn apply(&self, func: &Fn(&Vec<T>) -> R) -> R {
        func(&self.values)
    }
}

impl<T: PartialEq, U: Hash + Eq> PartialEq for Series<T, U> {
    fn eq(&self, other: &Series<T, U>) -> bool {
        (self.index.eq(&other.index)) && (self.values.eq(&other.values))
    }
}


#[cfg(test)]
mod tests {

    use super::Series;
    use super::super::indexer::{Indexer, IndexerIndexer};
    use super::super::traits::RowIndexer;

    #[test]
    fn test_series_creation_from_vec() {
        let values: Vec<f64> = vec![1., 2., 3.];

        let s = Series::<f64, i64>::from_vec(values);

        let exp_values: Vec<f64> = vec![1., 2., 3.];
        let exp_index: Indexer<usize> = Indexer::new(vec![0, 1, 2]);
        assert_eq!(s.values, exp_values);
        assert_eq!(s.index, exp_index);

        assert_eq!(s.len(), 3);
        assert_eq!(s.index.len(), 3);
    }

    #[test]
    fn test_series_creation_from_index() {
        let values: Vec<f64> = vec![1., 2., 3.];
        let index: Vec<i64> = vec![5, 6, 7];

        let s = Series::<f64, i64>::new(values, index);

        let exp_values: Vec<f64> = vec![1., 2., 3.];
        let exp_index: Indexer<i64> = Indexer::new(vec![5, 6, 7]);
        assert_eq!(s.values, exp_values);
        assert_eq!(s.index, exp_index);

        assert_eq!(s.len(), 3);
        assert_eq!(s.index.len(), 3);
    }

    #[test]
    fn test_series_creation_from_into_index() {
        let values: Vec<f64> = vec![1., 2., 3.];
        let index: Indexer<i64> = Indexer::new(vec![5, 6, 7]);

        let s = Series::<f64, i64>::new(values, index);

        let exp_values: Vec<f64> = vec![1., 2., 3.];
        let exp_index: Indexer<i64> = Indexer::new(vec![5, 6, 7]);
        assert_eq!(s.values, exp_values);
        assert_eq!(s.index, exp_index);

        assert_eq!(s.len(), 3);
        assert_eq!(s.index.len(), 3);
    }

    #[test]
    fn test_series_copy() {
        let values: Vec<f64> = vec![1., 2., 3.];
        let index: Vec<i64> = vec![5, 6, 7];

        let s = Series::<f64, i64>::new(values, index);
        let copied = s.clone();

        let exp_values: Vec<f64> = vec![1., 2., 3.];
        let exp_index: Indexer<i64> = Indexer::new(vec![5, 6, 7]);
        assert_eq!(copied.values, exp_values);
        assert_eq!(copied.index, exp_index);

        assert_eq!(copied, s);
    }

    #[test]
    fn test_series_equals() {
        let s1 = Series::<f64, i64>::new(vec![1., 2., 3.], vec![5, 6, 7]);
        let s2 = Series::<f64, i64>::new(vec![1., 2., 3.], vec![9, 6, 7]);;
        let s3 = Series::<f64, i64>::new(vec![1., 2., 3.], vec![5, 6, 7]);;
        let s4 = Series::<f64, i64>::new(vec![1., 2., 4.], vec![5, 6, 7]);;

        assert_eq!(s1 == s2, false);
        assert_eq!(s1 == s3, true);
        assert_eq!(s1 == s4, false);
    }

    #[test]
    fn test_series_slice_locs() {
        let values: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];

        let mut s = Series::new(values, index);

        // test construction
        let exp_values: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let exp_index: Indexer<i64> = Indexer::new(vec![10, 20, 30, 40, 50]);
        assert_eq!(s.values, exp_values);
        assert_eq!(s.index, exp_index);

        // test label slice
        let res = s.locs(&vec![20, 30, 50]);
        let exp: Series<f64, i64> = Series::new(vec![2., 3., 5.], vec![20, 30, 50]);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_series_slice_ilocs() {
        let values: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];

        let s = Series::<f64, i64>::new(values, index);

        // test construction
        let exp_values: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let exp_index: Indexer<i64> = Indexer::new(vec![10, 20, 30, 40, 50]);
        assert_eq!(s.values, exp_values);
        assert_eq!(s.index, exp_index);

        // test index slice
        let res = s.ilocs(&vec![0, 2, 4]);

        let exp: Series<f64, i64> = Series::new(vec![1., 3., 5.], vec![10, 30, 50]);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_series_slice_blocs() {
        let values: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];

        let s = Series::<f64, i64>::new(values, index);

        // test construction
        let exp_values: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let exp_index: Indexer<i64> = Indexer::new(vec![10, 20, 30, 40, 50]);
        assert_eq!(s.values, exp_values);
        assert_eq!(s.index, exp_index);

        // test bool slice
        let res = s.blocs(&vec![true, false, false, true, true]);

        let exp: Series<f64, i64> = Series::new(vec![1., 4., 5.], vec![10, 40, 50]);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_series_reindex() {
        let mut s: Series<&str, &str> = Series::new(vec!["a", "b", "c", "d"],
                                                    vec!["A", "B", "C", "D"]);
        let res = s.reindex(&vec!["D", "C", "A"]);
        let exp: Series<&str, &str> = Series::new(vec!["d", "c", "a"],
                                                  vec!["D", "C", "A"]);

        assert_eq!(res, exp);
    }

    #[test]
    fn test_series_reindex_by_index() {
        let s: Series<&str, &str> = Series::new(vec!["a", "b", "c", "d"],
                                                vec!["A", "B", "C", "D"]);
        let res = s.reindex_by_index(&vec![1, 3, 0]);
        let exp: Series<&str, &str> = Series::new(vec!["b", "d", "a"],
                                                  vec!["B", "D", "A"]);

        assert_eq!(res, exp);
    }

    #[test]
    fn test_series_append() {
        let values: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];

        let s1 = Series::new(values, index);

        let values: Vec<f64> = vec![11., 12., 13., 14., 15.];
        let index: Vec<i64> = vec![110, 120, 130, 140, 150];

        let s2 = Series::new(values, index);

        let res = s1.append(&s2);
        let exp: Series<f64, i64> = Series::new(vec![1., 2., 3., 4., 5., 11., 12., 13., 14., 15.],
                                                vec![10, 20, 30, 40, 50, 110, 120, 130, 140, 150]);
        assert_eq!(res, exp);
    }
}