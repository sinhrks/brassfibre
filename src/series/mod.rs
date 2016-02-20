extern crate itertools;
extern crate num;

use itertools::Zip;
use std::hash::Hash;

use super::seriesgroupby::SeriesGroupBy;
use super::indexer::Indexer;

mod aggregation;
mod formatting;
mod ops;

pub struct Series<T, U: Hash> {
    pub values: Vec<T>,
    pub index: Indexer<U>,
}

// Indexing

impl<T, U> Series<T, U>
    where T: Copy,
          U: Copy + Eq + Hash {

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

    fn assert_binop(&self, other: &Series<T, U>) {
        if !self.index.equals(&other.index) {
            panic!("index must be the same!");
        }
    }

    pub fn len(&self) -> usize {
        return self.values.len();
    }

    pub fn copy(&self) -> Series<T, U> {
        // copy vec
        return Series::new(self.values.clone(), self.index.copy_values());
    }

    pub fn equals(&self, other: &Series<T, U>) -> bool
        where T: PartialEq {
        /*
        Whether Series is equal to other
        */
        return self.index.equals(&other.index) && self.values == other.values;
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
        let mut new_values: Vec<T> = self.values.clone();
        let mut new_index: Vec<U> = self.index.values.clone();
        new_values.append(&mut other.values.clone());
        new_index.append(&mut other.index.values.clone());

        return Series::<T, U>::new(new_values, new_index);
    }

    pub fn groupby<G>(&self, other: Vec<G>) -> SeriesGroupBy<T, U, G>
        where G: Copy + Eq + Hash + Ord {
        return SeriesGroupBy::new(self.copy(), other);
    }

    pub fn apply<W: Copy>(&self, func: &Fn(&Vec<T>) -> W) -> W {
        /*
        Apply passed function to each columns.
        */
        return func(&self.values);
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
    fn test_series_copy() {
        let values: Vec<f64> = vec![1., 2., 3.];
        let index: Vec<i64> = vec![5, 6, 7];

        let s = Series::<f64, i64>::new(values, index);
        let copied = s.copy();

        let exp_values: Vec<f64> = vec![1., 2., 3.];
        let exp_index: Vec<i64> = vec![5, 6, 7];
        assert_eq!(&copied.values, &exp_values);
        assert_eq!(&copied.index.values, &exp_index);
    }

    #[test]
    fn test_series_equals() {
        let s1 = Series::<f64, i64>::new(vec![1., 2., 3.], vec![5, 6, 7]);
        let s2 = Series::<f64, i64>::new(vec![1., 2., 3.], vec![9, 6, 7]);;
        let s3 = Series::<f64, i64>::new(vec![1., 2., 3.], vec![5, 6, 7]);;

        assert_eq!(&s1.equals(&s2), &false);
        assert_eq!(&s1.equals(&s3), &true);
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
}