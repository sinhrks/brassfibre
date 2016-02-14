extern crate itertools;
extern crate num;

use itertools::Zip;
use num::{Num, Zero, Float, ToPrimitive};
use std::hash::Hash;
use std::fmt;

use super::computations;
use super::formatting;
use super::seriesgroupby::SeriesGroupBy;
use super::index::Indexer;

pub struct Series<T, U: Hash> {
    pub values: Vec<T>,
    pub index: Indexer<U>,
}

// Indexing

impl<T: Copy, U: Copy + Eq + Hash> Series<T, U> {

    pub fn from_vec(values: Vec<T>) -> Series<T, i64> {
        let mut index: Vec<i64> = vec![];
        for i in 0..values.len() as i64 {
            index.push(i);
        }
        Series {
            values: values,
            index: Indexer::new(index),
        }
    }

    pub fn new(values: Vec<T>, index: Vec<U>) -> Series<T, U> {
        if values.len() != index.len() {
            panic!("Length mismatch!");
        }
        Series {
            values: values,
            index: Indexer::new(index),
        }
    }

    pub fn len(&self) -> usize {
        return self.values.len();
    }

    pub fn copy(&self) -> Series<T, U> {
        // copy vec
        let new_values = self.values.iter().map(|x| *x).collect();
        return Series::new(new_values, self.index.copy_values());
    }

    pub fn get_by_label(&mut self, label: &U) -> T {
        /*
        return single value corresponding to given label.
        */
        let loc = self.index.get_label_loc(&label);
        return self.get_by_index(&loc);
    }

    pub fn get_by_index(&self, location: &usize) -> T {
        /*
        return single value corresponding to given location.
        */
        return self.values[*location];
    }

    pub fn slice_by_label(&mut self, labels: &Vec<U>) -> Series<T, U> {
        /*
        slice Series using given labels.
        */

        // self must be mut to update label_mapper
        let locs = self.index.slice_label_loc(labels);
        return self.slice_by_index(&locs);
    }

    pub fn slice_by_index(&self, locations: &Vec<usize>) -> Series<T, U> {
        /*
        slice Series using given locations.
        */
        let mut new_values: Vec<T> = vec![];
        let mut new_index: Vec<U> = vec![];

        for loc in locations {
            new_values.push(self.values[*loc]);
            new_index.push(self.index.values[*loc]);
        }
        return Series::<T, U>::new(new_values, new_index);
    }

    pub fn slice_by_bool(&self, flags: &Vec<bool>) -> Series<T, U> {
        /*
        slice Series using given bool flags.
        */

        if self.len() != flags.len() {
            panic!("Values and Indexer length are different");
        }

        let mut new_values: Vec<T> = vec![];
        let mut new_index: Vec<U> = vec![];

        for (&flag, &v, &i) in Zip::new((flags, &self.values,
                                         &self.index.values)) {
            if flag {
                new_values.push(v);
                new_index.push(i);
            }
        }
        return Series::<T, U>::new(new_values, new_index);
    }

    pub fn append(&self, other: &Series<T, U>) -> Series<T, U> {
        let mut new_values: Vec<T> = vec![];
        let mut new_index: Vec<U> = vec![];

        for (&v, &i) in Zip::new((&self.values, &self.index.values)) {
            new_values.push(v);
            new_index.push(i);
        }
        for (&v, &i) in Zip::new((&other.values, &other.index.values)) {
            new_values.push(v);
            new_index.push(i);
        }
        return Series::<T, U>::new(new_values, new_index);
    }

    pub fn groupby<G: Copy + Eq + Hash + Ord>(&self, other: Vec<G>) -> SeriesGroupBy<T, U, G> {
        return SeriesGroupBy::new(self.copy(), other);
    }
}

// Formatting

impl<T: fmt::Debug, U> fmt::Display for Series<T, U> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Series({:?})", &self.values);
    }

}

