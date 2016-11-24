
use num::{Num, Zero, ToPrimitive};
use std::cmp::Ord;
use std::hash::Hash;

use super::Block;
use super::super::algos::groupby::{GroupBy, HashGroupBy};
use super::super::traits::{RowIndexer, Applicable, Aggregator};

pub struct BlockGroupBy<'a, T: 'a, U: 'a + Hash, V: 'a + Hash, G: 'a + Hash> {
    /// Grouped Block
    /// T: type of Block values
    /// U: type of Block indexer
    /// V: type of Block columns
    /// G: type of Group indexer

    pub block: &'a Block<T, U, V>,
    pub grouper: HashGroupBy<G>,
}

impl<'a, T, U, V, G> BlockGroupBy<'a, T, U, V, G>
    where T: Copy,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash,
          G: Copy + Eq + Hash + Ord {

    pub fn new(block: &'a Block<T, U, V>, indexer: Vec<G>) -> BlockGroupBy<T, U, V, G>{

        if block.len() != indexer.len() {
            panic!("Block and Indexer length are different");
        }

        let grouper: HashGroupBy<G> = HashGroupBy::groupby(&indexer);

        BlockGroupBy {
            block: block,
            grouper: grouper,
        }
    }

    pub fn get_group(&self, group: &G) -> Block<T, U, V> {

        if let Some(locs) = self.grouper.get(group) {
            self.block.ilocs(&locs)
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

////////////////////////////////////////////////////////////////////////////////
// Apply
////////////////////////////////////////////////////////////////////////////////

impl<'a, T, U, V, G, W> Applicable<Block<T, U, V>, Vec<W>, Block<W, G, V>>
    for BlockGroupBy<'a, T, U, V, G>

    where T: Copy,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash,
          G: Copy + Eq + Hash + Ord,
          W: Copy {

    /// Apply passed function to each group
    fn apply(&self, func: &Fn(&Block<T, U, V>) -> Vec<W>) -> Block<W, G, V> {
        let mut new_values: Vec<W> = Vec::with_capacity(self.grouper.len());

        let groups = self.groups();
        for g in groups.iter() {
            let s = self.get_group(&g);
            new_values.append(&mut func(&s));
        }
        Block::from_row_vec(new_values, groups,
                            self.block.columns.clone())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Aggregation
////////////////////////////////////////////////////////////////////////////////

impl<'a, T, U, V, G> Aggregator for BlockGroupBy<'a, T, U, V, G>
    where T: Copy + Eq + Hash + Num + Zero + ToPrimitive,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash,
          G: Copy + Eq + Hash + Ord {

    type Kept = Block<T, G, V>;
    type Counted = Block<usize, G, V>;
    type Coerced = Block<f64, G, V>;

    fn sum(&self) -> Block<T, G, V> {
        self.apply(&|x: &Block<T, U, V>| x.sum().values)
    }

    fn count(&self) -> Block<usize, G, V> {
        self.apply(&|x: &Block<T, U, V>| x.count().values)
    }

    fn mean(&self) -> Block<f64, G, V> {
        self.apply(&|x: &Block<T, U, V>| x.mean().values)
    }

    fn var(&self) -> Block<f64, G, V> {
        self.apply(&|x: &Block<T, U, V>| x.var().values)
    }

    fn unbiased_var(&self) -> Block<f64, G, V> {
        self.apply(&|x: &Block<T, U, V>| x.unbiased_var().values)
    }

    fn std(&self) -> Block<f64, G, V> {
        self.apply(&|x: &Block<T, U, V>| x.std().values)
    }

    fn unbiased_std(&self) -> Block<f64, G, V> {
        self.apply(&|x: &Block<T, U, V>| x.unbiased_std().values)
    }

}