extern crate itertools;

use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

pub struct Indexer<U: Hash> {
    // index must be hashable, note that float can't be hashed
    pub values: Vec<U>,
    pub label_mapper: HashMap<U, usize>,
}


impl<U: Copy + Eq + Hash> Indexer<U> {

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
        self.init_label_mapper();
        return self.label_mapper.contains_key(label);
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


// Formatting

impl<U> fmt::Display for Indexer<U>
    where U: Copy + Eq + Hash + fmt::Debug {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Index({:?})", &self.values);
    }

}

impl<U> fmt::Debug for Indexer<U>
    where U: Copy + Eq + Hash + fmt::Debug {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Index({:?})", &self.values);
    }

}


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
    fn test_index_slice_int64() {
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
    fn test_index_creation_str() {
        let values: Vec<&str> = vec!["A", "B", "C"];
        let idx = Indexer::<&str>::new(values);

        let exp_index: Vec<&str> = vec!["A", "B", "C"];
        assert_eq!(&idx.values, &exp_index);
        assert_eq!(&idx.len(), &3);
    }

    #[test]
    fn test_index_slice_str() {
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
}