impl<T: ToString, U: ToString> fmt::Debug for Series<T, U> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_index = formatting::pad_string_vector(&self.index.values);
        let str_values = formatting::pad_string_vector(&self.values);

        let mut result = "".to_string();
        for (i, v) in Zip::new((&str_index, &str_values)) {
            result.push_str(i);
            result.push(' ');
            result.push_str(v);
            result.push_str("\n");
        }
        // debug expression {:?} outputs linesep as character, do not use
        return write!(f, "{:}", &result);
    }

}

// Aggregation

impl<T: Copy + Num + Zero + ToPrimitive, U: Hash> Series<T, U> {

    pub fn sum(&self) -> T {
        return computations::vec_sum(&self.values);
    }

    pub fn count(&self) -> usize {
        return computations::vec_count(&self.values);
    }

    pub fn mean(&self) -> f64 {
        return computations::vec_mean(&self.values)
    }

    pub fn var(&self) -> f64 {
        return computations::vec_var(&self.values);
    }

    pub fn unbiased_var(&self) -> f64 {
        return computations::vec_unbiased_var(&self.values);
    }

    pub fn std(&self) -> f64 {
        return computations::vec_std(&self.values);
    }

    pub fn unbiased_std(&self) -> f64 {
        return computations::vec_unbiased_std(&self.values);
    }

    pub fn describe(&self) -> Series<f64, &str> {
        let new_index: Vec<&str> = vec!["count", "mean", "std"];
        let count_f64 = computations::vec_count_as_f64(&self.values);
        let new_values: Vec<f64> = vec![count_f64,
                                        self.mean(),
                                        self.std()];
        // ToDo:: min / max
        return Series::new(new_values, new_index);
    }
}

// Integer
impl<T: Copy + Num + Ord, U: Hash> Series<T, U> {

    pub fn min(&self) -> T {
        return computations::vec_min(&self.values);
    }

    pub fn max(&self) -> T {
        return computations::vec_max(&self.values);
    }
}

// Float
impl<T: Copy + Num + Float, U: Hash> Series<T, U> {

    pub fn min(&self) -> T {
        return computations::vec_min_float(&self.values);
    }

    pub fn max(&self) -> T {
        return computations::vec_max_float(&self.values);
    }
}


#[cfg(test)]
mod tests {

    use super::Series;

    #[test]
    fn test_series_creation_from_vec() {
        let values: Vec<f64> = vec![1., 2., 3.];

        let s = Series::<f64, i64>::from_vec(values);

        let exp_values: Vec<f64> = vec![1., 2., 3.];
        let exp_index: Vec<i64> = vec![0, 1, 2];
        assert_eq!(&s.values, &exp_values);
        assert_eq!(&s.index.values, &exp_index);

        assert_eq!(&s.len(), &3);
        assert_eq!(&s.index.len(), &3);
    }

    #[test]
    fn test_series_creation_from_index() {
        let values: Vec<f64> = vec![1., 2., 3.];
        let index: Vec<i64> = vec![5, 6, 7];

        let s = Series::<f64, i64>::new(values, index);

        let exp_values: Vec<f64> = vec![1., 2., 3.];
        let exp_index: Vec<i64> = vec![5, 6, 7];
        assert_eq!(&s.values, &exp_values);
        assert_eq!(&s.index.values, &exp_index);

        assert_eq!(&s.len(), &3);
        assert_eq!(&s.index.len(), &3);
    }

    #[test]
    fn test_series_slice_by_label() {
        let values: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];

        let mut s = Series::<f64, i64>::new(values, index);

        // test construction
        let exp_values: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let exp_index: Vec<i64> = vec![10, 20, 30, 40, 50];
        assert_eq!(&s.values, &exp_values);
        assert_eq!(&s.index.values, &exp_index);

        // test label slice
        let res = s.slice_by_label(&vec![20, 30, 50]);

