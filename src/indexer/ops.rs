extern crate num;

use num::{Num};
use std::hash::Hash;
use std::ops::{Add, Mul, Sub, Div, Rem};

use super::Indexer;

// definition is the same as Series. Move to computation?
fn elemwise<T>(left: &Vec<T>, right: &Vec<T>,
            func: &Fn((&T, &T)) -> T) -> Vec<T>
    where T: Copy + Num {
    left.iter()
        .zip(right.iter())
        .map(func).collect()
}

macro_rules! define_numric_op {
    ($t:ident $m:ident) => {

        // Broadcast
        impl<U> $t<U> for Indexer<U>
            where U: Copy + Eq + Hash + Num {

            type Output = Self;
            fn $m(self, _rhs: U) -> Self {
                let new_values = self.values.iter().map(|x: &U| (*x).$m(_rhs)).collect();
                Indexer::new(new_values)
            }
        }

        impl<'a, U> $t<&'a U> for Indexer<U>
            where U: Copy + Eq + Hash + Num {

            type Output = Self;
            fn $m(self, _rhs: &U) -> Self {
                let new_values = self.values.iter().map(|x: &U| (*x).$m(*_rhs)).collect();
                Indexer::new(new_values)
            }
        }

        impl<'b, U> $t<U> for &'b Indexer<U>
            where U: Copy + Eq + Hash + Num {

            // can't use self as impl is for reference?
            type Output = Indexer<U>;
            fn $m(self, _rhs: U) -> Indexer<U> {
                let new_values = self.values.iter().map(|x: &U| (*x).$m(_rhs)).collect();
                Indexer::new(new_values)
            }
        }

        impl<'a, 'b, U> $t<&'a U> for &'b Indexer<U>
            where U: Copy + Eq + Hash + Num {

            type Output = Indexer<U>;
            fn $m(self, _rhs: &U) -> Indexer<U> {
                let new_values = self.values.iter().map(|x: &U| (*x).$m(*_rhs)).collect();
                Indexer::new(new_values)
            }
        }

        // Element-wise
        impl<U> $t<Indexer<U>> for Indexer<U>
            where U: Copy + Eq + Hash + Num {

            type Output = Self;
            fn $m(self, _rhs: Self) -> Self {
                let new_values = elemwise(&self.values, &_rhs.values, &|(x, y)| (*x).$m(*y));
                Indexer::new(new_values)
            }
        }

        impl<'a, U> $t<&'a Indexer<U>> for Indexer<U>
            where U: Copy + Eq + Hash + Num {

            type Output = Self;
            fn $m(self, _rhs: &Self) -> Self {
                let new_values = elemwise(&self.values, &_rhs.values, &|(x, y)| (*x).$m(*y));
                Indexer::new(new_values)
            }
        }

        impl<'b, U> $t<Indexer<U>> for &'b Indexer<U>
            where U: Copy + Eq + Hash + Num {

            type Output = Indexer<U>;
            fn $m(self, _rhs: Indexer<U>) -> Indexer<U> {
                let new_values = elemwise(&self.values, &_rhs.values, &|(x, y)| (*x).$m(*y));
                Indexer::new(new_values)
            }
        }

        impl<'a, 'b, U> $t<&'a Indexer<U>> for &'b Indexer<U>
            where U: Copy + Eq + Hash + Num {

            type Output = Indexer<U>;
            fn $m(self, _rhs: &Indexer<U>) -> Indexer<U> {
                let new_values = elemwise(&self.values, &_rhs.values, &|(x, y)| (*x).$m(*y));
                Indexer::new(new_values)
            }
        }
    }
}

define_numric_op!(Add add);
define_numric_op!(Mul mul);
define_numric_op!(Sub sub);
define_numric_op!(Div div);
define_numric_op!(Rem rem);


#[cfg(test)]
mod tests {

    use super::super::Indexer;

    #[test]
    fn test_index_ops_i64_broadcast() {
        let idx = Indexer::<i64>::new(vec![1, 2, 3]);
        // idx moves by ops
        assert_eq!(&(idx + 3).values, &vec![4, 5, 6]);

        let idx = Indexer::<i64>::new(vec![1, 2, 3]);
        assert_eq!(&(idx * 2).values, &vec![2, 4, 6]);

        let idx = Indexer::<i64>::new(vec![1, 2, 3]);
        assert_eq!(&(idx - 3).values, &vec![-2, -1, 0]);

        let idx = Indexer::<i64>::new(vec![1, 2, 3]);
        assert_eq!(&(idx / 2).values, &vec![0, 1, 1]);

        let idx = Indexer::<i64>::new(vec![1, 2, 3]);
        assert_eq!(&(idx % 2).values, &vec![1, 0, 1]);
    }

    #[test]
    fn test_index_ops_i64_broadcast_refs() {
        let idx = Indexer::<i64>::new(vec![1, 2, 3]);
        assert_eq!(&(&idx + 3).values, &vec![4, 5, 6]);
        assert_eq!(&(&idx + &3).values, &vec![4, 5, 6]);
        assert_eq!(&(idx + &3).values, &vec![4, 5, 6]);
    }

    #[test]
    fn test_index_ops_i64_elemwise() {
        let idx = Indexer::<i64>::new(vec![1, 2, 3]);
        let r = Indexer::<i64>::new(vec![1, 3, 2]);
        // idx moves by ops
        assert_eq!(&(idx + r).values, &vec![2, 5, 5]);

        let idx = Indexer::<i64>::new(vec![1, 2, 3]);
        let r = Indexer::<i64>::new(vec![1, 3, 2]);
        assert_eq!(&(idx * r).values, &vec![1, 6, 6]);

        let idx = Indexer::<i64>::new(vec![1, 2, 3]);
        let r = Indexer::<i64>::new(vec![1, 3, 2]);
        assert_eq!(&(idx - r).values, &vec![0, -1, 1]);

        let idx = Indexer::<i64>::new(vec![1, 2, 3]);
        let r = Indexer::<i64>::new(vec![1, 3, 2]);
        assert_eq!(&(idx / r).values, &vec![1, 0, 1]);

        let idx = Indexer::<i64>::new(vec![1, 2, 3]);
        let r = Indexer::<i64>::new(vec![1, 3, 2]);
        assert_eq!(&(idx % r).values, &vec![0, 2, 1]);
    }

    #[test]
    fn test_index_ops_i64_elemwise_refs() {
        let idx = Indexer::<i64>::new(vec![1, 2, 3]);
        let r = Indexer::<i64>::new(vec![1, 3, 2]);

        assert_eq!(&(&idx + r).values, &vec![2, 5, 5]);

        let r = Indexer::<i64>::new(vec![1, 3, 2]);
        assert_eq!(&(&idx + &r).values, &vec![2, 5, 5]);
        assert_eq!(&(idx + &r).values, &vec![2, 5, 5]);
    }
}
