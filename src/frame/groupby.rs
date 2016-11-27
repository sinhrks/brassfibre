
use std::hash::Hash;

use super::DataFrame;
use super::super::algos::grouper::{Grouper, HashGrouper};
use super::super::traits::{RowIndexer};

pub struct DataFrameGroupBy<'a, D: 'a, G: Hash> {
    /// Grouped DataFrame
    /// D: grouped data
    /// G: type of Group indexer

    frame: &'a D,
    grouper: HashGrouper<G>,
}

impl<'a, U, V, G> DataFrameGroupBy<'a, DataFrame<U, V>, G>
    where U: Copy + Eq + Hash,
          V: Copy + Eq + Hash,
          G: Copy + Eq + Hash + Ord {

    pub fn new(frame: &'a DataFrame<U, V>, indexer: Vec<G>)
        -> DataFrameGroupBy<DataFrame<U, V>, G>{

        assert!(frame.len() == indexer.len(),
                "DataFrame and Indexer length are different");

        let grouper: HashGrouper<G> = HashGrouper::groupby(&indexer);

        DataFrameGroupBy {
            frame: frame,
            grouper: grouper,
        }
    }

    pub fn get_group(&self, group: &G) -> DataFrame<U, V> {

        if let Some(locs) = self.grouper.get(group) {
            self.frame.ilocs(&locs)
        } else {
            panic!("Group not found!");
        }
    }

    pub fn groups(&self) -> Vec<G> {
        let mut keys: Vec<G> = self.grouper.keys();
        keys.sort();
        keys
    }
}
