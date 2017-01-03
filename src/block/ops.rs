use std::borrow::{Borrow, Cow};
use std::hash::Hash;
use std::ops::{Add, Mul, Sub, Div, Rem, BitAnd, BitOr, BitXor};

use super::Block;
use algos::elemwise::Elemwise;

macro_rules! define_numeric_op {
    ($t:ident, $m:ident) => {

        // Broadcast
        impl<'v, 'i, 'c, V, I, C, O> $t<V> for Block<'v, 'i, 'c, V, I, C>
            where V: Clone + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  C: Clone + Eq + Hash,
                  O: 'v + Clone {

            type Output = Block<'v, 'i, 'c, O, I, C>;
            fn $m(self, _rhs: V) -> Self::Output {
                let mut new_values: Vec<Cow<Vec<O>>> = Vec::with_capacity(self.values.len());
                for value in self.values.into_iter() {
                    let new_value = Elemwise::broadcast_rr(value.as_ref(),
                                                           &_rhs, |x, y| x.$m(y));
                    new_values.push(Cow::Owned(new_value));
                }
                Block::from_cow(new_values, self.index, self.columns)
            }
        }

        impl<'v, 'i, 'c, 'r, V, I, C, O> $t<&'r V> for Block<'v, 'i, 'c, V, I, C>
            where V: Clone + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  C: Clone + Eq + Hash,
                  O: 'v + Clone {

            type Output = Block<'v, 'i, 'c, O, I, C>;
            fn $m(self, _rhs: &V) -> Self::Output {
                let mut new_values: Vec<Cow<Vec<O>>> = Vec::with_capacity(self.values.len());
                for value in self.values.into_iter() {
                    let new_value = Elemwise::broadcast_rr(value.as_ref(),
                                                           _rhs, |x, y| x.$m(y));
                    new_values.push(Cow::Owned(new_value));
                }
                Block::from_cow(new_values, self.index, self.columns)
            }
        }

        impl<'v, 'i, 'c, 'l, V, I, C, O> $t<V> for &'l Block<'v, 'i, 'c, V, I, C>
            where V: Clone + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  C: Clone + Eq + Hash,
                  O: 'l + Clone {

            type Output = Block<'l, 'l, 'l, O, I, C>;
            fn $m(self, _rhs: V) -> Self::Output {
                let mut new_values: Vec<Cow<Vec<O>>> = Vec::with_capacity(self.values.len());
                for value in self.values.iter() {
                    let new_value = Elemwise::broadcast_rr(value.as_ref(), &_rhs, |x, y| x.$m(y));
                    new_values.push(Cow::Owned(new_value));
                }
                Block::from_cow(new_values,
                                Cow::Borrowed(self.index.borrow()),
                                Cow::Borrowed(self.columns.borrow()))
            }
        }

        impl<'v, 'i, 'c, 'l, 'r, V, I, C, O> $t<&'r V> for &'l Block<'v, 'i, 'c, V, I, C>
            where V: Clone + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  C: Clone + Eq + Hash,
                  O: 'l + Clone {

            type Output = Block<'l, 'l, 'l, O, I, C>;
            fn $m(self, _rhs: &V) -> Self::Output {
                let mut new_values: Vec<Cow<Vec<O>>> = Vec::with_capacity(self.values.len());
                for value in self.values.iter() {
                    let new_value = Elemwise::broadcast_rr(value.as_ref(),
                                                           _rhs, |x, y| x.$m(y));
                    new_values.push(Cow::Owned(new_value));
                }
                Block::from_cow(new_values,
                                Cow::Borrowed(self.index.borrow()),
                                Cow::Borrowed(self.columns.borrow()))
            }
        }

        // Element-wise
        impl<'lv, 'li, 'lc, 'rv, 'ri, 'rc, V, I, C, O> $t<Block<'rv, 'ri, 'rc, V, I, C>>
            for Block<'lv, 'li, 'lc, V, I, C>

            where V: Clone + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  C: Clone + Eq + Hash,
                  O: 'lv + Clone {

            type Output = Block<'lv, 'li, 'lc, O, I, C>;
            fn $m(self, _rhs: Block<V, I, C>) -> Self::Output {
                self.assert_binop(&_rhs);
                let mut new_values: Vec<Cow<Vec<O>>> = Vec::with_capacity(self.values.len());
                for (value, rvalue) in self.values.into_iter()
                                           .zip(_rhs.values.into_iter()) {
                    let new_value = Elemwise::elemwise_rr(value.as_ref(),
                                                          rvalue.as_ref(),
                                                          |x, y| x.$m(y));
                    new_values.push(Cow::Owned(new_value));
                }
                Block::from_cow(new_values, self.index, self.columns)
            }
        }

        impl<'lv, 'li, 'lc, 'rv, 'ri, 'rc, 'r, V, I, C, O> $t<&'r Block<'rv, 'ri, 'rc, V, I, C>>
            for Block<'lv, 'li, 'lc, V, I, C>

            where V: Clone + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  C: Clone + Eq + Hash,
                  O: 'lv + Clone {

            type Output = Block<'lv, 'li, 'lc, O, I, C>;
            fn $m(self, _rhs: &'r Block<V, I, C>) -> Self::Output {
                self.assert_binop(&_rhs);
                let mut new_values: Vec<Cow<Vec<O>>> = Vec::with_capacity(self.values.len());
                for (value, rvalue) in self.values.into_iter()
                                           .zip(_rhs.values.iter()) {
                    let new_value = Elemwise::elemwise_rr(value.as_ref(),
                                                          rvalue.as_ref(),
                                                          |x, y| x.$m(y));
                    new_values.push(Cow::Owned(new_value));
                }
                Block::from_cow(new_values, self.index, self.columns)
            }
        }

        impl<'lv, 'li, 'lc, 'rv, 'ri, 'rc, 'l, V, I, C, O> $t<Block<'rv, 'ri, 'rc, V, I, C>>
            for &'l Block<'lv, 'li, 'lc, V, I, C>

            where V: Clone + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  C: Clone + Eq + Hash,
                  O: 'l + Clone {

            type Output = Block<'l, 'l, 'l, O, I, C>;
            fn $m(self, _rhs: Block<V, I, C>) -> Self::Output {
                self.assert_binop(&_rhs);
                let mut new_values: Vec<Cow<Vec<O>>> = Vec::with_capacity(self.values.len());
                for (value, rvalue) in self.values.iter()
                                           .zip(_rhs.values.into_iter()) {
                    let new_value = Elemwise::elemwise_rr(value.as_ref(),
                                                          rvalue.as_ref(), |x, y| x.$m(y));
                    new_values.push(Cow::Owned(new_value));
                }
                Block::from_cow(new_values,
                                Cow::Borrowed(self.index.borrow()),
                                Cow::Borrowed(self.columns.borrow()))
            }
        }

        impl<'lv, 'li, 'lc, 'rv, 'ri , 'rc, 'l, 'r, V, I, C, O> $t<&'r Block<'rv, 'ri, 'rc, V, I, C>>
            for &'l Block<'lv, 'li, 'lc, V, I, C>

            where V: Clone + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  C: Clone + Eq + Hash,
                  O: 'l + Clone {

            type Output = Block<'l, 'l, 'l, O, I, C>;
            fn $m(self, _rhs: &'r Block<V, I, C>) -> Self::Output {
                self.assert_binop(&_rhs);
                let mut new_values: Vec<Cow<Vec<O>>> = Vec::with_capacity(self.values.len());
                for (value, rvalue) in self.values.iter()
                                           .zip(_rhs.values.iter()) {
                    let new_value = Elemwise::elemwise_rr(value.as_ref(),
                                                          rvalue.as_ref(),
                                                          |x, y| x.$m(y));
                    new_values.push(Cow::Owned(new_value));
                }
                Block::from_cow(new_values,
                                Cow::Borrowed(self.index.borrow()),
                                Cow::Borrowed(self.columns.borrow()))
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

    use block::Block;

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
    }

    #[test]
    fn test_block_ops_i64_broadcast_move() {
        let exp = Block::from_col_vec(vec![4, 5, 6, 7, 8, 9],
                                      vec![10, 20, 30], vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
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
    }

    #[test]
    fn test_block_ops_f64_broadcast_move() {
        let exp = Block::from_col_vec(vec![4., 5., 6., 7., 8., 9.],
                                      vec![10, 20, 30], vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
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
    }

    #[test]
    fn test_block_ops_i64_elemwise_move() {
        let exp = Block::from_col_vec(vec![3, 5, 4, 6, 8, 7],
                                      vec![10, 20, 30], vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
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
    }

    #[test]
    fn test_block_ops_f64_elemwise_move() {
        let exp = Block::from_col_vec(vec![3., 5., 4., 6., 8., 7.],
                                      vec![10, 20, 30], vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);

        let res = b + &r;
        assert_eq!(res, exp);
    }

    #[test]
    fn test_block_ops_bool_elemwise_logical() {
        let b = Block::from_col_vec(vec![true, false, true, false, true, false],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![true, true, true, false, false, false],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        // b moves by ops
        let res = b & r;
        let exp = Block::from_col_vec(vec![true, false, true, false, false, false],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![true, false, true, false, true, false],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![true, true, true, false, false, false],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b | r;
        let exp = Block::from_col_vec(vec![true, true, true, false, true, false],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);

        let b = Block::from_col_vec(vec![true, false, true, false, true, false],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![true, true, true, false, false, false],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let res = b ^ r;
        let exp = Block::from_col_vec(vec![false, true, false, false, true, false],
                                      vec![10, 20, 30], vec!["X", "Y"]);
        assert_eq!(res, exp);
    }
}
