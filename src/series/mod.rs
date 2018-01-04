use std::borrow::Cow;
use std::hash::Hash;
use std::iter::FromIterator;
use std::slice;
use std::vec;

use nullvec::prelude::dev::algos::Indexing;
use indexer::Indexer;
use groupby::GroupBy;
use traits::{Slicer, IndexerIndex, RowIndex, Append, Apply};

mod aggregation;
mod convert;
mod formatting;
mod groupby;
mod ops;
mod sort;

#[derive(Clone)]
pub struct Series<'v, 'i, V, I>
where
    V: 'v + Clone,
    I: 'i + Clone + Hash,
{
    pub values: Cow<'v, Vec<V>>,
    pub index: Cow<'i, Indexer<I>>,
}

/// /////////////////////////////////////////////////////////////////////////////
/// Indexing
/// /////////////////////////////////////////////////////////////////////////////

impl<'v, 'i, V, I> RowIndex<'i> for Series<'v, 'i, V, I>
where
    V: Clone,
    I: Clone + Eq + Hash,
{
    type Key = I;
    type Row = V;

    fn len(&self) -> usize {
        self.values.len()
    }

    fn loc(&self, label: &Self::Key) -> Self::Row {
        let loc = self.index.get_loc(label);
        self.iloc(&loc)
    }

    fn iloc(&self, location: &usize) -> Self::Row {
        self.values[*location].clone()
    }

    fn reindex(&self, labels: &[Self::Key]) -> Self {
        let locations = self.index.get_locs(labels);

        let new_index = self.index.reindex(&locations);
        let new_values = unsafe { Indexing::reindex_unchecked(&self.values, &locations) };
        Series::new(new_values, new_index)
    }

    fn reindex_by_index(&self, locations: &[usize]) -> Self {
        let new_index = self.index.reindex(locations);
        let new_values = unsafe { Indexing::reindex_unchecked(&self.values, locations) };
        Series::new(new_values, new_index)
    }

    /// Slice using given Vec<bool> (slice by Bool LOCationS)
    fn blocs(&self, flags: &[bool]) -> Self {
        let new_values: Vec<Self::Row> = Indexing::blocs(&self.values, flags);
        let new_index = self.index.blocs(flags);
        Series::new(new_values, new_index)
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Misc
/// /////////////////////////////////////////////////////////////////////////////

impl<'v, 'i, V, I> Series<'v, 'i, V, I>
where
    V: 'v + Clone,
    I: 'i + Clone + Eq + Hash,
{
    pub fn from_vec(values: Vec<V>) -> Series<'v, 'i, V, usize> {
        let index: Indexer<usize> = Indexer::<usize>::from_len(values.len());

        Series {
            values: Cow::Owned(values),
            index: Cow::Owned(index),
        }
    }

    pub fn new<X>(values: Vec<V>, index: X) -> Self
    where
        X: Into<Indexer<I>>,
    {

        let index: Indexer<I> = index.into();

        assert!(values.len() == index.len(), "Length mismatch!");

        Series {
            values: Cow::Owned(values),
            index: Cow::Owned(index),
        }
    }

    pub fn from_cow(values: Cow<'v, Vec<V>>, index: Cow<'i, Indexer<I>>) -> Self {

        assert!(values.len() == index.len(), "Length mismatch!");

        Series {
            values: values,
            index: index,
        }
    }

    fn assert_binop(&self, other: &Self) {
        assert!(self.index == other.index, "index must be the same!");
    }

    pub fn groupby<G>(&self, other: &[G]) -> GroupBy<Series<V, I>, G>
    where
        G: Clone + Eq + Hash + Ord,
    {
        GroupBy::new(self, other)
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Append
/// /////////////////////////////////////////////////////////////////////////////

impl<'v, 'i, V, I> Append<'i> for Series<'v, 'i, V, I>
where
    V: Clone,
    I: Clone + Eq + Hash,
{
    fn append(&self, other: &Self) -> Self {
        // clone COW (not values, then into_owned())
        let mut new_values: Vec<V> = self.values.clone().into_owned();
        // clone COW (not values, then to_mut())
        new_values.append(&mut other.values.clone().to_mut());
        let new_index = self.index.append(&other.index);
        Series::new(new_values, new_index)
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Apply
/// /////////////////////////////////////////////////////////////////////////////

impl<'v, 'i, V, I, R> Apply<'i, R> for Series<'v, 'i, V, I>
where
    V: 'i + Clone,
    I: Clone + Eq + Hash,
{
    type In = Vec<V>;
    type FOut = R;
    type Out = R;

    fn apply<'f>(&'i self, func: &'f Fn(&Self::In) -> Self::FOut) -> Self::Out {
        func(&self.values)
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Eq
/// /////////////////////////////////////////////////////////////////////////////

impl<'v, 'i, V, I> PartialEq for Series<'v, 'i, V, I>
where
    V: Clone + PartialEq,
    I: Clone + Hash + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        (self.index.eq(&other.index)) && (self.values.eq(&other.values))
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Iterator
/// /////////////////////////////////////////////////////////////////////////////

impl<'v, 'i, V, I> IntoIterator for Series<'v, 'i, V, I>
where
    V: Clone,
    I: Clone + Eq + Hash,
{
    type Item = V;
    type IntoIter = vec::IntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_owned().into_iter()
    }
}

impl<'v, 'i, V, I> Series<'v, 'i, V, I>
where
    V: Clone,
    I: Clone + Eq + Hash,
{
    pub fn iter(&self) -> slice::Iter<V> {
        self.values.as_ref().iter()
    }
}

impl<'v, 'i, V> FromIterator<V> for Series<'v, 'i, V, usize>
where
    V: Clone,
{
    fn from_iter<T>(iter: T) -> Series<'v, 'i, V, usize>
    where
        T: IntoIterator<Item = V>,
    {

        let values: Vec<V> = iter.into_iter().collect();
        Series::<V, usize>::from_vec(values)
    }
}
