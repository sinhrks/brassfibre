use std::hash::Hash;

use super::algos::sort::Sorter;
use super::eval::Applicable;
use super::indexer::{Indexer, IndexerIndexer};
use super::internals::Array;
use super::series::Series;
use super::traits::{RowIndexer, ColIndexer};

#[derive(Clone)]
pub struct DataFrame<U: Hash, V: Hash> {
    /// 2-dimentional block contains multiple type.
    /// U: type of indexer
    /// V: type of columns

    // Do not use #[derice(PartialEq)] as internals may not be comparable
    pub values: Vec<Array>,
    pub index: Indexer<U>,
    pub columns: Indexer<V>,
}

////////////////////////////////////////////////////////////////////////////////
// Indexing
////////////////////////////////////////////////////////////////////////////////

impl<U, V> RowIndexer<U> for DataFrame<U, V>
    where U: Copy + Eq + Hash,
          V: Copy + Eq + Hash {

    fn reindex(&mut self, labels: &Vec<U>) -> Self {
        let locations = self.index.get_locs(labels);
        self.reindex_by_index(&locations)
    }

    fn reindex_by_index(&self, locations: &Vec<usize>) -> Self {
        let new_index = self.index.reindex(locations);

        let mut new_values: Vec<Array> = Vec::with_capacity(self.columns.len());
        for current in self.values.iter() {
            let new_value = current.ilocs(locations);
            new_values.push(new_value);
        }
        DataFrame::from_vec(new_values,
                            new_index,
                            self.columns.clone())
    }

    fn blocs(&self, labels: &Vec<bool>) -> Self {
        unimplemented!()
        // ToDo: fix Series impl
    }
}

