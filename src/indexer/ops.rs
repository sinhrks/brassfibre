use std::hash::Hash;
use std::ops::{Add, Mul, Sub, Div, Rem};

use super::Indexer;

macro_rules! define_numric_op {
    ($t:ident $m:ident) => {

        // Broadcast
        impl<U, O> $t<U> for Indexer<U>
            where U: Clone + Eq + Hash + $t<Output=O>,
                  O: Clone + Eq + Hash {

            type Output = Indexer<O>;
            fn $m(self, _rhs: U) -> Self::Output {
                let new_values: Vec<O> = self.values.into_iter()
                                                    .map(|x: U| x.$m(_rhs.clone()))
                                                    .collect();
                Indexer::new(new_values)
            }
        }

        impl<'a, U, O> $t<&'a U> for Indexer<U>
            where U: Clone + Eq + Hash + $t<Output=O>,
                  O: Clone + Eq + Hash {

            type Output = Indexer<O>;
            fn $m(self, _rhs: &U) -> Self::Output {
                let new_values: Vec<O> = self.values.into_iter()
                                                    .map(|x: U| x.$m((*_rhs).clone()))
                                                    .collect();
                Indexer::new(new_values)
            }
        }

        impl<'b, U, O> $t<U> for &'b Indexer<U>
            where U: Clone + Eq + Hash + $t<Output=O>,
                  O: Clone + Eq + Hash {

            // can't use self as impl is for reference?
            type Output = Indexer<O>;
            fn $m(self, _rhs: U) -> Self::Output {
                let new_values: Vec<O> = self.values.iter()
                                                    .map(|x: &U| (*x).clone().$m(_rhs.clone()))
                                                    .collect();
                Indexer::new(new_values)
            }
        }

        impl<'a, 'b, U, O> $t<&'a U> for &'b Indexer<U>
            where U: Clone + Eq + Hash + $t<Output=O>,
                  O: Clone + Eq + Hash {

            type Output = Indexer<O>;
            fn $m(self, _rhs: &U) -> Self::Output {
                let new_values: Vec<O> = self.values.iter()
                                                    .map(|x: &U| (*x).clone().$m((*_rhs).clone()))
                                                    .collect();
                Indexer::new(new_values)
            }
        }

        // Element-wise
        impl<U, O> $t<Indexer<U>> for Indexer<U>
            where U: Clone + Eq + Hash + $t<Output=O>,
                  O: Clone + Eq + Hash {

            type Output = Indexer<O>;
            fn $m(self, _rhs: Self) -> Self::Output {
                let new_values: Vec<O> = self.values.into_iter()
                                                    .zip(_rhs.values.into_iter())
                                                    .map(|(x, y)| x.$m(y))
                                                    .collect();
                Indexer::new(new_values)
            }
        }

        impl<'a, U, O> $t<&'a Indexer<U>> for Indexer<U>
            where U: Clone + Eq + Hash + $t<Output=O>,
                  O: Clone + Eq + Hash {

            type Output = Indexer<O>;
            fn $m(self, _rhs: &Self) -> Self::Output {
                let new_values: Vec<O> = self.values.into_iter()
                                                    .zip(_rhs.values.iter())
                                                    .map(|(x, y)| x.$m(y.clone()))
                                                    .collect();
                Indexer::new(new_values)
            }
        }

        impl<'b, U, O> $t<Indexer<U>> for &'b Indexer<U>
            where U: Clone + Eq + Hash + $t<Output=O>,
                  O: Clone + Eq + Hash {

            type Output = Indexer<O>;
            fn $m(self, _rhs: Indexer<U>) -> Self::Output {
                let new_values: Vec<O> = self.values.iter()
                                                    .zip(_rhs.values.into_iter())
                                                    .map(|(x, y)| x.clone().$m(y))
                                                    .collect();
                Indexer::new(new_values)
            }
        }

        impl<'a, 'b, U, O> $t<&'a Indexer<U>> for &'b Indexer<U>
            where U: Clone + Eq + Hash + $t<Output=O>,
                  O: Clone + Eq + Hash {

            type Output = Indexer<O>;
            fn $m(self, _rhs: &Indexer<U>) -> Self::Output {
                let new_values: Vec<O> = self.values.iter()
                                                    .zip(_rhs.values.iter())
                                                    .map(|(x, y)| x.clone().$m(y.clone()))
                                                    .collect();
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
