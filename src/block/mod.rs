use std::hash::Hash;

use super::algos::sort::Sorter;
use super::eval::Applicable;
use super::indexer::{Indexer, IndexerIndexer};
use super::series::Series;
use super::traits::{RowIndexer, ColIndexer};

mod aggregation;
mod formatting;
mod groupby;
mod ops;

#[derive(Clone)]
pub struct Block<T, U: Hash, V: Hash> {
    /// 2-dimentional block contains a single type.
    /// T: type of values
    /// U: type of indexer
    /// V: type of columns

    // ToDo: may be simpler to use 1-d Vec?
    pub values: Vec<Vec<T>>,
    pub index: Indexer<U>,
    pub columns: Indexer<V>,
}

////////////////////////////////////////////////////////////////////////////////
// Indexing
////////////////////////////////////////////////////////////////////////////////

impl<T, U, V> RowIndexer<U> for Block<T, U, V>
    where T: Copy,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash {

    fn reindex(&mut self, labels: &Vec<U>) -> Self {
        let locations = self.index.get_locs(labels);
        self.reindex_by_index(&locations)
    }

    fn reindex_by_index(&self, locations: &Vec<usize>) -> Self {
        let new_index = self.index.reindex(locations);

        let mut new_values: Vec<Vec<T>> = Vec::with_capacity(self.columns.len());
        for current in self.values.iter() {
            let new_value = Sorter::reindex(current, locations);
            new_values.push(new_value);
        }
        Block::from_vec(new_values, new_index, self.columns.clone())
    }

    fn blocs(&self, labels: &Vec<bool>) -> Self {
        unimplemented!()
        // ToDo: fix Series impl
    }
}

