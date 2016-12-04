use std::borrow::{Borrow, Cow};
use std::hash::Hash;
use std::ops::{Add, Mul, Sub, Div, Rem};

use super::Series;

macro_rules! define_numric_op {
    ($t:ident, $m:ident) => {

        // Broadcast
        impl<'i, V, I, O> $t<V> for Series<'i, V, I>
            where V: Copy + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  O: Copy {

            type Output = Series<'i, O, I>;
            fn $m(self, _rhs: V) -> Self::Output {
                let new_values: Vec<O> = self.values.into_iter()
                                                    .map(|x: V| x.$m(_rhs))
                                                    .collect();
                // self is moved, pass index to new instance
                Series::from_cow(new_values, self.index)
            }
        }

        impl<'i, 'r, V, I, O> $t<&'r V> for Series<'i, V, I>
            where V: Copy + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  O: Copy {

            type Output = Series<'i, O, I>;
            fn $m(self, _rhs: &'r V) -> Self::Output {
                let new_values: Vec<O> = self.values.into_iter()
                                                    .map(|x: V| x.$m((*_rhs).clone()))
                                                    .collect();
                Series::from_cow(new_values, self.index)
            }
        }

        impl<'i, 'l, V, I, O> $t<V> for &'l Series<'i, V, I>
            where V: Copy + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  O: Copy {

            type Output = Series<'l, O, I>;
            fn $m(self, _rhs: V) -> Self::Output {
                let new_values: Vec<O> = self.values.iter()
                                                    .map(|x: &V| (*x).$m(_rhs))
                                                    .collect();
                Series::from_cow(new_values, Cow::Borrowed(self.index.borrow()))
            }
        }

        impl<'i, 'l, 'r, V, I, O> $t<&'r V> for &'l Series<'i, V, I>
            where V: Copy + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  O: Copy {

            type Output = Series<'l, O, I>;
            fn $m(self, _rhs: &'r V) -> Self::Output {
                let new_values: Vec<O> = self.values.iter()
                                                    .map(|x: &V| (*x).$m(*_rhs))
                                                    .collect();
                Series::from_cow(new_values, Cow::Borrowed(self.index.borrow()))
            }
        }

        // Element-wise
        impl<'li, 'ri, V, I, O> $t<Series<'ri, V, I>> for Series<'li, V, I>
            where V: Copy + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  O: Copy {

            type Output = Series<'li, O, I>;
            fn $m(self, _rhs: Series<V, I>) -> Self::Output {
                self.assert_binop(&_rhs);
                let new_values: Vec<O> = self.values.into_iter()
                                                    .zip(_rhs.values.into_iter())
                                                    .map(|(x, y)| x.$m(y))
                                                    .collect();
                Series::from_cow(new_values, self.index)
            }
        }

        impl<'li, 'ri, 'r, V, I, O> $t<&'r Series<'ri, V, I>> for Series<'li, V, I>
            where V: Copy + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  O: Copy {

            type Output = Series<'li, O, I>;
            fn $m(self, _rhs: &'r Series<V, I>) -> Self::Output {
                self.assert_binop(&_rhs);
                let new_values: Vec<O> = self.values.into_iter()
                                                    .zip(_rhs.values.iter())
                                                    .map(|(x, &y)| x.$m(y.clone()))
                                                    .collect();
                Series::from_cow(new_values, self.index)
            }
        }

        impl<'li, 'ri, 'l, V, I, O> $t<Series<'ri, V, I>> for &'l Series<'li, V, I>
            where V: Copy + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  O: Copy {

            type Output = Series<'l, O, I>;
            fn $m(self, _rhs: Series<V, I>) -> Self::Output {
                self.assert_binop(&_rhs);
                let new_values: Vec<O> = self.values.iter()
                                                    .zip(_rhs.values.into_iter())
                                                    .map(|(&x, y)| x.clone().$m(y))
                                                    .collect();
                Series::from_cow(new_values, Cow::Borrowed(self.index.borrow()))
            }
        }

        impl<'li, 'ri, 'l, 'r, V, I, O> $t<&'r Series<'ri, V, I>> for &'l Series<'li, V, I>
            where V: Copy + $t<Output=O>,
                  I: Clone + Eq + Hash,
                  O: Copy {

            type Output = Series<'l, O, I>;
            fn $m(self, _rhs: &'r Series<V, I>) -> Self::Output {
                self.assert_binop(&_rhs);
                let new_values: Vec<O> = self.values.iter()
                                                    .zip(_rhs.values.iter())
                                                    .map(|(&x, &y)| x.clone().$m(y.clone()))
                                                    .collect();
                Series::from_cow(new_values, Cow::Borrowed(self.index.borrow()))
            }
        }
    }
}

define_numric_op!(Add, add);
define_numric_op!(Mul, mul);
define_numric_op!(Sub, sub);
define_numric_op!(Div, div);
define_numric_op!(Rem, rem);

