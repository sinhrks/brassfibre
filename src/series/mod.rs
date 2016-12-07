use std::borrow::Cow;
use std::hash::Hash;
use std::slice;
use std::vec;

use super::algos::indexing::Indexing;
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
          I: Clone + Eq + Hash {

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
        let new_values: Vec<Self::Row> = Indexing::blocs(&self.values, flags);
        let new_index = self.index.blocs(flags);
        Series::new(new_values, new_index)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////

impl<'i, V, I> Series<'i, V, I>
    where V: Copy,
          I: 'i + Clone + Eq + Hash {

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

        assert!(values.len() == index.len(), "Length mismatch!");

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
          I: Clone + Eq + Hash {

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

impl<'i, V, I, R> Applicable<'i, R> for Series<'i, V, I>
    where V: Copy,
          I: Clone + Eq + Hash {

    type In = Vec<V>;
    type FOut = R;
    type Out = R;

    fn apply<'f>(&'i self, func: &'f Fn(&Vec<V>) -> R) -> R {
        func(&self.values)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Eq
////////////////////////////////////////////////////////////////////////////////

impl<'i, V, I> PartialEq for Series<'i, V, I>
    where V: PartialEq,
          I: Clone + Hash + Eq {

    fn eq(&self, other: &Self) -> bool {
        (self.index.eq(&other.index)) && (self.values.eq(&other.values))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Iterator
////////////////////////////////////////////////////////////////////////////////

impl<'i, V, I> IntoIterator for Series<'i, V, I>
    where I: Clone + Eq + Hash {

    type Item = V;
    type IntoIter = vec::IntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'i, V, I> Series<'i, V, I>
    where I: Clone + Eq + Hash {

    pub fn iter(&self) -> slice::Iter<V> {
        self.values.iter()
    }
}