impl<T, U, V> ColIndexer<V, Series<T, U>> for Block<T, U, V>
    where T: Copy,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash {

    fn get(&mut self, label: &V) -> Series<T, U> {
        let loc = self.columns.get_loc(label);
        self.iget(&loc)
    }

    fn iget(&self, loc: &usize) -> Series<T, U> {
        let new_values = self.values[*loc].clone();
        Series::new(new_values, self.index.clone())
    }

    fn gets(&mut self, labels: &Vec<V>) -> Self {
        let locs = self.columns.get_locs(labels);
        self.igets(&locs)
    }

    fn igets(&self, locations: &Vec<usize>) -> Self {
        let new_columns = self.columns.reindex(locations);

        let mut new_values: Vec<Vec<T>> = Vec::with_capacity(new_columns.len());
        for loc in locations {
            new_values.push(self.values[*loc].clone());
        }
        Block::from_vec(new_values, self.index.clone(), new_columns)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////

impl<T, U, V> Block<T, U, V>
    where T: Copy,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash {

    /// Instanciate from column-wise Vec
    pub fn from_col_vec<I, C>(values: Vec<T>, index: I, columns: C) -> Self
        where I: Into<Indexer<U>>,
              C: Into<Indexer<V>> {

        let index: Indexer<U> = index.into();
        let columns: Indexer<V> = columns.into();

        let len: usize = index.len();
        let cols: usize = columns.len();

        if values.len() != len * cols {
            panic!("Length mismatch!");
        }

        let mut new_values: Vec<Vec<T>> = Vec::with_capacity(columns.len());
        for value in values.chunks(len) {
            let v: Vec<T> = value.iter().cloned().collect();
            new_values.push(v);
        }
        Block {
            values: new_values,
            index: index,
            columns: columns,
        }
    }

    /// Instanciate from column-wise Vec
    pub fn from_row_vec<I, C>(values: Vec<T>, index: I, columns: C) -> Self
        where I: Into<Indexer<U>>,
              C: Into<Indexer<V>> {

        let index: Indexer<U> = index.into();
        let columns: Indexer<V> = columns.into();

        let len: usize = index.len();
        let cols: usize = columns.len();

        if values.len() != len * cols {
            panic!("Length mismatch!");
        }

        let mut new_values: Vec<Vec<T>> = Vec::with_capacity(columns.len());
        for i in 0..cols {
            let mut new_value: Vec<T> = Vec::with_capacity(index.len());
            for j in 0..len {
                new_value.push(values[j * cols + i]);
            }
            new_values.push(new_value);
        }
        Block {
            values: new_values,
            index: index,
            columns: columns,
        }
    }

    /// Instanciate from nested Vec
    pub fn from_vec<I, C>(values: Vec<Vec<T>>, index: I, columns: C) -> Self
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
        Block {
            values: values,
            index: index,
            columns: columns,
        }
    }

    /// Instanciate from nested Vec
    pub fn from_nested_vec<I, C>(values: Vec<Vec<T>>, index: I, columns: C) -> Self
        where I: Into<Indexer<U>>,
              C: Into<Indexer<V>> {
        Block::from_vec(values, index, columns)
    }

    /// Instanciate from Series
    pub fn from_series(series: Series<T, U>, name: V) -> Self {
        let values: Vec<Vec<T>> = vec![series.values];

        // mapper is not updated properly by vec![name]
        let mut columns = Indexer::new(vec![]);
        columns.push(name);

        Block {
            values: values,
            index: series.index,
            columns: columns,
        }
    }

    fn assert_binop(&self, other: &Block<T, U, V>) {
        if self.index != other.index {
            panic!("index must be the same!");
        }
        if self.columns != other.columns {
            panic!("columns must be the same!");
        }
    }

    /// Add columns inplace
    pub fn insert(&mut self, name: V, values: Vec<T>) {
        if self.len() != values.len() {
            panic!("Length mismatch!");
        }
        self.values.push(values);
        self.columns.push(name);
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn append(&self, other: &Block<T, U, V>) -> Block<T, U, V> {
        if self.columns != other.columns {
            panic!("columns must be the same!")
        }

        let mut new_index: Vec<U> = self.index.values.clone();
        new_index.append(&mut other.index.values.clone());

        let mut new_values: Vec<Vec<T>> = Vec::with_capacity(self.columns.len());
        for (svalues, ovalues) in self.values.iter().zip(&other.values) {
            let mut new_value = svalues.clone();
            new_value.append(&mut ovalues.clone());
            new_values.push(new_value);
        }

        Block::from_vec(new_values, new_index, self.columns.clone())
    }

    pub fn groupby<G>(&self, other: Vec<G>) -> groupby::BlockGroupBy<T, U, V, G>
        where G: Copy + Eq + Hash + Ord {
        groupby::BlockGroupBy::new(&self, other)
    }

    pub fn transpose(&self) -> Block<T, V, U> {

        let mut new_values: Vec<Vec<T>> = vec![];
        for i in 0..self.index.len() {
            let mut new_value: Vec<T> = vec![];
            for value in self.values.iter() {
                new_value.push(value[i]);
            }
            new_values.push(new_value);
        }
        Block::from_vec(new_values, self.columns.clone(), self.index.clone())
    }
}

impl<T, U, V, R> Applicable<T, R, Series<R, V>> for Block<T, U, V>
    where T: Copy,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash,
          R: Copy {

    fn apply(&self, func: &Fn(&Vec<T>) -> R) -> Series<R, V> {
        let mut new_values = vec![];
        for current in self.values.iter() {
            new_values.push(func(&current));
        }
        Series::new(new_values, self.columns.clone())
    }
}

impl<T: PartialEq, U: Hash + Eq, V: Hash + Eq> PartialEq for Block<T, U, V> {
    fn eq(&self, other: &Block<T, U, V>) -> bool {
        (self.index.eq(&other.index)) &&
        (self.columns.eq(&other.columns)) &&
        (self.values.eq(&other.values))
    }
}

#[cfg(test)]
mod tests {

    use super::Block;
    use super::super::indexer::{Indexer, IndexerIndexer};
    use super::super::series::Series;
    use super::super::traits::{RowIndexer, ColIndexer};

    #[test]
    fn test_block_creation_from_col_vec() {
        let values = vec![1, 2, 3, 4, 5,
                          6, 7, 8, 9, 10,
                          11, 12, 13, 14, 15];
        let mut b = Block::from_col_vec(values,
                                        vec!["A", "BB", "CC", "D", "EEE"],
                                        vec!["X", "YYY", "ZZ"]);
        assert_eq!(b.len(), 5);

        let exp_index: Indexer<&str> = Indexer::new(vec!["A", "BB", "CC", "D", "EEE"]);
        let exp_columns: Indexer<&str> = Indexer::new(vec!["X", "YYY", "ZZ"]);
        assert_eq!(b.index, exp_index);
        assert_eq!(b.columns, exp_columns);

        let c = b.get(&"X");
        let exp_values: Vec<i64> = vec![1, 2, 3, 4, 5];
        assert_eq!(c.values, exp_values);
        assert_eq!(c.index, exp_index);

        let c = b.get(&"YYY");
        let exp_values: Vec<i64> = vec![6, 7, 8, 9, 10];
        assert_eq!(c.values, exp_values);
        assert_eq!(c.index, exp_index);

        let c = b.get(&"ZZ");
        let exp_values: Vec<i64> = vec![11, 12, 13, 14, 15];
        assert_eq!(c.values, exp_values);
        assert_eq!(c.index, exp_index);
    }

    #[test]
    fn test_block_creation_from_row_vec() {
        let values = vec![1, 6, 11,
                          2, 7, 12,
                          3, 8, 13,
                          4, 9, 14,
                          5, 10, 15];
        let mut b = Block::from_row_vec(values,
                                        vec!["A", "BB", "CC", "D", "EEE"],
                                        vec!["X", "YYY", "ZZ"]);
        assert_eq!(b.len(), 5);

        let exp_index: Indexer<&str> = Indexer::new(vec!["A", "BB", "CC", "D", "EEE"]);
        let exp_columns: Indexer<&str> = Indexer::new(vec!["X", "YYY", "ZZ"]);
        assert_eq!(b.index, exp_index);
        assert_eq!(b.columns, exp_columns);

        let c = b.get(&"X");
        let exp_values: Vec<i64> = vec![1, 2, 3, 4, 5];
        assert_eq!(c.values, exp_values);
        assert_eq!(c.index, exp_index);

        let c = b.get(&"YYY");
        let exp_values: Vec<i64> = vec![6, 7, 8, 9, 10];
        assert_eq!(c.values, exp_values);
        assert_eq!(c.index, exp_index);

        let c = b.get(&"ZZ");
        let exp_values: Vec<i64> = vec![11, 12, 13, 14, 15];
        assert_eq!(c.values, exp_values);
        assert_eq!(c.index, exp_index);
    }

    #[test]
    fn test_block_creation_from_vec() {
        let values = vec![vec![1, 2, 3, 4, 5],
                          vec![6, 7, 8, 9, 10],
                          vec![11, 12, 13, 14, 15]];
        let mut b = Block::from_vec(values,
                                    vec!["A", "BB", "CC", "D", "EEE"],
                                    vec!["X", "YYY", "ZZ"]);
        assert_eq!(b.len(), 5);

        let exp_index: Indexer<&str> = Indexer::new(vec!["A", "BB", "CC", "D", "EEE"]);
        let exp_columns: Indexer<&str> = Indexer::new(vec!["X", "YYY", "ZZ"]);
        assert_eq!(b.index, exp_index);
        assert_eq!(b.columns, exp_columns);

        let c = b.get(&"X");
        let exp_values: Vec<i64> = vec![1, 2, 3, 4, 5];
        assert_eq!(c.values, exp_values);
        assert_eq!(c.index, exp_index);

        let c = b.get(&"YYY");
        let exp_values: Vec<i64> = vec![6, 7, 8, 9, 10];
        assert_eq!(c.values, exp_values);
        assert_eq!(c.index, exp_index);

        let c = b.get(&"ZZ");
        let exp_values: Vec<i64> = vec![11, 12, 13, 14, 15];
        assert_eq!(c.values, exp_values);
        assert_eq!(c.index, exp_index);
    }

    #[test]
    fn test_block_creation_from_nested_vec() {
        let values = vec![vec![1, 2, 3, 4, 5],
                          vec![6, 7, 8, 9, 10],
                          vec![11, 12, 13, 14, 15]];
        let mut b = Block::from_nested_vec(values,
                                           vec!["A", "BB", "CC", "D", "EEE"],
                                           vec!["X", "YYY", "ZZ"]);
        assert_eq!(b.len(), 5);

        let exp_values = vec![vec![1, 2, 3, 4, 5],
                              vec![6, 7, 8, 9, 10],
                              vec![11, 12, 13, 14, 15]];
        let exp = Block::from_vec(exp_values,
                                  vec!["A", "BB", "CC", "D", "EEE"],
                                  vec!["X", "YYY", "ZZ"]);
        assert_eq!(b, exp);
    }

    #[test]
    fn test_block_creation_from_series() {
        let values: Vec<f64> = vec![1., 2., 3.];
        let index: Vec<&str> = vec!["A", "B", "C"];
        let s = Series::<f64, &str>::new(values, index);

        let mut b = Block::<f64, &str, i64>::from_series(s, 1);
        assert_eq!(b.len(), 3);

        let exp_index: Indexer<&str> = Indexer::new(vec!["A", "B", "C"]);
        let exp_columns: Indexer<i64> = Indexer::new(vec![1]);
        assert_eq!(b.index, exp_index);
        assert_eq!(b.columns, exp_columns);

        let c = b.get(&1);
        let exp_values: Vec<f64> = vec![1., 2., 3.];
        assert_eq!(c.values, exp_values);
        assert_eq!(c.index, exp_index);
    }

    #[test]
    fn test_block_creation_into() {
        let values = vec![1, 2, 3, 4, 5,
                          6, 7, 8, 9, 10,
                          11, 12, 13, 14, 15];
        let exp = Block::from_col_vec(values,
                                      vec!["A", "BB", "CC", "D", "EEE"],
                                      vec!["X", "YYY", "ZZ"]);

        let index = Indexer::new(vec!["A", "BB", "CC", "D", "EEE"]);
        let columns = Indexer::new(vec!["X", "YYY", "ZZ"]);
        let values = vec![1, 2, 3, 4, 5,
                          6, 7, 8, 9, 10,
                          11, 12, 13, 14, 15];
        let b = Block::from_col_vec(values, index, columns);
        assert_eq!(b, exp);

        let index = Indexer::new(vec!["A", "BB", "CC", "D", "EEE"]);
        let columns = Indexer::new(vec!["X", "YYY", "ZZ"]);
        let values = vec![1, 6, 11,
                          2, 7, 12,
                          3, 8, 13,
                          4, 9, 14,
                          5, 10, 15];
        let b = Block::from_row_vec(values, index, columns);
        assert_eq!(b, exp);

        let index = Indexer::new(vec!["A", "BB", "CC", "D", "EEE"]);
        let columns = Indexer::new(vec!["X", "YYY", "ZZ"]);
        let values = vec![vec![1, 2, 3, 4, 5],
                          vec![6, 7, 8, 9, 10],
                          vec![11, 12, 13, 14, 15]];
        let b = Block::from_nested_vec(values, index, columns);
        assert_eq!(b, exp);
    }

    #[test]
    fn test_block_columns_get() {
        let values = vec![1, 2, 3, 4, 5,
                          6, 7, 8, 9, 10,
                          11, 12, 13, 14, 15];
        let mut b = Block::from_col_vec(values,
                                        vec!["A", "BB", "CC", "D", "EEE"],
                                        vec!["X", "YYY", "ZZ"]);

        let res = b.get(&"YYY");
        let exp: Series<i64, &str> = Series::new(vec![6, 7, 8, 9, 10],
                                                 vec!["A", "BB", "CC", "D", "EEE"]);
        assert_eq!(res, exp);

        let res = b.iget(&1);
        let exp: Series<i64, &str> = Series::new(vec![6, 7, 8, 9, 10],
                                                 vec!["A", "BB", "CC", "D", "EEE"]);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_columns_slice() {
        let values = vec![1, 2, 3, 4, 5,
                          6, 7, 8, 9, 10,
                          11, 12, 13, 14, 15];
        let mut b = Block::from_col_vec(values,
                                        vec!["A", "BB", "CC", "D", "EEE"],
                                        vec!["X", "YYY", "ZZ"]);

        let exp = Block::from_col_vec(vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5,],
                                      vec!["A", "BB", "CC", "D", "EEE"],
                                      vec!["YYY", "X"]);
        let res = b.gets(&vec!["YYY", "X"]);
        assert_eq!(res, exp);

        let res = b.igets(&vec![1, 0]);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_insert() {
        let values: Vec<f64> = vec![1., 2., 3.];
        let index: Vec<&str> = vec!["A", "B", "C"];
        let s = Series::<f64, &str>::new(values, index);

        let mut b = Block::<f64, &str, i64>::from_series(s, 1);

        assert_eq!(b.len(), 3);
        let exp_index: Indexer<&str> = Indexer::new(vec!["A", "B", "C"]);
        let exp_columns: Indexer<i64> = Indexer::new(vec![1]);
        assert_eq!(b.index, exp_index);
        assert_eq!(b.columns, exp_columns);

        // add columns
        let values2: Vec<f64> = vec![4., 5., 6.];
        b.insert(3, values2);
        assert_eq!(b.len(), 3);
        let exp_columns: Indexer<i64> = Indexer::new(vec![1, 3]);
        assert_eq!(b.index, exp_index);
        assert_eq!(b.columns, exp_columns);

        assert_eq!(b.columns.get_loc(&1), 0);
        assert_eq!(b.columns.get_loc(&3), 1);
        let c = b.get(&1);
        let exp_values: Vec<f64> = vec![1., 2., 3.];
        assert_eq!(c.values, exp_values);
        assert_eq!(c.index, exp_index);

        let c = b.get(&3);
        let exp_values: Vec<f64> = vec![4., 5., 6.];
        assert_eq!(c.values, exp_values);
        assert_eq!(c.index, exp_index);
    }

    #[test]
    fn test_slice_ilocs() {
        let values: Vec<f64> = vec![1., 2., 3.];
        let index: Vec<&str> = vec!["A", "B", "C"];
        let s = Series::<f64, &str>::new(values, index);
        let mut b = Block::<f64, &str, i64>::from_series(s, 1);
        // add columns
        let values2: Vec<f64> = vec![4., 5., 6.];
        b.insert(3, values2);
        assert_eq!(b.len(), 3);

        // slice
        let mut sliced = b.ilocs(&vec![0, 2]);
        let exp_index: Indexer<&str> = Indexer::new(vec!["A", "C"]);
        let exp_columns: Indexer<i64> = Indexer::new(vec![1, 3]);
        assert_eq!(sliced.index, exp_index);
        assert_eq!(sliced.columns, exp_columns);

        // compare columns
        let c = sliced.get(&1);
        let exp_values: Vec<f64> = vec![1., 3.];
        assert_eq!(c.values, exp_values);
        let c = sliced.get(&3);
        let exp_values: Vec<f64> = vec![4., 6.];
        assert_eq!(c.values, exp_values);
    }

    #[test]
    fn test_slice_locs() {
        let values: Vec<f64> = vec![1., 2., 3.];
        let index: Vec<&str> = vec!["A", "B", "C"];
        let s = Series::<f64, &str>::new(values, index);
        let mut b = Block::<f64, &str, i64>::from_series(s, 1);
        // add columns
        let values2: Vec<f64> = vec![4., 5., 6.];
        b.insert(3, values2);
        assert_eq!(b.len(), 3);

        // slice
        let mut sliced = b.locs(&vec!["B", "C"]);
        let exp_index: Indexer<&str> = Indexer::new(vec!["B", "C"]);
        let exp_columns: Indexer<i64> = Indexer::new(vec![1, 3]);
        assert_eq!(sliced.index, exp_index);
        assert_eq!(sliced.columns, exp_columns);

        // compare columns
        let c = sliced.get(&1);
        let exp_values: Vec<f64> = vec![2., 3.];
        assert_eq!(c.values, exp_values);
        let c = sliced.get(&3);
        let exp_values: Vec<f64> = vec![5., 6.];
        assert_eq!(c.values, exp_values);
    }

    #[test]
    fn test_block_reindex() {
        let values = vec![vec![1, 2, 3, 4, 5],
                          vec![6, 7, 8, 9, 10],
                          vec![11, 12, 13, 14, 15]];
        let mut b = Block::from_nested_vec(values,
                                           vec!["A", "BB", "CC", "D", "EEE"],
                                           vec!["X", "YYY", "ZZ"]);
        let res = b.reindex(&vec!["BB", "D", "A"]);

        let values = vec![vec![2, 4, 1],
                          vec![7, 9, 6],
                          vec![12, 14, 11]];
        let exp = Block::from_nested_vec(values,
                                         vec!["BB", "D", "A"],
                                         vec!["X", "YYY", "ZZ"]);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_reindex_by_index() {
        let values = vec![vec![1, 2, 3, 4, 5],
                          vec![6, 7, 8, 9, 10],
                          vec![11, 12, 13, 14, 15]];
        let b = Block::from_nested_vec(values,
                                       vec!["A", "BB", "CC", "D", "EEE"],
                                       vec!["X", "YYY", "ZZ"]);
        let res = b.reindex_by_index(&vec![1, 3, 0]);

        let values = vec![vec![2, 4, 1],
                          vec![7, 9, 6],
                          vec![12, 14, 11]];
        let exp = Block::from_nested_vec(values,
                                         vec!["BB", "D", "A"],
                                         vec!["X", "YYY", "ZZ"]);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_append() {
        let b1 = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                     vec!["A", "B", "C"],
                                     vec!["X", "Y"]);
        let b2 = Block::from_col_vec(vec![7., 8., 9., 10., 11., 12.],
                                     vec!["D", "E", "F"],
                                     vec!["X", "Y"]);

        let mut res = b1.append(&b2);

        let exp_index: Indexer<&str> = Indexer::new(vec!["A", "B", "C", "D", "E", "F"]);
        let exp_columns: Indexer<&str> = Indexer::new(vec!["X", "Y"]);
        assert_eq!(res.index, exp_index);
        assert_eq!(res.columns, exp_columns);

        let c = res.get(&"X");
        assert_eq!(c.values, vec![1., 2., 3., 7., 8., 9.]);
        let c = res.get(&"Y");
        assert_eq!(c.values, vec![4., 5., 6., 10., 11., 12.]);
    }

    #[test]
    fn test_block_transpose() {
        let b1 = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                     vec!["A", "B", "C"],
                                     vec!["X", "Y"]);
        let mut res = b1.transpose();

        let exp = Block::from_row_vec(vec![1., 2., 3., 4., 5., 6.],
                                      vec!["X", "Y"],
                                      vec!["A", "B", "C"]);
        assert_eq!(res, exp);
    }
}