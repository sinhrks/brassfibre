use std::borrow::{Borrow, Cow};
use std::hash::Hash;
use std::slice;
use std::vec;

use super::algos::sort::Sorter;
use super::indexer::Indexer;
use super::groupby::GroupBy;
use super::series::Series;
use super::traits::{IndexerIndexer, RowIndexer, ColIndexer,
                    Applicable};

mod aggregation;
mod formatting;
mod groupby;
mod ops;
mod reshape;

#[derive(Clone)]
pub struct Block<'i, 'c, V, I, C>
    where I: 'i + Clone + Hash,
          C: 'c + Clone + Hash {
    /// 2-dimentional block contains a single type.
    /// V: type of values
    /// I: type of indexer
    /// C: type of columns

    // ToDo: may be simpler to use 1-d Vec?
    pub values: Vec<Vec<V>>,
    pub index: Cow<'i, Indexer<I>>,
    pub columns: Cow<'c, Indexer<C>>,
}

////////////////////////////////////////////////////////////////////////////////
// Indexing
////////////////////////////////////////////////////////////////////////////////

impl<'i, 'c, V, I, C> RowIndexer<'c> for Block<'i, 'c, V, I, C>
    where V: Copy,
          I: Clone + Eq + Hash,
          C: Clone + Eq + Hash {

    type Key = I;
    type Row = V;

    fn len(&'c self) -> usize {
        self.index.len()
    }

    fn loc<'l>(&'c self, label: &'l Self::Key) -> Self::Row {
        unimplemented!()
    }

    fn iloc<'l>(&'c self, locaiton: &'l usize) -> Self::Row {
        unimplemented!()
    }

    fn reindex<'l>(&'c self, labels: &'l Vec<Self::Key>) -> Self {
        let locations = self.index.get_locs(labels);
        self.reindex_by_index(&locations)
    }

    fn reindex_by_index<'l>(&'c self, locations: &'l Vec<usize>) -> Self {
        let new_index = self.index.reindex(locations);

        let mut new_values: Vec<Vec<V>> = Vec::with_capacity(self.columns.len());
        for current in self.values.iter() {
            let new_value = Sorter::reindex(current, locations);
            new_values.push(new_value);
        }
        Block::from_cow(new_values,
                        Cow::Owned(new_index),
                        Cow::Borrowed(self.columns.borrow()))
    }

    fn blocs<'l>(&'c self, labels: &'l Vec<bool>) -> Self {
        unimplemented!()
        // ToDo: fix Series impl
    }
}

impl<'i, 'c, V, I, C> ColIndexer<'i> for Block<'i, 'c, V, I, C>
    where V: Copy,
          I: 'i + Clone + Eq + Hash,
          C: Clone + Eq + Hash {

    type Key = C;
    type Column = Series<'i, V, I>;

    fn get<'l>(&'i self, label: &'l Self::Key) -> Self::Column {
        let loc = self.columns.get_loc(label);
        self.iget(&loc)
    }

    fn iget<'l>(&'i self, loc: &'l usize) -> Self::Column {
        let new_values = self.values[*loc].clone();
        Series::from_cow(new_values, Cow::Borrowed(self.index.borrow()))
    }

    fn gets<'l>(&'i self, labels: &'l Vec<Self::Key>) -> Self {
        let locs = self.columns.get_locs(labels);
        self.igets(&locs)
    }

    fn igets<'l>(&'i self, locations: &'l Vec<usize>) -> Self {
        let new_columns = self.columns.reindex(locations);

        let mut new_values: Vec<Vec<V>> = Vec::with_capacity(new_columns.len());
        for loc in locations {
            new_values.push(self.values[*loc].clone());
        }
        Block::from_cow(new_values,
                        Cow::Borrowed(self.index.borrow()),
                        Cow::Owned(new_columns))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////

