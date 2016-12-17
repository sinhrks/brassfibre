use std::borrow::{Borrow, Cow};
use std::hash::Hash;
use std::slice;
use std::vec;

use super::algos::sort::Sorter;
use super::indexer::Indexer;
use super::groupby::GroupBy;
use super::series::Series;
use super::traits::{Slicer, IndexerIndex, RowIndex, ColIndex,
                    Apply};

mod aggregation;
mod formatting;
mod groupby;
mod ops;
mod reshape;

#[derive(Clone)]
pub struct Block<'v, 'i, 'c, V, I, C>
    where V: 'v + Clone,
          I: 'i + Clone + Hash,
          C: 'c + Clone + Hash {
    /// 2-dimentional block contains a single type.
    /// V: type of values
    /// I: type of indexer
    /// C: type of columns

    // ToDo: may be simpler to use 1-d Vec?
    pub values: Vec<Cow<'v, Vec<V>>>,
    pub index: Cow<'i, Indexer<I>>,
    pub columns: Cow<'c, Indexer<C>>,
}

////////////////////////////////////////////////////////////////////////////////
// Indexing
////////////////////////////////////////////////////////////////////////////////

impl<'v, 'i, 'c, V, I, C> RowIndex<'c> for Block<'v, 'i, 'c, V, I, C>
    where V: Clone,
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

    fn reindex<'l>(&'c self, labels: &'l [Self::Key]) -> Self {
        let locations = self.index.get_locs(labels);
        self.reindex_by_index(&locations)
    }

    fn reindex_by_index<'l>(&'c self, locations: &'l [usize]) -> Self {
        let new_index = self.index.reindex(locations);
        // boudaries are checked in Indexer.reindex

        let mut new_values: Vec<Cow<Vec<V>>> = Vec::with_capacity(self.columns.len());
        for current in self.values.iter() {
            let new_value = unsafe {
                Sorter::reindex_unchecked(current, locations)
            };
            new_values.push(Cow::Owned(new_value));
        }
        Block::from_cow(new_values,
                        Cow::Owned(new_index),
                        Cow::Borrowed(self.columns.borrow()))
    }

    fn blocs<'l>(&'c self, labels: &'l [bool]) -> Self {
        unimplemented!()
        // ToDo: fix Series impl
    }
}

impl<'v, 'i, 'c, V, I, C> ColIndex<'i> for Block<'v, 'i, 'c, V, I, C>
    where V: 'i + Clone,
          I: 'i + Clone + Eq + Hash,
          C: Clone + Eq + Hash {

    type Key = C;
    type Column = Series<'i, 'i, V, I>;

    fn get<'l>(&'i self, label: &'l Self::Key) -> Self::Column {
        let loc = self.columns.get_loc(label);
        self.iget(&loc)
    }

    fn iget<'l>(&'i self, loc: &'l usize) -> Self::Column {
        Series::from_cow(Cow::Borrowed(self.values[*loc].borrow()),
                         Cow::Borrowed(self.index.borrow()))
    }

    fn gets<'l>(&'i self, labels: &'l [Self::Key]) -> Self {
        let locs = self.columns.get_locs(labels);
        self.igets(&locs)
    }

    fn igets<'l>(&'i self, locations: &'l [usize]) -> Self {
        let new_columns = self.columns.reindex(locations);

        let mut new_values: Vec<Cow<Vec<V>>> = Vec::with_capacity(new_columns.len());
        for loc in locations {
            // ToDo: Avoid clone
            // new_values.push(Cow::Borrowed(self.values[*loc].borrow()));
            new_values.push(Cow::Owned(self.values[*loc].clone().into_owned()));
        }
        Block::from_cow(new_values,
                        Cow::Borrowed(self.index.borrow()),
                        Cow::Owned(new_columns))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////

