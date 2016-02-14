extern crate multimap;

use multimap::MultiMap;
use num::{Num, Zero, ToPrimitive};
use std::cmp::Ord;
use std::hash::Hash;

use super::block::Block;

pub struct BlockGroupBy<T, U: Hash, V: Hash, G: Hash> {
    /// Grouped Series
    /// T: type of Series values
    /// U: type of Series indexer
    /// V: type of Group indexer

    pub block: Block<T, U, V>,
    pub grouper: MultiMap<G, usize>,
}

impl<T: Copy, U: Copy + Eq + Hash, V: Copy + Eq + Hash, G: Copy + Eq + Hash + Ord> BlockGroupBy<T, U, V, G> {

    pub fn new(block: Block<T, U, V>, indexer: Vec<G>) -> BlockGroupBy<T, U, V, G>{

        if block.len() != indexer.len() {
            panic!("Block and Indexer length are different");
        }

        let mut mapper = MultiMap::new();

        for (loc, label) in indexer.iter().enumerate() {
            mapper.insert(*label, loc);
        }

        BlockGroupBy {
            block: block,
            grouper: mapper,
        }
    }

    pub fn get_group(&self, group: &G) -> Block<T, U, V> {

        if let Some(locs) = self.grouper.get_vec(group) {
            let values = locs.iter().map(|x| *x).collect();
            return self.block.slice_by_index(&values);
        } else {
            panic!("Group not found!");
        }
    }

    pub fn groups(&self) -> Vec<G> {
        let mut keys = vec![];
        for key in self.grouper.keys() {
            keys.push(*key)
        }
        // key is returned arbitrary order
        keys.sort();
        return keys;
    }
}