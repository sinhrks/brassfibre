extern crate itertools;

use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Index;

mod formatting;
mod ops;

pub struct Indexer<U: Hash> {
    // index must be hashable, note that float can't be hashed
    pub values: Vec<U>,
    pub label_mapper: HashMap<U, usize>,
}


impl<U> Indexer<U>
    where U: Copy + Eq + Hash {

    pub fn from_len(len: usize) -> Indexer<i64> {
        let mut index: Vec<i64> = vec![];
        for i in 0..len as i64 {
            index.push(i);
        }
        Indexer {
            values: index,
            label_mapper: HashMap::new(),
        }
    }

    pub fn new(values: Vec<U>) -> Indexer<U> {
        Indexer {
            values: values,
            label_mapper: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        return self.values.len();
    }

    pub fn copy_values(&self) -> Vec<U> {
        return self.values.clone();
    }

    pub fn copy(&self) -> Indexer<U> {
        return Indexer::new(self.copy_values());
    }

    pub fn contains(&mut self, label: &U) -> bool {
        /*
        Whether Indexer contains label or not
        */
        self.init_label_mapper();
        return self.label_mapper.contains_key(label);
    }

    pub fn equals(&self, other: &Indexer<U>) -> bool
        where U: PartialEq {
        /*
        Whether Indexer is equal to other
        */
        return self.values == other.values;
    }

    pub fn push(&mut self, label: U) {
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

    pub fn get_label_loc(&mut self, label: &U) -> usize {
        /*
        return label location (usize) corresponding to given label (Scalar).
        */
        self.init_label_mapper();
        return *self.label_mapper.get(label).unwrap()
    }

    pub fn slice_label_loc(&mut self, labels: &Vec<U>) -> Vec<usize> {
        /*
        return label locations (Vector) corresponding to given labels (Vector).
        */
        return labels.iter().map(|label| self.get_label_loc(&label)).collect();
    }

    fn init_label_mapper(&mut self) {
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

// Indexing

impl<U> Index<usize> for Indexer<U>
    where U: Copy + Eq + Hash {
    /*
    Location (usize) Indexing
    */
    type Output = U;
    fn index(&self, index: usize) -> &U {
        return &self.values[index];
    }
}

/*
impl<'a, U> Index<&'a Vec<bool>> for &'a Indexer<U>
    where U: Copy + Eq + Hash {
    /*
    Boolian Indexing
    */
    type Output = Vec<U>;
    fn index<'b>(&'b self, index: &Vec<bool>) -> &'b Vec<U> {
        if self.len() != index.len() {
            panic!("Length Mismatch!");
        }
        let mut new_values: Vec<U> = vec![];
        for (&v, &flag) in self.values.iter().zip(index.iter()) {
            if flag {
                new_values.push(v);
            }
        }
        let idx: &'a Indexer<U> = &Indexer::new(*new_values);
        return &idx;
    }
}
*/

// Sorting

/*
impl<U> Indexer<U>
    where U: Copy + Eq + Hash + Ord {

    pub fn argsort(&self) -> Vec<usize> {
        let mut indexer: Vec<usize> = (0..self.len()).collect();
        // let mut slice = &indexer[..];
        // slice.sort_by_key(|k| k.abs());
        return indexer;
    }

    pub fn sort(&self) -> Indexer<U> {
        let indexer = self.argsort();
        let mut new_values = vec![];
        for i in indexer.iter() {
            new_values.push(self[*i]);
        }
        return Indexer::new(new_values);
    }
}
*/


#[cfg(test)]
mod tests {

    use super::Indexer;

    #[test]
    fn test_index_creation_int64() {
        let values: Vec<i64> = vec![1, 2, 3];
        let idx = Indexer::<i64>::new(values);

        let exp_index: Vec<i64> = vec![1, 2, 3];
        assert_eq!(&idx.values, &exp_index);
        assert_eq!(&idx.len(), &3);
    }

    #[test]
    fn test_index_loc_int64() {
        let values: Vec<i64> = vec![1, 2, 3];
        let mut idx = Indexer::<i64>::new(values);

        assert_eq!(&idx.get_label_loc(&1), &0);
        assert_eq!(&idx.get_label_loc(&3), &2);

        assert_eq!(&idx.slice_label_loc(&vec![1, 3]), &vec![0, 2]);
        assert_eq!(&idx.slice_label_loc(&vec![3, 2]), &vec![2, 1]);

        assert_eq!(&idx.contains(&1), &true);
        assert_eq!(&idx.contains(&5), &false);
    }

    #[test]
    fn test_indexing_int64() {
        let values: Vec<i64> = vec![1, 2, 3];
        let idx = Indexer::<i64>::new(values);

        assert_eq!(&idx[2], &3);
        assert_eq!(&idx[0], &1);

        // let res = *idx[&vec![true, false, true]];
        // assert_eq!(&res.values, &vec![1, 3]);
    }

    #[test]
    fn test_index_creation_str() {
        let values: Vec<&str> = vec!["A", "B", "C"];
        let idx = Indexer::<&str>::new(values);

        let exp_index: Vec<&str> = vec!["A", "B", "C"];
        assert_eq!(&idx.values, &exp_index);
        assert_eq!(&idx.len(), &3);
    }

    #[test]
    fn test_index_loc_str() {
        let values: Vec<&str> = vec!["A", "B", "C"];
        let mut idx = Indexer::<&str>::new(values);

        assert_eq!(&idx.get_label_loc(&"B"), &1);
        assert_eq!(&idx.get_label_loc(&"C"), &2);

        assert_eq!(&idx.slice_label_loc(&vec!["B", "C"]), &vec![1, 2]);
        assert_eq!(&idx.slice_label_loc(&vec!["A", "C"]), &vec![0, 2]);

        assert_eq!(&idx.contains(&"C"), &true);
        assert_eq!(&idx.contains(&"X"), &false);
    }

    #[test]
    fn test_indexing_str() {
        let values: Vec<&str> = vec!["A", "B", "C"];
        let idx = Indexer::<&str>::new(values);

        assert_eq!(&idx[2], &"C");
        assert_eq!(&idx[0], &"A");

        // let res = *idx[&vec![true, false, true]];
        // assert_eq!(&res.values, &vec!["A", "C"]);
    }

    #[test]
    fn test_copy() {
        let values: Vec<&str> = vec!["A", "B", "C"];
        let idx = Indexer::<&str>::new(values);

        // copy values as Vec
        let copied = idx.copy_values();
        let exp_values: Vec<&str> = vec!["A", "B", "C"];
        assert_eq!(&copied, &exp_values);

        // copy Indexer
        let copied = idx.copy();
        let exp_values: Vec<&str> = vec!["A", "B", "C"];
        assert_eq!(&copied.values, &exp_values);
    }

    #[test]
    fn test_equals() {
        let idx = Indexer::<&str>::new(vec!["A", "B", "C"]);

        let other = Indexer::<&str>::new(vec!["A", "B"]);
        assert_eq!(&idx.equals(&other), &false);

        let other = Indexer::<&str>::new(vec!["A", "B", "X"]);
        assert_eq!(&idx.equals(&other), &false);

        let other = Indexer::<&str>::new(vec!["A", "B", "C"]);
        assert_eq!(&idx.equals(&other), &true);
    }

    #[test]
    fn test_index_push() {
        let values: Vec<&str> = vec!["A", "B", "C"];
        let mut idx = Indexer::<&str>::new(values);

        let exp_index: Vec<&str> = vec!["A", "B", "C"];
        assert_eq!(&idx.values, &exp_index);
        assert_eq!(&idx.len(), &3);
        assert_eq!(&idx.get_label_loc(&"C"), &2);

        idx.push("D");
        assert_eq!(&idx.len(), &4);
        assert_eq!(&idx.get_label_loc(&"C"), &2);
        assert_eq!(&idx.get_label_loc(&"D"), &3);

        idx.push("E");
        assert_eq!(&idx.len(), &5);
        assert_eq!(&idx.get_label_loc(&"D"), &3);
        assert_eq!(&idx.get_label_loc(&"E"), &4);
    }
}