impl<'v, 'i, 'c, V, I, C> Block<'v, 'i, 'c, V, I, C>
    where V: Clone,
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

        let mut new_values: Vec<Cow<Vec<V>>> = Vec::with_capacity(columns.len());
        for value in values.chunks(len) {
            let v: Vec<V> = value.iter().cloned().collect();
            new_values.push(Cow::Owned(v));
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

        let mut new_values: Vec<Cow<Vec<V>>> = Vec::with_capacity(columns.len());
        for i in 0..cols {
            let mut new_value: Vec<V> = Vec::with_capacity(index.len());
            for j in 0..len {
                // ToDo: avoid clone
                new_value.push(values[j * cols + i].clone());
            }
            new_values.push(Cow::Owned(new_value));
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
        // ToDo: can merge with below logic?
        for value in values.iter() {
            assert!(value.len() == len, "Length mismatch!");
        }
        let values: Vec<Cow<Vec<V>>> = values.into_iter()
                                             .map(|x| Cow::Owned(x))
                                             .collect();
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

    fn from_cow(values: Vec<Cow<'v, Vec<V>>>,
                index: Cow<'i, Indexer<I>>,
                columns: Cow<'c, Indexer<C>>) -> Self {
        Block {
            values: values,
            index: index,
            columns: columns,
        }
    }

    /// Instanciate from Series
    pub fn from_series(series: Series<'v, 'i, V, I>, name: C) -> Self {
        let values: Vec<Cow<Vec<V>>> = vec![series.values];

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
        self.values.push(Cow::Owned(values));
        self.columns.to_mut().push(name);
    }

    pub fn groupby<G>(&self, other: Vec<G>) -> GroupBy<Block<V, I, C>, G>
        where G: Clone + Eq + Hash + Ord {
        GroupBy::new(&self, other)
    }

    pub fn transpose(&'i self) -> Block<'i, 'i, 'i, V, C, I> {

        let mut new_values: Vec<Cow<Vec<V>>> = vec![];
        for i in 0..self.index.len() {
            let mut new_value: Vec<V> = vec![];
            for value in self.values.iter() {
                new_value.push(value[i].clone());
            }
            new_values.push(Cow::Owned(new_value));
        }
        Block::from_cow(new_values,
                        Cow::Borrowed(self.columns.borrow()),
                        Cow::Borrowed(self.index.borrow()))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Apply
////////////////////////////////////////////////////////////////////////////////

impl<'v, 'i, 'c, V, I, C, R> Apply<'i, R> for Block<'v, 'i, 'c, V, I, C>
    where V: Clone,
          I: Clone + Eq + Hash,
          C: 'i + Clone + Eq + Hash,
          R: 'i + Clone {

    type In = Vec<V>;
    type FOut = R;
    // ToDo: use 'n lifetime for values
    type Out = Series<'i, 'i, R, C>;

    fn apply<'f>(&'i self, func: &'f Fn(&Self::In) -> Self::FOut) -> Self::Out {
        let mut new_values = vec![];
        for current in self.values.iter() {
            new_values.push(func(&current));
        }
        Series::from_cow(Cow::Owned(new_values),
                         Cow::Borrowed(self.columns.borrow()))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Eq
////////////////////////////////////////////////////////////////////////////////

impl<'v, 'i, 'c, V, I, C> PartialEq for Block<'v, 'i, 'c, V, I, C>
    where V: Clone + PartialEq,
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

impl<'v, 'i, 'c, V, I, C> IntoIterator for Block<'v, 'i, 'c, V, I, C>
    where V: Clone,
          I: Clone + Hash + Eq,
          C: Clone + Hash + Eq  {

    // ToDo: remove cow, return Series
    type Item = Cow<'v, Vec<V>>;
    type IntoIter = vec::IntoIter<Cow<'v, Vec<V>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'v, 'i, 'c, V, I, C> Block<'v, 'i, 'c, V, I, C>
    where V: Clone,
          I: Clone + Hash + Eq,
          C: Clone + Hash + Eq  {

    pub fn iter(&self) -> slice::Iter<Cow<Vec<V>>> {
        self.values.iter()
    }
}
