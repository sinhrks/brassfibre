extern crate multimap;

use multimap::MultiMap;
use num::{Num, Zero, ToPrimitive};
use std::cmp::Ord;
use std::hash::Hash;

use super::block::Block;

pub struct BlockGroupBy<T, U: Hash, V: Hash, G: Hash> {
    /// Grouped Block
    /// T: type of Block values
    /// U: type of Block indexer
    /// V: type of Block columns
    /// G: type of Group indexer

    pub block: Block<T, U, V>,
    pub grouper: MultiMap<G, usize>,
}

impl<T, U, V, G> BlockGroupBy<T, U, V, G>
    where T: Copy,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash,
          G: Copy + Eq + Hash + Ord {

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
            return self.block.slice_by_index(&locs.clone());
        } else {
            panic!("Group not found!");
        }
    }

    pub fn groups(&self) -> Vec<G> {
        let mut keys: Vec<G> = self.grouper.keys().map(|x| *x).collect();
        keys.sort();
        return keys;
    }

    pub fn apply<W: Copy>(&self, func: &Fn(&Block<T, U, V>) -> Vec<W>) -> Block<W, G, V> {
        /*
        Apply passed function to each group.
        */
        let mut new_values: Vec<W> = vec![];

        let groups = self.groups();
        for g in groups.iter() {
            let s = self.get_group(&g);
            new_values.append(&mut func(&s));
        }
        return Block::from_row_vec(new_values, groups,
                                      self.block.columns.copy_values(), );
    }
}


impl<T, U, V, G> BlockGroupBy<T, U, V, G>
    where T: Copy + Eq + Hash + Num + Zero + ToPrimitive,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash,
          G: Copy + Eq + Hash + Ord {

    pub fn sum(&self) -> Block<T, G, V> {
        return self.apply(&|x: &Block<T, U, V>| x.sum().values);
    }

    pub fn count(&self) -> Block<usize, G, V> {
        return self.apply(&|x: &Block<T, U, V>| x.count().values);
    }

    pub fn mean(&self) -> Block<f64, G, V> {
        return self.apply(&|x: &Block<T, U, V>| x.mean().values);
    }

    pub fn var(&self) -> Block<f64, G, V> {
        return self.apply(&|x: &Block<T, U, V>| x.var().values);
    }

    pub fn unbiased_var(&self) -> Block<f64, G, V> {
        return self.apply(&|x: &Block<T, U, V>| x.unbiased_var().values);
    }

    pub fn std(&self) -> Block<f64, G, V> {
        return self.apply(&|x: &Block<T, U, V>| x.std().values);
    }

    pub fn unbiased_std(&self) -> Block<f64, G, V> {
        return self.apply(&|x: &Block<T, U, V>| x.unbiased_std().values);
    }

}

#[cfg(test)]
mod tests {

    use super::super::block::Block;

    #[test]
    fn test_block_get_group() {
        let values = vec![1, 2, 3, 4, 5,
                          6, 7, 8, 9, 10,
                          11, 12, 13, 14, 15];
        let b = Block::from_col_vec(values,
                                    vec!["A", "B", "C", "D", "E"],
                                    vec!["X", "Y", "Z"]);
        assert_eq!(&b.len(), &5);

        let bg = b.groupby(vec![1, 2, 1, 1, 2]);
        assert_eq!(&bg.groups().len(), &2);

        let mut b1 = bg.get_group(&1);
        assert_eq!(&b1.index.values, &vec!["A", "C", "D"]);
        assert_eq!(&b1.columns.values, &vec!["X", "Y", "Z"]);
        let c11 = b1.get_column_by_label(&"X");
        assert_eq!(&c11.values, &vec![1, 3, 4]);
        let c11 = b1.get_column_by_label(&"Y");
        assert_eq!(&c11.values, &vec![6, 8, 9]);
        let c11 = b1.get_column_by_label(&"Z");
        assert_eq!(&c11.values, &vec![11, 13, 14]);
    }

    #[test]
    fn test_block_agg() {
        let values = vec![1, 2, 3, 4, 5,
                          6, 7, 8, 9, 10,
                          11, 12, 13, 14, 15];
        let b = Block::from_col_vec(values,
                                    vec!["A", "B", "C", "D", "E"],
                                    vec!["X", "Y", "Z"]);
        assert_eq!(&b.len(), &5);

        let bg = b.groupby(vec![1, 2, 1, 1, 2]);
        let mut bsum = bg.sum();

        assert_eq!(&bsum.len(), &2);
        assert_eq!(&bsum.index.values, &vec![1, 2]);
        assert_eq!(&bsum.columns.values, &vec!["X", "Y", "Z"]);
        let c11 = bsum.get_column_by_label(&"X");
        assert_eq!(&c11.values, &vec![8, 7]);
        let c11 = bsum.get_column_by_label(&"Y");
        assert_eq!(&c11.values, &vec![23, 17]);
        let c11 = bsum.get_column_by_label(&"Z");
        assert_eq!(&c11.values, &vec![38, 27]);
    }
}