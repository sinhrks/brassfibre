use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

use super::algos::sort::Sorter;
use super::traits::{IndexerIndexer, Appender};

mod convert;
mod formatting;
mod indexing;
mod ops;
mod sort;

/// Immutable hash index
#[derive(Clone)]
pub struct Indexer<U: Hash> {
    // index must be hashable, note that float can't be hashed
    pub values: Vec<U>,

    // provides interior mutability
    htable: RefCell<HashMap<U, usize>>,
}

////////////////////////////////////////////////////////////////////////////////
// Constructor
////////////////////////////////////////////////////////////////////////////////

impl<U> Indexer<U> where U: Copy + Eq + Hash {

    pub fn from_len(len: usize) -> Indexer<usize> {
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

impl<U> IndexerIndexer for Indexer<U>  where U: Copy + Eq + Hash {

    type Key = U;

    fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether Indexer contains label or not
    fn contains(&self, label: &U) -> bool {
        self.init_state();
        self.htable.borrow().contains_key(label)
    }

    fn push(&mut self, label: U) {
        let loc = self.len();
        // ToDo: merge with init_label_mapper
        let mut mapper = self.htable.borrow_mut();
        if !mapper.contains_key(&label) {
            mapper.insert(label, loc);
        } else {
            // temp, do not allow duplicates for now
            panic!("Duplicated key!");
        }
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

    fn reindex(&self, locations: &Vec<usize>) -> Self {
        let new_values = Sorter::reindex(&self.values, locations);
        Indexer::new(new_values)
    }

    fn init_state(&self) {
        // update htable
        let mut htable = self.htable.borrow_mut();
        if htable.len() != 0 {
            return;
        }
        for (loc, label) in self.values.iter().enumerate() {
            if !htable.contains_key(label) {
                htable.insert(*label, loc);
            } else {
                // temp, do not allow duplicates for now
                panic!("Duplicated key!");
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Append
////////////////////////////////////////////////////////////////////////////////

impl<T> Appender for Indexer<T>
    where T: Copy + Eq + Hash {

    fn append(&self, other: &Self) -> Self {
        let mut new_values: Vec<T> = self.values.clone();
        new_values.append(&mut other.values.clone());
        Indexer::new(new_values)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Eq
////////////////////////////////////////////////////////////////////////////////

impl<U: Hash + Eq> PartialEq for Indexer<U> {
    fn eq(&self, other: &Indexer<U>) -> bool {
        self.values == other.values
    }
}
