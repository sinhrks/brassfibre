extern crate num;

use num::{Num};
use std::hash::Hash;
use std::ops::{Add, Mul, Sub, Div, Rem};

use super::Block;

fn broadcast<T>(left: &Vec<Vec<T>>, func: &Fn(&T) -> T) -> Vec<Vec<T>> {
    let mut new_values: Vec<Vec<T>> = Vec::with_capacity(left.len());
    for value in left.iter() {
        let new_value = value.iter().map(func).collect();
        new_values.push(new_value);
    }
    new_values
}

fn elemwise<T>(left: &Vec<Vec<T>>, right: &Vec<Vec<T>>,
            func: &Fn((&T, &T)) -> T) -> Vec<Vec<T>>
    where T: Copy + Num {

    let mut new_values: Vec<Vec<T>> = Vec::with_capacity(left.len());
    for (value, rvalue) in left.iter().zip(right.iter()) {
        let new_value = value.iter()
                             .zip(rvalue.iter())
                             .map(func).collect();
        new_values.push(new_value);
    }
    new_values
}

macro_rules! define_numeric_op {
    ($t:ident, $m:ident) => {

        // Broadcast
        impl<T, U, V> $t<T> for Block<T, U, V>
            where T: Copy + Num,
                  U: Copy + Eq + Hash,
                  V: Copy + Eq + Hash {

            type Output = Block<T, U, V>;
            fn $m(self, _rhs: T) -> Block<T, U, V> {
                let new_values = broadcast(&self.values, &|x| (*x).$m(_rhs));
                Block::from_nested_vec(new_values,
                                       self.index.clone(),
                                       self.columns.clone())
            }
        }

        impl<'a, T, U, V> $t<&'a T> for Block<T, U, V>
            where T: Copy + Num,
                  U: Copy + Eq + Hash,
                  V: Copy + Eq + Hash {

            type Output = Block<T, U, V>;
            fn $m(self, _rhs: &T) -> Block<T, U, V> {
                let new_values = broadcast(&self.values, &|x: &T| (*x).$m(*_rhs));
                Block::from_nested_vec(new_values,
                                       self.index.clone(),
                                       self.columns.clone())
            }
        }

        impl<'b, T, U, V> $t<T> for &'b Block<T, U, V>
            where T: Copy + Num,
                  U: Copy + Eq + Hash,
                  V: Copy + Eq + Hash {

            type Output = Block<T, U, V>;
            fn $m(self, _rhs: T) -> Block<T, U, V> {
                let new_values = broadcast(&self.values, &|x: &T| (*x).$m(_rhs));
                Block::from_nested_vec(new_values,
                                       self.index.clone(),
                                       self.columns.clone())
            }
        }

        impl<'a, 'b, T, U, V> $t<&'a T> for &'b Block<T, U, V>
            where T: Copy + Num,
                  U: Copy + Eq + Hash,
                  V: Copy + Eq + Hash {

            type Output = Block<T, U, V>;
            fn $m(self, _rhs: &T) -> Block<T, U, V> {
                let new_values = broadcast(&self.values, &|x: &T| (*x).$m(*_rhs));
                Block::from_nested_vec(new_values,
                                       self.index.clone(),
                                       self.columns.clone())
            }
        }

        // Element-wise
        impl<T, U, V> $t<Block<T, U, V>> for Block<T, U, V>
            where T: Copy + Num,
                  U: Copy + Eq + Hash,
                  V: Copy + Eq + Hash {

            type Output = Block<T, U, V>;
            fn $m(self, _rhs: Block<T, U, V>) -> Block<T, U, V> {
                self.assert_binop(&_rhs);
                let new_values = elemwise(&self.values, &_rhs.values,
                                          &|(x, y)| (*x).$m(*y));
                Block::from_nested_vec(new_values,
                                       self.index.clone(),
                                       self.columns.clone())
            }
        }

        impl<'a, T, U, V> $t<&'a Block<T, U, V>> for Block<T, U, V>
            where T: Copy + Num,
                  U: Copy + Eq + Hash,
                  V: Copy + Eq + Hash {

            type Output = Block<T, U, V>;
            fn $m(self, _rhs: &Block<T, U, V>) -> Block<T, U, V> {
                self.assert_binop(&_rhs);
                let new_values = elemwise(&self.values, &_rhs.values,
                                          &|(x, y)| (*x).$m(*y));
                Block::from_nested_vec(new_values,
                                       self.index.clone(),
                                       self.columns.clone())
            }
        }

        impl<'b, T, U, V> $t<Block<T, U, V>> for &'b Block<T, U, V>
            where T: Copy + Num,
                  U: Copy + Eq + Hash,
                  V: Copy + Eq + Hash {

            type Output = Block<T, U, V>;
            fn $m(self, _rhs: Block<T, U, V>) -> Block<T, U, V> {
                self.assert_binop(&_rhs);
                let new_values = elemwise(&self.values, &_rhs.values,
                                          &|(x, y)| (*x).$m(*y));
                Block::from_nested_vec(new_values,
                                       self.index.clone(),
                                       self.columns.clone())
            }
        }

        impl<'a, 'b, T, U, V> $t<&'a Block<T, U, V>> for &'b Block<T, U, V>
            where T: Copy + Num,
                  U: Copy + Eq + Hash,
                  V: Copy + Eq + Hash {

            type Output = Block<T, U, V>;
            fn $m(self, _rhs: &Block<T, U, V>) -> Block<T, U, V> {
                self.assert_binop(&_rhs);
                let new_values = elemwise(&self.values, &_rhs.values,
                                          &|(x, y)| (*x).$m(*y));
                Block::from_nested_vec(new_values,
                                       self.index.clone(),
                                       self.columns.clone())
            }
        }
    }
}

