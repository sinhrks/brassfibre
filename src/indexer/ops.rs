use std::hash::Hash;
use std::ops::{Add, Mul, Sub, Div, Rem, BitAnd, BitOr, BitXor};

use super::Indexer;
use super::super::algos::elemwise::Elemwise;

macro_rules! define_numeric_op {
    ($t:ident, $m:ident) => {

        // Broadcast
        impl<U, O> $t<U> for Indexer<U>
            where U: Clone + Eq + Hash + $t<Output=O>,
                  O: Clone + Eq + Hash {

            type Output = Indexer<O>;
            fn $m(self, _rhs: U) -> Self::Output {
                let new_values: Vec<O> = Elemwise::broadcast_oo(self.values, _rhs, |x, y| x.$m(y));
                Indexer::new(new_values)
            }
        }

        impl<'a, U, O> $t<&'a U> for Indexer<U>
            where U: Clone + Eq + Hash + $t<Output=O>,
                  O: Clone + Eq + Hash {

            type Output = Indexer<O>;
            fn $m(self, _rhs: &U) -> Self::Output {
                let new_values: Vec<O> = Elemwise::broadcast_or(self.values, _rhs, |x, y| x.$m(y));
                Indexer::new(new_values)
            }
        }

        impl<'b, U, O> $t<U> for &'b Indexer<U>
            where U: Clone + Eq + Hash + $t<Output=O>,
                  O: Clone + Eq + Hash {

            // can't use self as impl is for reference?
            type Output = Indexer<O>;
            fn $m(self, _rhs: U) -> Self::Output {
                let new_values: Vec<O> = Elemwise::broadcast_ro(&self.values, _rhs, |x, y| x.$m(y));
                Indexer::new(new_values)
            }
        }

        impl<'a, 'b, U, O> $t<&'a U> for &'b Indexer<U>
            where U: Clone + Eq + Hash + $t<Output=O>,
                  O: Clone + Eq + Hash {

            type Output = Indexer<O>;
            fn $m(self, _rhs: &U) -> Self::Output {
                let new_values: Vec<O> = Elemwise::broadcast_rr(&self.values, _rhs, |x, y| x.$m(y));
                Indexer::new(new_values)
            }
        }

        // Element-wise
        impl<U, O> $t<Indexer<U>> for Indexer<U>
            where U: Clone + Eq + Hash + $t<Output=O>,
                  O: Clone + Eq + Hash {

            type Output = Indexer<O>;
            fn $m(self, _rhs: Self) -> Self::Output {
                let new_values: Vec<O> = Elemwise::elemwise_oo(self.values, _rhs.values, |x, y| x.$m(y));
                Indexer::new(new_values)
            }
        }

        impl<'a, U, O> $t<&'a Indexer<U>> for Indexer<U>
            where U: Clone + Eq + Hash + $t<Output=O>,
                  O: Clone + Eq + Hash {

            type Output = Indexer<O>;
            fn $m(self, _rhs: &Self) -> Self::Output {
                let new_values: Vec<O> = Elemwise::elemwise_or(self.values, &_rhs.values, |x, y| x.$m(y));
                Indexer::new(new_values)
            }
        }

        impl<'b, U, O> $t<Indexer<U>> for &'b Indexer<U>
            where U: Clone + Eq + Hash + $t<Output=O>,
                  O: Clone + Eq + Hash {

            type Output = Indexer<O>;
            fn $m(self, _rhs: Indexer<U>) -> Self::Output {
                let new_values: Vec<O> = Elemwise::elemwise_ro(&self.values, _rhs.values, |x, y| x.$m(y));
                Indexer::new(new_values)
            }
        }

        impl<'a, 'b, U, O> $t<&'a Indexer<U>> for &'b Indexer<U>
            where U: Clone + Eq + Hash + $t<Output=O>,
                  O: Clone + Eq + Hash {

            type Output = Indexer<O>;
            fn $m(self, _rhs: &Indexer<U>) -> Self::Output {
                let new_values: Vec<O> = Elemwise::elemwise_rr(&self.values, &_rhs.values, |x, y| x.$m(y));
                Indexer::new(new_values)
            }
        }
    }
}

define_numeric_op!(Add, add);
define_numeric_op!(Mul, mul);
define_numeric_op!(Sub, sub);
define_numeric_op!(Div, div);
define_numeric_op!(Rem, rem);
define_numeric_op!(BitAnd, bitand);
define_numeric_op!(BitOr, bitor);
define_numeric_op!(BitXor, bitxor);


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

    /*
    ToDo
    #[test]
    fn test_index_ops_str_broadcast() {
        let idx = Indexer::<String>::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        // idx moves by ops
        let exp = Indexer::<String>::new(vec!["ax".to_string(), "bx".to_string(), "cx".to_string()]);
        assert_eq!(idx + "x".to_string(), exp);
    }
    */

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
