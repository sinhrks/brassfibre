use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::slice;
use std::vec;

use super::algos::indexing::Indexing;
use super::algos::sort::Sorter;
use super::traits::{Slicer, IndexerIndexer, Appender};

mod convert;
mod formatting;
mod indexing;
mod ops;
mod sort;

/// Immutable hash index
#[derive(Clone)]
pub struct Indexer<U: Clone + Hash> {
    // index must be hashable, note that float can't be hashed
    pub values: Vec<U>,

    // provides interior mutability
    htable: RefCell<HashMap<U, usize>>,
}

////////////////////////////////////////////////////////////////////////////////
// Constructor
////////////////////////////////////////////////////////////////////////////////

impl<U> Indexer<U> where U: Clone + Eq + Hash {

    pub fn from_len(len: usize) -> Indexer<usize> {
        // ToDo: don't need hash if index is range-like
        let index: Vec<usize> = (0..len).collect();
        Indexer::new(index)
    }

    pub fn new(values: Vec<U>) -> Self {
        Indexer {
            values: values,
            htable: RefCell::new(HashMap::new()),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Indexing
////////////////////////////////////////////////////////////////////////////////

impl<U> Slicer for Indexer<U> where U: Clone + Eq + Hash {

    fn len(&self) -> usize {
        self.values.len()
    }

    fn ilocs(&self, locations: &Vec<usize>) -> Self {
        let new_values = Sorter::reindex(&self.values, locations);
        Indexer::new(new_values)
    }

    fn blocs(&self, flags: &Vec<bool>) -> Self {
        let new_values: Vec<U> = Indexing::blocs(&self.values, flags);
        Indexer::new(new_values)
    }
}

impl<U> IndexerIndexer for Indexer<U> where U: Clone + Eq + Hash {

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
    fn get_locs(&self, labels: &Vec<U>) -> Vec<usize> {
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

////////////////////////////////////////////////////////////////////////////////
// Append
////////////////////////////////////////////////////////////////////////////////

impl<'a, T> Appender<'a> for Indexer<T>
    where T: Clone + Eq + Hash {

    fn append(&self, other: &Self) -> Self {
        let mut new_values: Vec<T> = self.values.clone();
        new_values.append(&mut other.values.clone());
        Indexer::new(new_values)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Eq
////////////////////////////////////////////////////////////////////////////////

impl<U> PartialEq for Indexer<U>
    where U: Clone + Eq + Hash {
    fn eq(&self, other: &Indexer<U>) -> bool {
        self.values == other.values
    }
}

////////////////////////////////////////////////////////////////////////////////
// Iterator
////////////////////////////////////////////////////////////////////////////////

impl<U> IntoIterator for Indexer<U>
    where U: Clone + Eq + Hash {

    type Item = U;
    type IntoIter = vec::IntoIter<U>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<U> Indexer<U>
    where U: Clone + Eq + Hash {

    pub fn iter(&self) -> slice::Iter<U> {
        self.values.iter()
    }
}
