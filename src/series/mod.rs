extern crate itertools;
extern crate num;

use itertools::Zip;
use std::hash::Hash;

use super::algos::sort::Sorter;
use super::indexer::Indexer;
use super::traits::{IndexerIndexer, RowIndexer, Appender, Applicable};

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

    fn reindex(&self, labels: &Vec<U>) -> Self {
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
    pub fn get_by_label(&self, label: &U) -> T {
        let loc = self.index.get_loc(&label);
        self.get_by_index(&loc)
    }

    /// Return single value corresponding to given location
    pub fn get_by_index(&self, location: &usize) -> T {
        self.values[*location]
    }

    pub fn groupby<G>(&self, other: Vec<G>) -> groupby::SeriesGroupBy<T, U, G>
        where G: Copy + Eq + Hash + Ord {
        groupby::SeriesGroupBy::new(&self, other)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Append
////////////////////////////////////////////////////////////////////////////////

impl<T, U> Appender for Series<T, U>
    where T: Copy,
          U: Copy + Eq + Hash {

    fn append(&self, other: &Self) -> Self {
        let mut new_values: Vec<T> = self.values.clone();
        new_values.append(&mut other.values.clone());
        let new_index = self.index.append(&other.index);
        Series::new(new_values, new_index)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Apply
////////////////////////////////////////////////////////////////////////////////

impl<T, U, R> Applicable<Vec<T>, R, R> for Series<T, U>
    where T: Copy,
          U: Copy + Eq + Hash {

    fn apply(&self, func: &Fn(&Vec<T>) -> R) -> R {
        func(&self.values)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Eq
////////////////////////////////////////////////////////////////////////////////

impl<T: PartialEq, U: Hash + Eq> PartialEq for Series<T, U> {
    fn eq(&self, other: &Series<T, U>) -> bool {
        (self.index.eq(&other.index)) && (self.values.eq(&other.values))
    }
}

