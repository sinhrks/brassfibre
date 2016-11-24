use std::hash::Hash;

use super::algos::sort::Sorter;
use super::indexer::Indexer;
use super::series::Series;
use super::traits::{IndexerIndexer, RowIndexer, ColIndexer, Appender,
                    Applicable};

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

    fn reindex(&self, labels: &Vec<U>) -> Self {
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

    fn get(&self, label: &V) -> Series<T, U> {
        let loc = self.columns.get_loc(label);
        self.iget(&loc)
    }

    fn iget(&self, loc: &usize) -> Series<T, U> {
        let new_values = self.values[*loc].clone();
        Series::new(new_values, self.index.clone())
    }

    fn gets(&self, labels: &Vec<V>) -> Self {
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

////////////////////////////////////////////////////////////////////////////////
// Append
////////////////////////////////////////////////////////////////////////////////

impl<T, U, V> Appender for Block<T, U, V>
    where T: Copy,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash {

    fn append(&self, other: &Self) -> Self {
        if self.columns != other.columns {
            panic!("columns must be the same!")
        }

        let new_index = self.index.append(&other.index);

        let mut new_values: Vec<Vec<T>> = Vec::with_capacity(self.columns.len());
        for (svalues, ovalues) in self.values.iter().zip(&other.values) {
            let mut new_value = svalues.clone();
            new_value.append(&mut ovalues.clone());
            new_values.push(new_value);
        }

        Block::from_vec(new_values, new_index, self.columns.clone())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Apply
////////////////////////////////////////////////////////////////////////////////

impl<T, U, V, R> Applicable<Vec<T>, R, Series<R, V>> for Block<T, U, V>
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

////////////////////////////////////////////////////////////////////////////////
// Eq
////////////////////////////////////////////////////////////////////////////////

impl<T: PartialEq, U: Hash + Eq, V: Hash + Eq> PartialEq for Block<T, U, V> {
    fn eq(&self, other: &Block<T, U, V>) -> bool {
        (self.index.eq(&other.index)) &&
        (self.columns.eq(&other.columns)) &&
        (self.values.eq(&other.values))
    }
}