impl<U, V> ColIndexer<V, Array> for DataFrame<U, V>
    where U: Copy + Eq + Hash,
          V: Copy + Eq + Hash {

    fn get(&mut self, label: &V) -> Array {
        unimplemented!();
    }

    fn iget(&self, loc: &usize) -> Array {
        unimplemented!();
    }

    fn gets(&mut self, labels: &Vec<V>) -> Self {
        let locs = self.columns.get_locs(labels);
        self.igets(&locs)
    }

    fn igets(&self, locations: &Vec<usize>) -> Self {
        let new_columns = self.columns.reindex(locations);

        let mut new_values: Vec<Array> = Vec::with_capacity(new_columns.len());
        for loc in locations {
            new_values.push(self.values[*loc].clone());
        }
        DataFrame::from_vec(new_values, self.index.clone(), new_columns)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////

impl<U, V> DataFrame<U, V>
    where U: Copy + Eq + Hash,
          V: Copy + Eq + Hash {

    pub fn from_vec<I, C>(values: Vec<Array>,
                          index: I, columns: C) -> Self
        where I: Into<Indexer<U>>,
              C: Into<Indexer<V>> {

        let index: Indexer<U> = index.into();
        let columns: Indexer<V> = columns.into();

        if values.len() != columns.len() {
            panic!("Length mismatch!");
        }
        let len = index.len();
        for value in values.iter() {
            if value.len() != len {
                panic!("Length mismatch!");
            }
        }
        DataFrame {
            values: values,
            index: index,
            columns: columns,
        }
    }

    fn assert_binop(&self, other: &DataFrame<U, V>) {
        if self.index != other.index {
            panic!("index must be the same!");
        }
        if self.columns != other.columns {
            panic!("columns must be the same!");
        }
    }

    pub fn insert(&mut self, values: Array, name: V) {
        if self.len() != values.len() {
            panic!("Length mismatch!");
        }
        self.values.push(values);
        self.columns.push(name);
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }
}

impl<U: Hash + Eq, V: Hash + Eq> PartialEq for DataFrame<U, V> {
    fn eq(&self, other: &DataFrame<U, V>) -> bool {
        (self.index.eq(&other.index)) &&
        (self.columns.eq(&other.columns)) &&
        (self.values.eq(&other.values))
    }
}

#[cfg(test)]
mod tests {

    use super::DataFrame;
    use super::super::internals::Array;
    use super::super::traits::{RowIndexer, ColIndexer};

    #[test]
    fn test_block_creation_from_vec() {
        let values = vec![Array::Int64Array(vec![1, 2, 3, 4, 5]),
                          Array::Float64Array(vec![6., 7., 8., 9., 10.]),
                          Array::Int64Array(vec![11, 12, 13, 14, 15])];
        let df = DataFrame::from_vec(values,
                                     vec!["A", "BB", "CC", "D", "EEE"],
                                     vec!["X", "YYY", "ZZ"]);
        assert_eq!(df.len(), 5);
    }

    #[test]
    fn test_block_add_columns() {
        let values = vec![Array::Int64Array(vec![1, 2, 3]),
                          Array::Float64Array(vec![4., 5., 6.])];
        let mut df = DataFrame::from_vec(values,
                                         vec!["A", "BB", "CC"],
                                         vec!["X", "Y"]);
        assert_eq!(df.len(), 3);
        df.insert(Array::Int64Array(vec![10, 11, 12]), "Z");

        let exp_values = vec![Array::Int64Array(vec![1, 2, 3]),
                              Array::Float64Array(vec![4., 5., 6.]),
                              Array::Int64Array(vec![10, 11, 12])];
        let exp = DataFrame::from_vec(exp_values,
                                      vec!["A", "BB", "CC"],
                                      vec!["X", "Y", "Z"]);
        assert_eq!(df.values, exp.values);
        assert_eq!(df.index, exp.index);
        assert_eq!(df.columns, exp.columns);
    }

    #[test]
    fn test_block_slice_locs() {
        let values = vec![Array::Int64Array(vec![1, 2, 3, 4, 5]),
                          Array::Float64Array(vec![6., 7., 8., 9., 10.]),
                          Array::Int64Array(vec![11, 12, 13, 14, 15])];
        let mut df = DataFrame::from_vec(values,
                                         vec!["A", "BB", "CC", "D", "EEE"],
                                         vec!["X", "YYY", "ZZ"]);
        assert_eq!(df.len(), 5);

        let res = df.locs(&vec!["A", "D", "CC"]);
        let exp_values = vec![Array::Int64Array(vec![1, 4, 3]),
                              Array::Float64Array(vec![6., 9., 8.]),
                              Array::Int64Array(vec![11, 14, 13])];
        let exp = DataFrame::from_vec(exp_values,
                                      vec!["A", "D", "CC"],
                                      vec!["X", "YYY", "ZZ"]);
        assert_eq!(res.values, exp.values);
        assert_eq!(res.index, exp.index);
        assert_eq!(res.columns, exp.columns);
    }

    #[test]
    fn test_block_slice_ilocs() {
        let values = vec![Array::Int64Array(vec![1, 2, 3, 4, 5]),
                          Array::Float64Array(vec![6., 7., 8., 9., 10.]),
                          Array::Int64Array(vec![11, 12, 13, 14, 15])];
        let df = DataFrame::from_vec(values,
                                     vec!["A", "BB", "CC", "D", "EEE"],
                                     vec!["X", "YYY", "ZZ"]);
        assert_eq!(df.len(), 5);

        let res = df.ilocs(&vec![0, 3, 2]);
        let exp_values = vec![Array::Int64Array(vec![1, 4, 3]),
                              Array::Float64Array(vec![6., 9., 8.]),
                              Array::Int64Array(vec![11, 14, 13])];
        let exp = DataFrame::from_vec(exp_values,
                                      vec!["A", "D", "CC"],
                                      vec!["X", "YYY", "ZZ"]);
        assert_eq!(res.values, exp.values);
        assert_eq!(res.index, exp.index);
        assert_eq!(res.columns, exp.columns);
    }

    #[test]
    fn test_block_columns_slice() {
        let values = vec![Array::Int64Array(vec![1, 2, 3, 4, 5]),
                          Array::Float64Array(vec![6., 7., 8., 9., 10.]),
                          Array::Int64Array(vec![11, 12, 13, 14, 15])];
        let mut b = DataFrame::from_vec(values,
                                        vec!["A", "BB", "CC", "D", "EEE"],
                                        vec!["X", "YYY", "ZZ"]);

        let exp_values = vec![Array::Float64Array(vec![6., 7., 8., 9., 10.]),
                              Array::Int64Array(vec![1, 2, 3, 4, 5])];
        let exp = DataFrame::from_vec(exp_values,
                                      vec!["A", "BB", "CC", "D", "EEE"],
                                      vec!["YYY", "X"]);
        let res = b.gets(&vec!["YYY", "X"]);
        assert_eq!(res.values, exp.values);
        assert_eq!(res.index, exp.index);
        assert_eq!(res.columns, exp.columns);

        let res = b.igets(&vec![1, 0]);
        assert_eq!(res.values, exp.values);
        assert_eq!(res.index, exp.index);
        assert_eq!(res.columns, exp.columns);
    }

}
