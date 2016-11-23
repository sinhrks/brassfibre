use std::collections::HashMap;
use std::hash::Hash;

use super::algos::sort::Sorter;

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
    pub label_mapper: HashMap<U, usize>,
}

pub trait IndexerTrait<U: Hash> {
    fn len(&self) -> usize;
    fn contains(&mut self, label: &U) -> bool;
    fn push(&mut self, label: U);
    fn get_loc(&mut self, label: &U) -> usize;
    fn get_locs(&mut self, labels: &Vec<U>) -> Vec<usize>;
    fn reindex(&self, locations: &Vec<usize>) -> Self;

    // temp
    fn init_state(&mut self);
}

impl<U> Indexer<U> where U: Copy + Eq + Hash {

    pub fn from_len(len: usize) -> Indexer<usize> {
        let index: Vec<usize> = (0..len).collect();
        Indexer::new(index)
    }

    pub fn new(values: Vec<U>) -> Self {
        Indexer {
            values: values,
            label_mapper: HashMap::new(),
        }
    }
}

impl<U> IndexerTrait<U> for Indexer<U>  where U: Copy + Eq + Hash {

    fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether Indexer contains label or not
    fn contains(&mut self, label: &U) -> bool {
        self.init_state();
        self.label_mapper.contains_key(label)
    }

    fn push(&mut self, label: U) {
        let loc = self.len();
        // ToDo: merge with init_label_mapper
        if !self.label_mapper.contains_key(&label) {
            self.label_mapper.insert(label, loc);
        } else {
            // temp, do not allow duplicates for now
            panic!("Duplicated key!");
        }
        self.values.push(label);
    }

    /// Return label location (usize) corresponding to given label (Scalar)
    fn get_loc(&mut self, label: &U) -> usize {
        self.init_state();
        *self.label_mapper.get(label).unwrap()
    }

    /// Return label locations (Vector) corresponding to given labels (Vector)
    fn get_locs(&mut self, labels: &Vec<U>) -> Vec<usize> {
        labels.iter().map(|label| self.get_loc(&label)).collect()
    }

    fn reindex(&self, locations: &Vec<usize>) -> Self {
        let new_values = Sorter::reindex(&self.values, locations);
        Indexer::new(new_values)
    }

    fn init_state(&mut self) {
        // update label_mapper
        if self.label_mapper.len() != 0 {
            return;
        }
        for (loc, label) in self.values.iter().enumerate() {
            if !self.label_mapper.contains_key(label) {
                self.label_mapper.insert(*label, loc);
            } else {
                // temp, do not allow duplicates for now
                panic!("Duplicated key!");
            }
        }
    }
}

//**********************************************
// Equality
//**********************************************

impl<U: Hash + Eq> PartialEq for Indexer<U> {
    fn eq(&self, other: &Indexer<U>) -> bool {
        self.values == other.values
    }
}

#[cfg(test)]
mod tests {

    use super::{Indexer, IndexerTrait};

    #[test]
    fn test_index_creation_from_len() {
        let idx: Indexer<usize> = Indexer::<usize>::from_len(3);
        assert_eq!(idx.values, vec![0, 1, 2]);
        assert_eq!(idx.len(), 3);

        let idx: Indexer<usize> = Indexer::<usize>::from_len(0);
        assert_eq!(idx.values, vec![]);
        assert_eq!(idx.len(), 0);
    }

    #[test]
    fn test_index_creation_int64() {
        let values: Vec<i64> = vec![1, 2, 3];
        let idx = Indexer::<i64>::new(values);

        let exp_index: Vec<i64> = vec![1, 2, 3];
        assert_eq!(idx.values, exp_index);
        assert_eq!(idx.len(), 3);
    }

    #[test]
    fn test_index_loc_int64() {
        let values: Vec<i64> = vec![1, 2, 3];
        let mut idx = Indexer::<i64>::new(values);

        assert_eq!(idx.get_loc(&1), 0);
        assert_eq!(idx.get_loc(&3), 2);

        assert_eq!(idx.get_locs(&vec![1, 3]), vec![0, 2]);
        assert_eq!(idx.get_locs(&vec![3, 2]), vec![2, 1]);

        assert_eq!(idx.contains(&1), true);
        assert_eq!(idx.contains(&5), false);
    }

    #[test]
    fn test_index_creation_str() {
        let values: Vec<&str> = vec!["A", "B", "C"];
        let idx = Indexer::<&str>::new(values);

        let exp_index: Vec<&str> = vec!["A", "B", "C"];
        assert_eq!(idx.values, exp_index);
        assert_eq!(idx.len(), 3);
    }

    #[test]
    fn test_index_loc_str() {
        let values: Vec<&str> = vec!["A", "B", "C"];
        let mut idx = Indexer::<&str>::new(values);

        assert_eq!(idx.get_loc(&"B"), 1);
        assert_eq!(idx.get_loc(&"C"), 2);

        assert_eq!(idx.get_locs(&vec!["B", "C"]), vec![1, 2]);
        assert_eq!(idx.get_locs(&vec!["A", "C"]), vec![0, 2]);

        assert_eq!(idx.contains(&"C"), true);
        assert_eq!(idx.contains(&"X"), false);
    }

    #[test]
    fn test_copy() {
        let values: Vec<&str> = vec!["A", "B", "C"];
        let idx = Indexer::<&str>::new(values);

        // copy Indexer
        let copied = idx.clone();
        let exp_values: Vec<&str> = vec!["A", "B", "C"];
        assert_eq!(&copied.values, &exp_values);
    }

    #[test]
    fn test_equals() {
        let idx = Indexer::<&str>::new(vec!["A", "B", "C"]);

        let other = Indexer::<&str>::new(vec!["A", "B"]);
        assert_eq!(idx == other, false);

        let other = Indexer::<&str>::new(vec!["A", "B", "X"]);
        assert_eq!(idx == other, false);

        let other = Indexer::<&str>::new(vec!["A", "B", "C"]);
        assert_eq!(idx == other, true);
        assert_eq!(idx, other);
    }

    #[test]
    fn test_index_push() {
        let values: Vec<&str> = vec!["A", "B", "C"];
        let mut idx = Indexer::<&str>::new(values);

        let exp_index: Vec<&str> = vec!["A", "B", "C"];
        assert_eq!(idx.values, exp_index);
        assert_eq!(idx.len(), 3);
        assert_eq!(idx.get_loc(&"C"), 2);

        idx.push("D");
        assert_eq!(idx.len(), 4);
        assert_eq!(idx.get_loc(&"C"), 2);
        assert_eq!(idx.get_loc(&"D"), 3);

        idx.push("E");
        assert_eq!(idx.len(), 5);
        assert_eq!(idx.get_loc(&"D"), 3);
        assert_eq!(idx.get_loc(&"E"), 4);
    }

    #[test]
    fn test_reindex() {
        let idx = Indexer::<&str>::new(vec!["A", "B", "C"]);

        let res = idx.reindex(&vec![1, 0, 2]);
        assert_eq!(res, Indexer::new(vec!["B", "A", "C"]));

        let res = idx.reindex(&vec![1, 0, 2]);
        assert_eq!(res, Indexer::new(vec!["B", "A", "C"]));

    }
}
