use std::borrow::{Borrow, Cow};
use std::hash::Hash;

use super::DataFrame;
use super::super::algos::grouper::{Grouper, HashGrouper};
use super::super::traits::{RowIndexer};

pub struct DataFrameGroupBy<'d, D: 'd, G: Hash> {
    /// Grouped DataFrame
    /// D: grouped data
    /// G: type of Group indexer

    frame: &'d D,
    grouper: HashGrouper<G>,
}

impl<'i, 'c, 'd, I, C, G> DataFrameGroupBy<'d, DataFrame<'i, 'c, I, C>, G>
    where I: Copy + Eq + Hash,
          C: Copy + Eq + Hash,
          G: Copy + Eq + Hash + Ord {

    pub fn new(frame: &'d DataFrame<'i, 'c, I, C>, indexer: Vec<G>)
        -> Self {

        assert!(frame.len() == indexer.len(),
                "DataFrame and Indexer length are different");

        let grouper: HashGrouper<G> = HashGrouper::groupby(&indexer);

        DataFrameGroupBy {
            frame: frame,
            grouper: grouper,
        }
    }

    pub fn get_group(&self, group: &G) -> DataFrame<I, C> {

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
