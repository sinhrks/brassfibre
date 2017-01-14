use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::iter::FromIterator;
use std::slice;
use std::vec;

use nullvec::prelude::dev::algos::Indexing;
use traits::{Slicer, IndexerIndex, Append};

mod convert;
mod formatting;
mod indexing;
mod ops;
mod sort;

/// Hash index
#[derive(Clone)]
pub struct Indexer<U: Clone + Hash> {
    // index must be hashable, note that float can't be hashed
    pub values: Vec<U>,

    // provides interior mutability
    // ToDo: use Cow?
    htable: RefCell<HashMap<U, usize>>,
}

/// /////////////////////////////////////////////////////////////////////////////
/// Constructor
/// /////////////////////////////////////////////////////////////////////////////

impl<U> Indexer<U>
    where U: Clone + Eq + Hash
{
    pub fn from_len(len: usize) -> Indexer<usize> {
        // ToDo: don't need hash if index is range-like
        (0..len).collect()
    }

    pub fn new(values: Vec<U>) -> Self {
        Indexer {
            values: values,
            htable: RefCell::new(HashMap::new()),
        }
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Indexing
/// /////////////////////////////////////////////////////////////////////////////

impl<U> Slicer for Indexer<U>
    where U: Clone + Eq + Hash
{
    type Scalar = U;

    fn len(&self) -> usize {
        self.values.len()
    }

    fn iloc(&self, location: &usize) -> Self::Scalar {
        self.values[*location].clone()
    }

    unsafe fn iloc_unchecked(&self, location: &usize) -> Self::Scalar {
        self.values.get_unchecked(*location).clone()
    }

    fn ilocs(&self, locations: &[usize]) -> Self {
        let new_values = Indexing::reindex(&self.values, locations);
        Indexer::new(new_values)
    }

    unsafe fn ilocs_unchecked(&self, locations: &[usize]) -> Self {
        let new_values = Indexing::reindex_unchecked(&self.values, locations);
        Indexer::new(new_values)
    }

    fn ilocs_forced(&self, locations: &[usize]) -> Self {
        unimplemented!()
    }

    fn blocs(&self, flags: &[bool]) -> Self {
        let new_values: Vec<U> = Indexing::blocs(&self.values, flags);
        Indexer::new(new_values)
    }
}

impl<U> IndexerIndex for Indexer<U>
    where U: Clone + Eq + Hash
{
    type Key = U;

    /// Whether Indexer contains label or not
    fn contains(&self, label: &U) -> bool {
        self.init_state();
        self.htable.borrow().contains_key(label)
    }

    fn push(&mut self, label: U) {
        let loc = self.len();
        // ToDo: merge with init_state
        let mut htable = self.htable.borrow_mut();
        match htable.entry(label.clone()) {
            Entry::Occupied(_) => panic!("duplicates are not allowed"),
            Entry::Vacant(e) => e.insert(loc),
        };
        self.values.push(label);
    }

    /// Return label location (usize) corresponding to given label (Scalar)
    fn get_loc(&self, label: &U) -> usize {
        self.init_state();
        *self.htable.borrow().get(label).unwrap()
    }

    /// Return label locations (Vector) corresponding to given labels (Vector)
    fn get_locs(&self, labels: &[U]) -> Vec<usize> {
        labels.iter().map(|label| self.get_loc(&label)).collect()
    }

    fn init_state(&self) {
        // update htable
        let mut htable = self.htable.borrow_mut();
        if htable.len() != 0 {
            return;
        }
        for (loc, label) in self.values.iter().enumerate() {
            match htable.entry(label.clone()) {
                Entry::Occupied(_) => panic!("duplicates are not allowed"),
                Entry::Vacant(e) => e.insert(loc),
            };
        }
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Append
/// /////////////////////////////////////////////////////////////////////////////

impl<'a, T> Append<'a> for Indexer<T>
    where T: Clone + Eq + Hash
{
    fn append(&self, other: &Self) -> Self {
        let mut new_values: Vec<T> = self.values.clone();
        new_values.append(&mut other.values.clone());
        Indexer::new(new_values)
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Eq
/// /////////////////////////////////////////////////////////////////////////////

impl<U> PartialEq for Indexer<U>
    where U: Clone + Eq + Hash
{
    fn eq(&self, other: &Indexer<U>) -> bool {
        self.values == other.values
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Iterator
/// /////////////////////////////////////////////////////////////////////////////

impl<U> IntoIterator for Indexer<U>
    where U: Clone + Eq + Hash
{
    type Item = U;
    type IntoIter = vec::IntoIter<U>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<U> Indexer<U>
    where U: Clone + Eq + Hash
{
    pub fn iter(&self) -> slice::Iter<U> {
        self.values.iter()
    }
}

impl<U> FromIterator<U> for Indexer<U>
    where U: Clone + Eq + Hash
{
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = U>
    {
        let values: Vec<U> = iter.into_iter().collect();
        Indexer::new(values)
    }
}