impl<'i, 'c, V, I, C> Block<'i, 'c, V, I, C>
    where V: Copy,
          I: Clone + Eq + Hash,
          C: Clone + Eq + Hash {

    /// Instanciate from column-wise Vec
    pub fn from_col_vec<X, Y>(values: Vec<V>, index: X, columns: Y) -> Self
        where X: Into<Indexer<I>>,
              Y: Into<Indexer<C>> {

        let index: Indexer<I> = index.into();
        let columns: Indexer<C> = columns.into();

        let len: usize = index.len();
        let cols: usize = columns.len();

        assert!(values.len() == len * cols, "Length mismatch!");

        let mut new_values: Vec<Vec<V>> = Vec::with_capacity(columns.len());
        for value in values.chunks(len) {
            let v: Vec<V> = value.iter().cloned().collect();
            new_values.push(v);
        }
        Block {
            values: new_values,
            index: Cow::Owned(index),
            columns: Cow::Owned(columns),
        }
    }

    /// Instanciate from column-wise Vec
    pub fn from_row_vec<X, Y>(values: Vec<V>, index: X, columns: Y) -> Self
        where X: Into<Indexer<I>>,
              Y: Into<Indexer<C>> {

        let index: Indexer<I> = index.into();
        let columns: Indexer<C> = columns.into();

        let len: usize = index.len();
        let cols: usize = columns.len();

        assert!(values.len() == len * cols, "Length mismatch!");

        let mut new_values: Vec<Vec<V>> = Vec::with_capacity(columns.len());
        for i in 0..cols {
            let mut new_value: Vec<V> = Vec::with_capacity(index.len());
            for j in 0..len {
                new_value.push(values[j * cols + i]);
            }
            new_values.push(new_value);
        }
        Block {
            values: new_values,
            index: Cow::Owned(index),
            columns: Cow::Owned(columns),
        }
    }

    /// Instanciate from nested Vec
    pub fn from_vec<X, Y>(values: Vec<Vec<V>>, index: X, columns: Y) -> Self
        where X: Into<Indexer<I>>,
              Y: Into<Indexer<C>> {

        let index: Indexer<I> = index.into();
        let columns: Indexer<C> = columns.into();

        assert!(values.len() == columns.len(), "Length mismatch!");

        let len = index.len();
        for value in values.iter() {
            assert!(value.len() == len, "Length mismatch!");
        }
        Block {
            values: values,
            index: Cow::Owned(index),
            columns: Cow::Owned(columns),
        }
    }

    /// Instanciate from nested Vec
    pub fn from_nested_vec<X, Y>(values: Vec<Vec<V>>, index: X, columns: Y) -> Self
        where X: Into<Indexer<I>>,
              Y: Into<Indexer<C>> {
        Block::from_vec(values, index, columns)
    }

    fn from_cow(values: Vec<Vec<V>>,
                index: Cow<'i, Indexer<I>>,
                columns: Cow<'c, Indexer<C>>) -> Self {
        Block {
            values: values,
            index: index,
            columns: columns,
        }
    }

    /// Instanciate from Series
    pub fn from_series(series: Series<'i, V, I>, name: C) -> Self {
        let values: Vec<Vec<V>> = vec![series.values];

        // mapper is not updated properly by vec![name]
        let mut columns = Indexer::new(vec![]);
        columns.push(name);

        Block {
            values: values,
            index: Cow::Owned(series.index.into_owned()),
            columns: Cow::Owned(columns),
        }
    }

    fn assert_binop(&self, other: &Self) {
        assert!(self.index == other.index, "index must be the same!");
        assert!(self.columns == other.columns, "columns must be the same!");
    }

    /// Add columns inplace
    pub fn insert(&mut self, name: C, values: Vec<V>) {
        assert!(self.len() == values.len(), "Length mismatch!");
        self.values.push(values);
        self.columns.to_mut().push(name);
    }

    pub fn groupby<G>(&self, other: Vec<G>) -> GroupBy<Block<V, I, C>, G>
        where G: Copy + Eq + Hash + Ord {
        GroupBy::new(&self, other)
    }

    pub fn transpose(&'i self) -> Block<'i, 'i, V, C, I> {

        let mut new_values: Vec<Vec<V>> = vec![];
        for i in 0..self.index.len() {
            let mut new_value: Vec<V> = vec![];
            for value in self.values.iter() {
                new_value.push(value[i]);
            }
            new_values.push(new_value);
        }
        Block::from_cow(new_values,
                        Cow::Borrowed(self.columns.borrow()),
                        Cow::Borrowed(self.index.borrow()))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Apply
////////////////////////////////////////////////////////////////////////////////

impl<'a, 'b, V, I, C, R> Applicable<'b, R> for Block<'a, 'b, V, I, C>
    where V: Copy,
          I: Clone + Eq + Hash,
          C: Clone + Eq + Hash,
          R: Copy {

    type In = Vec<V>;
    type FOut = R;
    type Out = Series<'b, R, C>;

    fn apply<'f>(&'b self, func: &'f Fn(&Vec<V>) -> R) -> Series<'b, R, C> {
        let mut new_values = vec![];
        for current in self.values.iter() {
            new_values.push(func(&current));
        }
        Series::from_cow(new_values, Cow::Borrowed(self.columns.borrow()))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Eq
////////////////////////////////////////////////////////////////////////////////

impl<'a, 'b, V, I, C> PartialEq for Block<'a, 'b, V, I, C>
    where V: PartialEq,
          I: Clone + Hash + Eq,
          C: Clone + Hash + Eq {

    fn eq(&self, other: &Self) -> bool {
        (self.index.eq(&other.index)) &&
        (self.columns.eq(&other.columns)) &&
        (self.values.eq(&other.values))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Iterator
////////////////////////////////////////////////////////////////////////////////

impl<'a, 'b, V, I, C> IntoIterator for Block<'a, 'b, V, I, C>
    where I: Clone + Hash + Eq,
          C: Clone + Hash + Eq  {

    type Item = Vec<V>;
    type IntoIter = vec::IntoIter<Vec<V>>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'a, 'b, V, I, C> Block<'a, 'b, V, I, C>
    where I: Clone + Hash + Eq,
          C: Clone + Hash + Eq  {

    pub fn iter(&self) -> slice::Iter<Vec<V>> {
        self.values.iter()
    }
}
