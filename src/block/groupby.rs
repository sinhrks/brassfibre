
use num::{Num, Zero, ToPrimitive};
use std::cmp::Ord;
use std::hash::Hash;

use super::Block;
use super::super::algos::grouper::{Grouper};
use super::super::groupby::GroupBy;
use super::super::traits::{Applicable, Aggregator};

////////////////////////////////////////////////////////////////////////////////
// Apply
////////////////////////////////////////////////////////////////////////////////

impl<'a, T, U, V, G, W> Applicable<Block<T, U, V>, Vec<W>, Block<W, G, V>>
    for GroupBy<'a, Block<T, U, V>, G>

    where T: Copy,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash,
          G: Copy + Eq + Hash + Ord,
          W: Copy {

    /// Apply passed function to each group
    fn apply(&self, func: &Fn(&Block<T, U, V>) -> Vec<W>) -> Block<W, G, V> {
        let mut new_values: Vec<W> = Vec::with_capacity(self.grouper.len());

        let groups = self.groups();
        for g in groups.iter() {
            let s = self.get_group(&g);
            new_values.append(&mut func(&s));
        }
        Block::from_row_vec(new_values, groups,
                            self.data.columns.clone())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Aggregation
////////////////////////////////////////////////////////////////////////////////

impl<'a, T, U, V, G> Aggregator for GroupBy<'a, Block<T, U, V>, G>
    where T: Copy + Eq + Hash + Num + Zero + ToPrimitive,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash,
          G: Copy + Eq + Hash + Ord {

    type Kept = Block<T, G, V>;
    type Counted = Block<usize, G, V>;
    type Coerced = Block<f64, G, V>;

    fn sum(&self) -> Self::Kept {
        self.apply(&|x: &Block<T, U, V>| x.sum().values)
    }

    fn count(&self) -> Self::Counted {
        self.apply(&|x: &Block<T, U, V>| x.count().values)
    }

    fn mean(&self) -> Self::Coerced {
        self.apply(&|x: &Block<T, U, V>| x.mean().values)
    }

    fn var(&self) -> Self::Coerced {
        self.apply(&|x: &Block<T, U, V>| x.var().values)
    }

    fn unbiased_var(&self) -> Self::Coerced {
        self.apply(&|x: &Block<T, U, V>| x.unbiased_var().values)
    }

    fn std(&self) -> Self::Coerced {
        self.apply(&|x: &Block<T, U, V>| x.std().values)
    }

    fn unbiased_std(&self) -> Block<f64, G, V> {
        self.apply(&|x: &Block<T, U, V>| x.unbiased_std().values)
    }

}