#[cfg(test)]
mod tests {

    use super::super::Series;

    #[test]
    fn test_series_ops_i64_broadcast() {
        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        // s moves by ops
        let result = s + 3;
        assert_eq!(result.values, vec![4, 5, 6]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let result = s * 2;
        assert_eq!(result.values, vec![2, 4, 6]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let result = s - 3;
        assert_eq!(result.values, vec![-2, -1, 0]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let result = s / 2;
        assert_eq!(result.values, vec![0, 1, 1]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let result = s % 2;
        assert_eq!(result.values, vec![1, 0, 1]);
        assert_eq!(result.index.values, vec![10, 20, 30]);
    }

    #[test]
    fn test_series_ops_i64_broadcast_refs() {
        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);

        // borrow
        let result = &s + 3;
        assert_eq!(result.values, vec![4, 5, 6]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        // borrow
        let result = &s + &3;
        assert_eq!(result.values, vec![4, 5, 6]);
        assert_eq!(result.index.values, vec![10, 20, 30]);
    }

    #[test]
    fn test_series_ops_i64_broadcast_move() {
        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);

        // move
        let result = s + &3;
        assert_eq!(result.values, vec![4, 5, 6]);
        assert_eq!(result.index.values, vec![10, 20, 30]);
    }

    #[test]
    fn test_series_ops_f64_broadcast() {
        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        // s moves by ops
        let result = s + 3.;
        assert_eq!(result.values, vec![4., 5., 6.]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let result = s * 2.;
        assert_eq!(result.values, vec![2., 4., 6.]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let result = s - 3.;
        assert_eq!(result.values, vec![-2., -1., 0.]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let result = s / 2.;
        assert_eq!(result.values, vec![0.5, 1., 1.5]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let result = s % 2.;
        assert_eq!(result.values, vec![1., 0., 1.]);
        assert_eq!(result.index.values, vec![10, 20, 30]);
    }

    #[test]
    fn test_series_ops_f64_broadcast_refs() {
        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);

        let result = &s + 3.;
        assert_eq!(result.values, vec![4., 5., 6.]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let result = &s + &3.;
        assert_eq!(result.values, vec![4., 5., 6.]);
        assert_eq!(result.index.values, vec![10, 20, 30]);
    }

    #[test]
    fn test_series_ops_f64_broadcast_move() {
        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);

        let result = s + &3.;
        assert_eq!(result.values, vec![4., 5., 6.]);
        assert_eq!(result.index.values, vec![10, 20, 30]);
    }

    #[test]
    fn test_series_ops_i64_elemwise() {
        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);
        // s moves by ops
        let result = s + r;
        assert_eq!(result.values, vec![2, 5, 5]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);
        let result = s * r;
        assert_eq!(result.values, vec![1, 6, 6]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);
        let result = s - r;
        assert_eq!(result.values, vec![0, -1, 1]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);
        let result = s / r;
        assert_eq!(result.values, vec![1, 0, 1]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);
        let result = s % r;
        assert_eq!(result.values, vec![0, 2, 1]);
        assert_eq!(result.index.values, vec![10, 20, 30]);
    }

    #[test]
    fn test_series_ops_i64_elemwise_refs() {
        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);

        let result = &s + r;
        assert_eq!(result.values, vec![2, 5, 5]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);
        let result = &s + &r;
        assert_eq!(result.values, vec![2, 5, 5]);
        assert_eq!(result.index.values, vec![10, 20, 30]);
    }

    #[test]
    fn test_series_ops_i64_elemwise_move() {
        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);

        let result = s + &r;
        assert_eq!(result.values, vec![2, 5, 5]);
        assert_eq!(result.index.values, vec![10, 20, 30]);
    }

    #[test]
    fn test_series_ops_f64_elemwise() {
        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);
        // s moves by ops
        let result = s + r;
        assert_eq!(result.values, vec![2., 5., 5.]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);
        let result = s * r;
        assert_eq!(result.values, vec![1., 6., 6.]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);
        let result = s - r;
        assert_eq!(result.values, vec![0., -1., 1.]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);
        let result = s / r;
        assert_eq!(result.values, vec![1., 0.6666666666666666, 1.5]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);
        let result = s % r;
        assert_eq!(result.values, vec![0., 2., 1.]);
        assert_eq!(result.index.values, vec![10, 20, 30]);
    }

    #[test]
    fn test_series_ops_f64_elemwise_refs() {
        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);

        let result = &s + r;
        assert_eq!(result.values, vec![2., 5., 5.]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);
        let result = &s + &r;
        assert_eq!(result.values, vec![2., 5., 5.]);
        assert_eq!(result.index.values, vec![10, 20, 30]);
    }

    #[test]
    fn test_series_ops_f64_elemwise_move() {
        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);

        let result = s + &r;
        assert_eq!(result.values, vec![2., 5., 5.]);
        assert_eq!(result.index.values, vec![10, 20, 30]);
    }
}

