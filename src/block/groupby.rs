
use num::{Num, Zero, ToPrimitive};
use std::cmp::Ord;
use std::hash::Hash;

use super::Block;
use super::super::algos::groupby::{GroupBy, HashGroupBy};
use super::super::traits::RowIndexer;

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

    pub fn apply<W: Copy>(&self, func: &Fn(&Block<T, U, V>) -> Vec<W>) -> Block<W, G, V> {
        /*
        Apply passed function to each group.
        */
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


impl<'a, T, U, V, G> BlockGroupBy<'a, T, U, V, G>
    where T: Copy + Eq + Hash + Num + Zero + ToPrimitive,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash,
          G: Copy + Eq + Hash + Ord {

    pub fn sum(&self) -> Block<T, G, V> {
        self.apply(&|x: &Block<T, U, V>| x.sum().values)
    }

    pub fn count(&self) -> Block<usize, G, V> {
        self.apply(&|x: &Block<T, U, V>| x.count().values)
    }

    pub fn mean(&self) -> Block<f64, G, V> {
        self.apply(&|x: &Block<T, U, V>| x.mean().values)
    }

    pub fn var(&self) -> Block<f64, G, V> {
        self.apply(&|x: &Block<T, U, V>| x.var().values)
    }

    pub fn unbiased_var(&self) -> Block<f64, G, V> {
        self.apply(&|x: &Block<T, U, V>| x.unbiased_var().values)
    }

    pub fn std(&self) -> Block<f64, G, V> {
        self.apply(&|x: &Block<T, U, V>| x.std().values)
    }

    pub fn unbiased_std(&self) -> Block<f64, G, V> {
        self.apply(&|x: &Block<T, U, V>| x.unbiased_std().values)
    }

}

#[cfg(test)]
mod tests {

    use super::super::Block;

    #[test]
    fn test_block_get_group() {
        let values = vec![1, 2, 3, 4, 5,
                          6, 7, 8, 9, 10,
                          11, 12, 13, 14, 15];
        let b = Block::from_col_vec(values,
                                    vec!["A", "B", "C", "D", "E"],
                                    vec!["X", "Y", "Z"]);
        assert_eq!(b.len(), 5);

        let bg = b.groupby(vec![1, 2, 1, 1, 2]);
        assert_eq!(&bg.groups().len(), &2);

        let b1 = bg.get_group(&1);
        let exp = Block::from_col_vec(vec![1, 3, 4, 6, 8, 9, 11, 13, 14],
                                      vec!["A", "C", "D"],
                                      vec!["X", "Y", "Z"]);
        assert_eq!(b1, exp);
    }

    #[test]
    fn test_block_agg() {
        let values = vec![1, 2, 3, 4, 5,
                          6, 7, 8, 9, 10,
                          11, 12, 13, 14, 15];
        let b = Block::from_col_vec(values,
                                    vec!["A", "B", "C", "D", "E"],
                                    vec!["X", "Y", "Z"]);
        assert_eq!(b.len(), 5);

        let bg = b.groupby(vec![1, 2, 1, 1, 2]);
        let bsum = bg.sum();

        assert_eq!(bsum.len(), 2);
        let exp = Block::from_col_vec(vec![8, 7, 23, 17, 38, 27],
                                      vec![1, 2],
                                      vec!["X", "Y", "Z"]);
        assert_eq!(bsum, exp);
    }

    #[test]
    fn test_block_agg2() {
        let values = vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                          vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]];
        let b = Block::from_nested_vec(values,
                                       vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                                       vec!["X", "Y"]);
        assert_eq!(&b.len(), &10);

        let bg = b.groupby(vec![1, 1, 1, 2, 2, 2, 1, 1, 1, 2]);
        let bagg = bg.sum();

        // mean
        let exp = Block::from_col_vec(vec![30, 25, 60, 50],
                                      vec![1, 2],
                                      vec!["X", "Y"]);
        assert_eq!(bagg, exp);

        // count
        let bagg = bg.count();
        let exp = Block::from_col_vec(vec![6, 4, 6, 4],
                                      vec![1, 2],
                                      vec!["X", "Y"]);
        assert_eq!(bagg, exp);

        // var
        let bagg = bg.var();
        let exp = Block::from_col_vec(vec![9.666666666666666, 5.1875,
                                           38.666666666666664, 20.75],
                                      vec![1, 2],
                                      vec!["X", "Y"]);
        assert_eq!(bagg, exp);

        // unbiased var
        let bagg = bg.unbiased_var();
        let exp = Block::from_col_vec(vec![11.6, 6.916666666666667,
                                           46.4, 27.666666666666668],
                                      vec![1, 2],
                                      vec!["X", "Y"]);
        assert_eq!(bagg, exp);

        // std
        let bagg = bg.std();
        let exp = Block::from_col_vec(vec![3.1091263510296048, 2.277608394786075,
                                           6.2182527020592095, 4.55521678957215],
                                      vec![1, 2],
                                      vec!["X", "Y"]);
        assert_eq!(bagg, exp);

        // unbiased std
        let bagg = bg.unbiased_std();
        let exp = Block::from_col_vec(vec![3.40587727318528, 2.6299556396765835,
                                           6.81175454637056, 5.259911279353167],
                                      vec![1, 2],
                                      vec!["X", "Y"]);
        assert_eq!(bagg, exp);
    }
}