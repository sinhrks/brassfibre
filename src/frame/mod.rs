use std::borrow::{Borrow, Cow};
use std::hash::Hash;
use std::slice;
use std::vec;

use super::groupby::GroupBy;
use super::indexer::Indexer;
use super::internals::Array;
use super::traits::{Slicer, IndexerIndexer, RowIndexer, ColIndexer};

mod aggregation;
mod formatting;
mod reshape;

#[derive(Clone)]
pub struct DataFrame<'i, 'c, I: Hash, C: Hash>
    where I: 'i + Clone + Hash,
          C: 'c + Clone + Hash {
    /// 2-dimentional block contains multiple type.
    /// I: type of indexer
    /// C: type of columns

    // Do not use #[derice(PartialEq)] as internals may not be comparable
    pub values: Vec<Array>,
    pub index: Cow<'i, Indexer<I>>,
    pub columns: Cow<'c, Indexer<C>>,
}

////////////////////////////////////////////////////////////////////////////////
// Indexing
////////////////////////////////////////////////////////////////////////////////

impl<'i, 'c, I, C> RowIndexer<'c> for DataFrame<'i, 'c, I, C>
    where I: Clone + Eq + Hash,
          C: Clone + Eq + Hash {

    type Key = I;
    type Row = Array;

    fn len(&'c self) -> usize {
        self.index.len()
    }

    fn loc(&'c self, label: &Self::Key) -> Self::Row {
        unimplemented!()
    }

    fn iloc(&'c self, locaiton: &usize) -> Self::Row {
        unimplemented!()
    }

    fn reindex<'l>(&'c self, labels: &'l [Self::Key]) -> Self {
        let locations = self.index.get_locs(labels);
        self.reindex_by_index(&locations)
    }

    fn reindex_by_index<'l>(&'c self, locations: &'l [usize]) -> Self {

        let new_index = self.index.reindex(locations);

        let mut new_values: Vec<Array> = Vec::with_capacity(self.columns.len());
        for current in self.values.iter() {
            let new_value = current.ilocs(locations);
            new_values.push(new_value);
        }
        DataFrame::from_cow(new_values,
                            Cow::Owned(new_index),
                            Cow::Borrowed(self.columns.borrow()))
    }

    fn blocs(&self, labels: &[bool]) -> Self {
        unimplemented!()
        // ToDo: fix Series impl
    }
}

impl<'i, 'c, I, C> ColIndexer<'i> for DataFrame<'i, 'c, I, C>
    where I: Clone + Eq + Hash,
          C: Clone + Eq + Hash {

    type Key = C;
    type Column = Array;

    fn get(&'i self, label: &Self::Key) -> Self::Column {
        unimplemented!();
    }

    fn iget(&'i self, loc: &usize) -> Self::Column {
        unimplemented!();
    }

    fn gets<'l>(&'i self, labels: &'l [Self::Key]) -> Self {
        let locs = self.columns.get_locs(labels);
        self.igets(&locs)
    }

    fn igets<'l>(&'i self, locations: &'l [usize]) -> Self {
        let new_columns = self.columns.reindex(locations);

        let mut new_values: Vec<Array> = Vec::with_capacity(new_columns.len());
        for loc in locations {
            new_values.push(self.values[*loc].clone());
        }
        DataFrame::from_cow(new_values,
                            Cow::Borrowed(self.index.borrow()),
                            Cow::Owned(new_columns))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////

impl<'s, 'i, 'c, I, C> DataFrame<'i, 'c, I, C>
    where I: Clone + Eq + Hash,
          C: Clone + Eq + Hash {

    pub fn from_vec<X, Y>(values: Vec<Array>,
                          index: X, columns: Y) -> Self
        where X: Into<Indexer<I>>,
              Y: Into<Indexer<C>> {

        let index: Indexer<I> = index.into();
        let columns: Indexer<C> = columns.into();

        assert!(values.len() == columns.len(), "Length mismatch!");

        let len = index.len();
        for value in values.iter() {
            assert!(value.len() == len, "Length mismatch!");
        }
        DataFrame {
            values: values,
            index: Cow::Owned(index),
            columns: Cow::Owned(columns),
        }
    }

    fn from_cow(values: Vec<Array>,
                index: Cow<'i, Indexer<I>>,
                columns: Cow<'c, Indexer<C>>) -> Self {
        // temp internal, use IntoCow
        DataFrame {
            values: values,
            index: index,
            columns: columns,
        }
    }

    pub fn dtypes(&self) -> Vec<String> {
        self.iter().map(|ref x| x.dtype()).collect()
    }

    pub fn is_numeric(&self) -> Vec<bool> {
        self.iter().map(|ref x| x.is_numeric()).collect()
    }

    fn get_numeric_data(&'i self) -> Self {
        let flags = self.is_numeric();
        let new_columns = self.columns.blocs(&flags);
        let new_values: Vec<Array> = self.iter()
                                         .zip(flags.iter())
                                         .filter(|&(_, y)| *y)
                                         .map(|(ref x, _)| (*x).clone())
                                         .collect();
        DataFrame::from_cow(new_values,
                            Cow::Borrowed(self.index.borrow()),
                            Cow::Owned(new_columns))
    }

    fn assert_binop(&self, other: &Self) {
        assert!(self.index == other.index, "index must be the same!");
        assert!(self.columns == other.columns, "columns must be the same!");
    }

    pub fn insert(&mut self, values: Array, name: C) {
        assert!(self.len() == values.len(), "Length mismatch!");

        self.values.push(values);
        self.columns.to_mut().push(name);
    }

    pub fn groupby<G>(&'i self, other: Vec<G>) -> GroupBy<DataFrame<I, C>, G>
        where G: Copy + Eq + Hash + Ord {

        GroupBy::new(&self, other)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Eq
////////////////////////////////////////////////////////////////////////////////

impl<'i, 'c, I, C> PartialEq for DataFrame<'i, 'c, I, C>
    where I: Clone + Hash + Eq,
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

impl<'a, 'b, I, C> IntoIterator for DataFrame<'a, 'b, I, C>
    where I: Clone + Hash + Eq,
          C: Clone + Hash + Eq  {

    type Item = Array;
    type IntoIter = vec::IntoIter<Array>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'a, 'b, I, C> DataFrame<'a, 'b, I, C>
    where I: Clone + Hash + Eq,
          C: Clone + Hash + Eq  {

    pub fn iter(&self) -> slice::Iter<Array> {
        self.values.iter()
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
        let df = DataFrame::from_vec(values,
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
        let b = DataFrame::from_vec(values,
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
