extern crate itertools;
extern crate num;

use itertools::Zip;
use std::borrow::Cow;
use std::hash::Hash;

use super::algos::sort::Sorter;
use super::indexer::Indexer;
use super::groupby::GroupBy;
use super::traits::{IndexerIndexer, RowIndexer, Appender, Applicable};

mod aggregation;
mod convert;
mod formatting;
mod groupby;
mod ops;
mod sort;

#[derive(Clone)]
pub struct Series<'i, V, I: 'i + Clone + Hash> {
    pub values: Vec<V>,
    pub index: Cow<'i, Indexer<I>>,
}

////////////////////////////////////////////////////////////////////////////////
// Indexing
////////////////////////////////////////////////////////////////////////////////

impl<'i, V, I> RowIndexer<'i> for Series<'i, V, I>
    where V: Copy,
          I: Copy + Eq + Hash {

    type Key = I;
    type Row = V;

    fn len(&self) -> usize {
        self.values.len()
    }

    fn loc(&self, label: &Self::Key) -> Self::Row {
        let loc = self.index.get_loc(&label);
        self.iloc(&loc)
    }

    fn iloc(&self, location: &usize) -> Self::Row {
        self.values[*location]
    }

    fn reindex(&self, labels: &Vec<Self::Key>) -> Self {
        let locs = self.index.get_locs(labels);
        let new_values = Sorter::reindex(&self.values, &locs);
        Series::new(new_values, labels.clone())
    }

    fn reindex_by_index(&self, locations: &Vec<usize>) -> Self {
        let new_index = self.index.reindex(&locations);
        let new_values = Sorter::reindex(&self.values, &locations);
        Series::new(new_values, new_index)
    }

    /// Slice using given Vec<bool> (slice by Bool LOCationS)
    fn blocs(&self, flags: &Vec<bool>) -> Self {

        assert!(self.len() == flags.len(),
                "Values and Indexer length are different");

        let mut new_values: Vec<Self::Row> = Vec::with_capacity(self.len());
        let mut new_index: Vec<Self::Key> = Vec::with_capacity(self.len());

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

impl<'i, V, I> Series<'i, V, I>
    where V: Copy,
          I: 'i + Copy + Eq + Hash {

    pub fn from_vec(values: Vec<V>) -> Series<'i, V, usize> {
        let index: Indexer<usize> = Indexer::<usize>::from_len(values.len());

        Series {
            values: values,
            index: Cow::Owned(index),
        }
    }

    pub fn new<X>(values: Vec<V>, index: X) -> Self
        where X: Into<Indexer<I>> {

        let index: Indexer<I> = index.into();

        assert!(values.len() == index.len(), "Length mismatch!");

        Series {
            values: values,
            index: Cow::Owned(index),
        }
    }

    pub fn from_cow(values: Vec<V>, index: Cow<'i, Indexer<I>>) -> Self {
        Series {
            values: values,
            index: index,
        }
    }

    fn assert_binop(&self, other: &Self) {
        assert!(self.index == other.index, "index must be the same!");
    }

    pub fn groupby<G>(&self, other: Vec<G>) -> GroupBy<Series<V, I>, G>
        where G: Copy + Eq + Hash + Ord {
        GroupBy::new(&self, other)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Append
////////////////////////////////////////////////////////////////////////////////

impl<'i, V, I> Appender<'i> for Series<'i, V, I>
    where V: Copy,
          I: Copy + Eq + Hash {

    fn append(&self, other: &Self) -> Self {
        let mut new_values: Vec<V> = self.values.clone();
        new_values.append(&mut other.values.clone());
        let new_index = self.index.append(&other.index);
        Series::new(new_values, new_index)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Apply
////////////////////////////////////////////////////////////////////////////////

impl<'i, V, I, R> Applicable<'i, Vec<V>, R, R> for Series<'i, V, I>
    where V: Copy,
          I: Copy + Eq + Hash {

    fn apply<'f>(&'i self, func: &'f Fn(&Vec<V>) -> R) -> R {
        func(&self.values)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Eq
////////////////////////////////////////////////////////////////////////////////

impl<'i, V: PartialEq, I: Clone + Hash + Eq> PartialEq for Series<'i, V, I> {
    fn eq(&self, other: &Self) -> bool {
        (self.index.eq(&other.index)) && (self.values.eq(&other.values))
    }
}