define_numeric_op!(Add, add);
define_numeric_op!(Mul, mul);
define_numeric_op!(Sub, sub);
define_numeric_op!(Div, div);
define_numeric_op!(Rem, rem);


#[cfg(test)]
mod tests {

    use super::super::Block;
    use super::super::super::traits::ColIndexer;

    #[test]
    fn test_block_ops_i64_broadcast() {
        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        // b moves by ops
        let res = b + 3;
        let exp = Block::from_col_vec(vec![4, 5, 6, 7, 8, 9],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b * 2;
        let exp = Block::from_col_vec(vec![2, 4, 6, 8, 10, 12],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b - 3;
        let exp = Block::from_col_vec(vec![-2, -1, 0, 1, 2, 3],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b / 2;
        let exp = Block::from_col_vec(vec![0, 1, 1, 2, 2, 3],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b % 2;
        let exp = Block::from_col_vec(vec![1, 0, 1, 0, 1, 0],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_ops_i64_broadcast_refs() {
        let exp = Block::from_col_vec(vec![4, 5, 6, 7, 8, 9],
                                      vec![10, 20, 30], vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = &b + 3;
        assert_eq!(res, exp);

        let res = &b + &3;
        assert_eq!(res, exp);

        let res = b + &3;
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_ops_f64_broadcast() {
        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        // b moves by ops
        let res = b + 3.;
        let exp = Block::from_col_vec(vec![4., 5., 6., 7., 8., 9.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b * 2.;
        let exp = Block::from_col_vec(vec![2., 4., 6., 8., 10., 12.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b - 3.;
        let exp = Block::from_col_vec(vec![-2., -1., 0., 1., 2., 3.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b / 2.;
        let exp = Block::from_col_vec(vec![0.5, 1., 1.5, 2., 2.5, 3.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b % 2.;
        let exp = Block::from_col_vec(vec![1., 0., 1., 0., 1., 0.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_ops_f64_broadcast_refs() {
        let exp = Block::from_col_vec(vec![4., 5., 6., 7., 8., 9.],
                                      vec![10, 20, 30], vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = &b + 3.;
        assert_eq!(res, exp);

        let res = &b + &3.;
        assert_eq!(res, exp);

        let res = b + &3.;
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_ops_i64_elemwise() {
        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        // b moves by ops
        let res = b + r;
        let exp = Block::from_col_vec(vec![3, 5, 4, 6, 8, 7],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b * r;
        let exp = Block::from_col_vec(vec![2, 6, 3, 8, 15, 6],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b - r;
        let exp = Block::from_col_vec(vec![-1, -1, 2, 2, 2, 5],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b / r;
        let exp = Block::from_col_vec(vec![0, 0, 3, 2, 1, 6],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b % r;
        let exp = Block::from_col_vec(vec![1, 2, 0, 0, 2, 0],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_ops_i64_elemwise_refs() {
        let exp = Block::from_col_vec(vec![3, 5, 4, 6, 8, 7],
                                      vec![10, 20, 30], vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = &b + r;
        assert_eq!(res, exp);

        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = &b + &r;
        assert_eq!(res, exp);

        let res = b + &r;
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_ops_f64_elemwise() {
        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        // b moves by ops
        let res = b + r;
        let exp = Block::from_col_vec(vec![3., 5., 4., 6., 8., 7.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b * r;
        let exp = Block::from_col_vec(vec![2., 6., 3., 8., 15., 6.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b - r;
        let exp = Block::from_col_vec(vec![-1., -1., 2., 2., 2., 5.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b / r;
        let exp = Block::from_col_vec(vec![0.5, 0.6666666666666666, 3.,
                                           2., 1.6666666666666667, 6.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b % r;
        let exp = Block::from_col_vec(vec![1., 2., 0., 0., 2., 0.],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_ops_f64_elemwise_refs() {
        let exp = Block::from_col_vec(vec![3., 5., 4., 6., 8., 7.],
                                      vec![10, 20, 30], vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);

        let res = &b + r;
        assert_eq!(res, exp);

        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = &b + &r;
        assert_eq!(res, exp);

        let mut result = b + &r;
        assert_eq!(res, exp);
    }
}
