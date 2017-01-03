use std::borrow::{Borrow, Cow};
use std::hash::Hash;

use super::Block;
use algos::join::{JoinOp, HashJoin};
use indexer::Indexer;
use traits::{Slicer, IndexerIndex, RowIndex,
             Append, Concatenation, Join};


impl<'v, 'i, 'c, V, I, C> Append<'c> for Block<'v, 'i, 'c, V, I, C>
    where V: Clone,
          I: Clone + Eq + Hash,
          C: Clone + Eq + Hash {

    fn append<'o>(&'c self, other: &'o Self) -> Self {
        assert!(self.columns == other.columns, "columns must be identical");

        let new_index = self.index.append(&other.index);

        let mut new_values: Vec<Cow<Vec<V>>> = Vec::with_capacity(self.columns.len());
        for (svalues, ovalues) in self.values.iter().zip(&other.values) {
            // ToDo: avoid clone
            let mut new_value = svalues.clone().into_owned();
            new_value.append(&mut ovalues.clone().into_owned());
            new_values.push(Cow::Owned(new_value));
        }

        Block::from_cow(new_values,
                        Cow::Owned(new_index),
                        Cow::Borrowed(self.columns.borrow()))
    }
}

impl<'v, 'i, 'c, V, I, C> Concatenation<'i> for Block<'v, 'i, 'c, V, I, C>
    where V: Clone,
          I: Clone + Eq + Hash,
          C: Clone + Eq + Hash {

    fn concat<'o>(&'i self, other: &'o Self) -> Self {
        assert!(self.index == other.index, "index must be identical");

        let new_columns = self.columns.append(&other.columns);

        let mut new_values: Vec<Cow<Vec<V>>> = Vec::with_capacity(new_columns.len());
        for values in self.values.iter().chain(&other.values) {
            // ToDo: avoid clone / into_owned()
            // new_values.push(Cow::Borrowed(values.borrow()));
            new_values.push(Cow::Owned(values.clone().into_owned()));
        }

        Block::from_cow(new_values,
                        Cow::Borrowed(self.index.borrow()),
                        Cow::Owned(new_columns))
    }
}

impl<'v, 'i, 'c, V, I, C> Join for Block<'v, 'i, 'c, V, I, C>
    where V: Clone,
          I: Clone + Eq + Hash,
          C: Clone + Eq + Hash {

    fn join_inner(&self, other: &Self) -> Self {

        let (new_index, lindexer, rindexer) = HashJoin::inner(&self.index.values, &other.index.values);
        let new_columns = self.columns.append(&other.columns);

        let mut new_values: Vec<Cow<Vec<V>>> = Vec::with_capacity(new_columns.len());

        for values in self.ilocs(&lindexer).values {
            new_values.push(values);
        }
        for values in other.ilocs(&rindexer).values {
            new_values.push(values);
        }

        Block::from_cow(new_values,
                        Cow::Owned(Indexer::new(new_index)),
                        Cow::Owned(new_columns))
    }
}