        let exp_values: Vec<f64> = vec![2., 3., 5.];
        let exp_index: Vec<i64> = vec![20, 30, 50];
        assert_eq!(&res.values, &exp_values);
        assert_eq!(&res.index.values, &exp_index);
    }

    #[test]
    fn test_series_slice_by_index() {
        let values: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];

        let s = Series::<f64, i64>::new(values, index);

        // test construction
        let exp_values: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let exp_index: Vec<i64> = vec![10, 20, 30, 40, 50];
        assert_eq!(&s.values, &exp_values);
        assert_eq!(&s.index.values, &exp_index);

        // test index slice
        let res = s.slice_by_index(&vec![0, 2, 4]);

        let exp_values: Vec<f64> = vec![1., 3., 5.];
        let exp_index: Vec<i64> = vec![10, 30, 50];
        assert_eq!(&res.values, &exp_values);
        assert_eq!(&res.index.values, &exp_index);
    }

    #[test]
    fn test_series_slice_by_bool() {
        let values: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];

        let s = Series::<f64, i64>::new(values, index);

        // test construction
        let exp_values: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let exp_index: Vec<i64> = vec![10, 20, 30, 40, 50];
        assert_eq!(&s.values, &exp_values);
        assert_eq!(&s.index.values, &exp_index);

        // test bool slice
        let res = s.slice_by_bool(&vec![true, false, false, true, true]);

        let exp_values: Vec<f64> = vec![1., 4., 5.];
        let exp_index: Vec<i64> = vec![10, 40, 50];
        assert_eq!(&res.values, &exp_values);
        assert_eq!(&res.index.values, &exp_index);
    }

    #[test]
    fn test_series_append() {
        let values: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];

        let s1 = Series::<f64, i64>::new(values, index);

        let values: Vec<f64> = vec![11., 12., 13., 14., 15.];
        let index: Vec<i64> = vec![110, 120, 130, 140, 150];

        let s2 = Series::<f64, i64>::new(values, index);

        let res = s1.append(&s2);
        let exp_values: Vec<f64> = vec![1., 2., 3., 4., 5., 11., 12., 13., 14., 15.];
        let exp_index: Vec<i64> = vec![10, 20, 30, 40, 50, 110, 120, 130, 140, 150];
        assert_eq!(&res.values, &exp_values);
        assert_eq!(&res.index.values, &exp_index);
    }

    #[test]
    fn test_series_agg_int() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];

        let s = Series::<i64, i64>::new(values, index);

        assert_eq!(&s.sum(), &15);
        assert_eq!(&s.min(), &1);
        assert_eq!(&s.max(), &5);
        assert_eq!(&s.count(), &5);
        assert_eq!(&s.mean(), &3.0);
        assert_eq!(&s.var(), &2.0);
        assert_eq!(&s.unbiased_var(), &2.5);

        let values: Vec<i64> = vec![2, 2, 2, 3, 3];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];

        let s = Series::<i64, i64>::new(values, index);
        assert_eq!(&s.mean(), &2.4);

        let values: Vec<i64> = vec![11, 12, 11, 14, 12];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<i64, i64>::new(values, index);

        assert_eq!(&s.var(), &1.2);
        assert_eq!(&s.unbiased_var(), &1.5);

        assert_eq!(&s.std(), &1.0954451150103321);
        assert_eq!(&s.unbiased_std(), &1.2247448713915889);
    }

    #[test]
    fn test_series_agg_float() {
        let values: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<f64, i64>::new(values, index);

        assert_eq!(&s.sum(), &15.);
        assert_eq!(&s.min(), &1.);
        assert_eq!(&s.max(), &5.);
        assert_eq!(&s.count(), &5);
        assert_eq!(&s.mean(), &3.);
        assert_eq!(&s.var(), &2.0);
        assert_eq!(&s.unbiased_var(), &2.5);

        let values: Vec<f64> = vec![11., 12., 11., 14., 12.];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<f64, i64>::new(values, index);

        assert_eq!(&s.var(), &1.2);
        assert_eq!(&s.unbiased_var(), &1.5);

        assert_eq!(&s.std(), &1.0954451150103321);
        assert_eq!(&s.unbiased_std(), &1.2247448713915889);
    }

    #[test]
    fn test_series_describe() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let s = Series::<i64, i64>::from_vec(values);

        let d = s.describe();
        let exp_values: Vec<f64> = vec![10., 5.5, 2.8722813232690143];
        let exp_index: Vec<&str> = vec!["count", "mean", "std"];
        assert_eq!(&d.values, &exp_values);
        assert_eq!(&d.index.values, &exp_index);
    }
}