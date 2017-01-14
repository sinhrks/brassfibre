use std::borrow::{Borrow, Cow};
use std::hash::Hash;
use std::ops::{Add, Mul, Sub, Div, Rem, BitAnd, BitOr, BitXor};

use super::Series;
use nullvec::prelude::dev::algos::Elemwise;

macro_rules! define_numeric_op {
    ($t:ident, $m:ident) => {

        // Broadcast
        impl<'v, 'i, V, I, O> $t<V> for Series<'v, 'i, V, I>
            where V: Clone + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  O: 'v + Clone {

            type Output = Series<'v, 'i, O, I>;
            fn $m(self, _rhs: V) -> Self::Output {
                // binary ops doesn't require value's ownership
                let new_values: Vec<O> = Elemwise::broadcast_ro(self.values.as_ref(),
                                                                _rhs, |x, y| x.$m(y));
                // self is moved, pass index to new instance
                Series::from_cow(Cow::Owned(new_values), self.index)
            }
        }

        impl<'v, 'i, 'r, V, I, O> $t<&'r V> for Series<'v, 'i, V, I>
            where V: Clone + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  O: 'v + Clone {

            type Output = Series<'v, 'i, O, I>;
            fn $m(self, _rhs: &'r V) -> Self::Output {
                let new_values: Vec<O> = Elemwise::broadcast_rr(self.values.as_ref(),
                                                                _rhs, |x, y| x.$m(y));
                Series::from_cow(Cow::Owned(new_values), self.index)
            }
        }

        impl<'v, 'i, 'l, V, I, O> $t<V> for &'l Series<'v, 'i, V, I>
            where V: Clone + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  O: 'l + Clone {

            type Output = Series<'l, 'l, O, I>;
            fn $m(self, _rhs: V) -> Self::Output {
                let new_values: Vec<O> = Elemwise::broadcast_ro(self.values.as_ref(),
                                                                _rhs, |x, y| x.$m(y));
                Series::from_cow(Cow::Owned(new_values),
                                 Cow::Borrowed(self.index.borrow()))
            }
        }

        impl<'v, 'i, 'l, 'r, V, I, O> $t<&'r V> for &'l Series<'v, 'i, V, I>
            where V: Clone + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  O: 'l + Clone {

            type Output = Series<'l, 'l, O, I>;
            fn $m(self, _rhs: &'r V) -> Self::Output {
                let new_values: Vec<O> = Elemwise::broadcast_rr(self.values.as_ref(),
                                                                _rhs, |x, y| x.$m(y));
                Series::from_cow(Cow::Owned(new_values),
                                 Cow::Borrowed(self.index.borrow()))
            }
        }

        // Element-wise
        impl<'lv, 'rv, 'li, 'ri, V, I, O> $t<Series<'rv, 'ri, V, I>> for Series<'lv, 'li, V, I>
            where V: Clone + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  O: 'lv + Clone {

            type Output = Series<'lv, 'li, O, I>;
            fn $m(self, _rhs: Series<V, I>) -> Self::Output {
                self.assert_binop(&_rhs);
                let new_values: Vec<O> = Elemwise::elemwise_oo(self.values.into_owned(),
                                                               _rhs.values.into_owned(),
                                                               |x, y| x.$m(y));
                Series::from_cow(Cow::Owned(new_values), self.index)
            }
        }

        impl<'lv, 'rv, 'li, 'ri, 'r, V, I, O> $t<&'r Series<'rv, 'ri, V, I>>
            for Series<'lv, 'li, V, I>
            where V: Clone + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  O: 'lv + Clone {

            type Output = Series<'lv, 'li, O, I>;
            fn $m(self, _rhs: &'r Series<V, I>) -> Self::Output {
                self.assert_binop(&_rhs);
                let new_values: Vec<O> = Elemwise::elemwise_or(self.values.into_owned(),
                                                               &_rhs.values.as_ref(),
                                                               |x, y| x.$m(y));
                Series::from_cow(Cow::Owned(new_values), self.index)
            }
        }

        impl<'lv, 'rv, 'li, 'ri, 'l, V, I, O> $t<Series<'rv, 'ri, V, I>>
            for &'l Series<'lv, 'li, V, I>
            where V: Clone + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  O: 'l + Clone {
            // cannot use 'n lifetime for associated dtype (uncostrained)
            type Output = Series<'l, 'l, O, I>;
            fn $m(self, _rhs: Series<V, I>) -> Self::Output {
                self.assert_binop(&_rhs);

                // ToDo: match with Cow::Owned / Borrowed
                let new_values: Vec<O> = Elemwise::elemwise_ro(&self.values.as_ref(),
                                                               _rhs.values.into_owned(),
                                                               |x, y| x.$m(y));
                Series::from_cow(Cow::Owned(new_values),
                                 Cow::Borrowed(self.index.borrow()))
            }
        }

        impl<'lv, 'rv, 'li, 'ri, 'l, 'r, V, I, O> $t<&'r Series<'rv, 'ri, V, I>>
            for &'l Series<'lv, 'li, V, I>

            where V: Clone + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  O: 'l + Clone {

            type Output = Series<'l, 'l, O, I>;
            fn $m(self, _rhs: &'r Series<V, I>) -> Self::Output {
                self.assert_binop(&_rhs);
                let new_values: Vec<O> = Elemwise::elemwise_rr(&self.values,
                                                               &_rhs.values,
                                                               |x, y| x.$m(y));
                Series::from_cow(Cow::Owned(new_values),
                                 Cow::Borrowed(self.index.borrow()))
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

    use super::super::Series;

    #[test]
    fn test_series_ops_i64_broadcast() {
        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        // s moves by ops
        let result = s + 3;
        let exp = Series::<i64, i64>::new(vec![4, 5, 6], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let result = s * 2;
        let exp = Series::<i64, i64>::new(vec![2, 4, 6], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let result = s - 3;
        let exp = Series::<i64, i64>::new(vec![-2, -1, 0], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let result = s / 2;
        let exp = Series::<i64, i64>::new(vec![0, 1, 1], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let result = s % 2;
        let exp = Series::<i64, i64>::new(vec![1, 0, 1], vec![10, 20, 30]);
        assert_eq!(result, exp);
    }

    #[test]
    fn test_series_ops_i64_broadcast_refs() {
        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);

        // borrow
        let result = &s + 3;
        let exp = Series::<i64, i64>::new(vec![4, 5, 6], vec![10, 20, 30]);
        assert_eq!(result, exp);

        // borrow
        let result = &s + &3;
        let exp = Series::<i64, i64>::new(vec![4, 5, 6], vec![10, 20, 30]);
        assert_eq!(result, exp);
    }

    #[test]
    fn test_series_ops_i64_broadcast_move() {
        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);

        // move
        let result = s + &3;
        let exp = Series::<i64, i64>::new(vec![4, 5, 6], vec![10, 20, 30]);
        assert_eq!(result, exp);
    }

    #[test]
    fn test_series_ops_f64_broadcast() {
        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        // s moves by ops
        let result = s + 3.;
        let exp = Series::<f64, i64>::new(vec![4., 5., 6.], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let result = s * 2.;
        let exp = Series::<f64, i64>::new(vec![2., 4., 6.], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let result = s - 3.;
        let exp = Series::<f64, i64>::new(vec![-2., -1., 0.], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let result = s / 2.;
        let exp = Series::<f64, i64>::new(vec![0.5, 1., 1.5], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let result = s % 2.;
        let exp = Series::<f64, i64>::new(vec![1., 0., 1.], vec![10, 20, 30]);
        assert_eq!(result, exp);
    }

    #[test]
    fn test_series_ops_f64_broadcast_refs() {
        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);

        let result = &s + 3.;
        let exp = Series::<f64, i64>::new(vec![4., 5., 6.], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let result = &s + &3.;
        let exp = Series::<f64, i64>::new(vec![4., 5., 6.], vec![10, 20, 30]);
        assert_eq!(result, exp);
    }

    #[test]
    fn test_series_ops_f64_broadcast_move() {
        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);

        let result = s + &3.;
        let exp = Series::<f64, i64>::new(vec![4., 5., 6.], vec![10, 20, 30]);
        assert_eq!(result, exp);
    }

    #[test]
    fn test_series_ops_bool_broadcast_logical() {
        let s = Series::<bool, i64>::new(vec![true, false, true], vec![10, 20, 30]);
        // s moves by ops
        let result = s & true;
        let exp = Series::<bool, i64>::new(vec![true, false, true], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<bool, i64>::new(vec![true, false, true], vec![10, 20, 30]);
        // s moves by ops
        let result = s | true;
        let exp = Series::<bool, i64>::new(vec![true, true, true], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<bool, i64>::new(vec![true, false, true], vec![10, 20, 30]);
        // s moves by ops
        let result = s ^ true;
        let exp = Series::<bool, i64>::new(vec![false, true, false], vec![10, 20, 30]);
        assert_eq!(result, exp);
    }

    #[test]
    fn test_series_ops_i64_elemwise() {
        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);
        // s moves by ops
        let result = s + r;
        let exp = Series::<i64, i64>::new(vec![2, 5, 5], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);
        let result = s * r;
        let exp = Series::<i64, i64>::new(vec![1, 6, 6], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);
        let result = s - r;
        let exp = Series::<i64, i64>::new(vec![0, -1, 1], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);
        let result = s / r;
        let exp = Series::<i64, i64>::new(vec![1, 0, 1], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);
        let result = s % r;
        let exp = Series::<i64, i64>::new(vec![0, 2, 1], vec![10, 20, 30]);
        assert_eq!(result, exp);
    }

    #[test]
    fn test_series_ops_i64_elemwise_refs() {
        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);

        let result = &s + r;
        let exp = Series::<i64, i64>::new(vec![2, 5, 5], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);
        let result = &s + &r;
        let exp = Series::<i64, i64>::new(vec![2, 5, 5], vec![10, 20, 30]);
        assert_eq!(result, exp);
    }

    #[test]
    fn test_series_ops_i64_elemwise_move() {
        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);

        let result = s + &r;
        let exp = Series::<i64, i64>::new(vec![2, 5, 5], vec![10, 20, 30]);
        assert_eq!(result, exp);
    }

    #[test]
    fn test_series_ops_f64_elemwise() {
        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);
        // s moves by ops
        let result = s + r;
        let exp = Series::<f64, i64>::new(vec![2., 5., 5.], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);
        let result = s * r;
        let exp = Series::<f64, i64>::new(vec![1., 6., 6.], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);
        let result = s - r;
        let exp = Series::<f64, i64>::new(vec![0., -1., 1.], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);
        let result = s / r;
        let exp = Series::<f64, i64>::new(vec![1., 0.6666666666666666, 1.5], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);
        let result = s % r;
        let exp = Series::<f64, i64>::new(vec![0., 2., 1.], vec![10, 20, 30]);
        assert_eq!(result, exp);
    }

    #[test]
    fn test_series_ops_f64_elemwise_refs() {
        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);

        let result = &s + r;
        let exp = Series::<f64, i64>::new(vec![2., 5., 5.], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);
        let result = &s + &r;
        let exp = Series::<f64, i64>::new(vec![2., 5., 5.], vec![10, 20, 30]);
        assert_eq!(result, exp);
    }

    #[test]
    fn test_series_ops_f64_elemwise_move() {
        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);

        let result = s + &r;
        let exp = Series::<f64, i64>::new(vec![2., 5., 5.], vec![10, 20, 30]);
        assert_eq!(result, exp);
    }

    #[test]
    fn test_series_ops_bool_elemwise_logical() {
        let s = Series::<bool, i64>::new(vec![true, false, true], vec![10, 20, 30]);
        let r = Series::<bool, i64>::new(vec![false, true, true], vec![10, 20, 30]);
        // s moves by ops
        let result = s & r;
        let exp = Series::<bool, i64>::new(vec![false, false, true], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<bool, i64>::new(vec![true, false, true], vec![10, 20, 30]);
        let r = Series::<bool, i64>::new(vec![false, true, true], vec![10, 20, 30]);
        // s moves by ops
        let result = s | r;
        let exp = Series::<bool, i64>::new(vec![true, true, true], vec![10, 20, 30]);
        assert_eq!(result, exp);

        let s = Series::<bool, i64>::new(vec![true, false, true], vec![10, 20, 30]);
        let r = Series::<bool, i64>::new(vec![false, true, true], vec![10, 20, 30]);
        // s moves by ops
        let result = s ^ r;
        let exp = Series::<bool, i64>::new(vec![true, true, false], vec![10, 20, 30]);
        assert_eq!(result, exp);
    }
}
