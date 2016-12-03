use num::{Num};
use std::hash::Hash;
use std::ops::{Add, Mul, Sub, Div, Rem};

use super::Series;

fn elemwise<V>(left: &Vec<V>, right: &Vec<V>,
               func: &Fn((&V, &V)) -> V) -> Vec<V>
    where V: Copy + Num {
    left.iter()
        .zip(right.iter())
        .map(func).collect()
}

macro_rules! define_numric_op {
    ($t:ident, $m:ident) => {

        // Broadcast
        impl<'i, V, I> $t<V> for Series<'i, V, I>
            where V: Copy + Num,
                  I: 'i + Copy + Eq + Hash {

            type Output = Self;
            fn $m(self, _rhs: V) -> Self {
                let new_values: Vec<V> = self.values.iter().map(|x: &V| (*x).$m(_rhs)).collect();
                // self is moved, pass index to new instance
                Series::from_cow(new_values, self.index)
            }
        }

        impl<'i, 'r, V, I> $t<&'r V> for Series<'i, V, I>
            where V: Copy + Num,
                  I: Copy + Eq + Hash {

            type Output = Self;
            fn $m(self, _rhs: &'r V) -> Self {
                let new_values = self.values.iter().map(|x: &V| (*x).$m(*_rhs)).collect();
                Series::from_cow(new_values, self.index)
            }
        }

        impl<'i, 'l, V, I> $t<V> for &'l Series<'i, V, I>
            where V: Copy + Num,
                  I: Copy + Eq + Hash {

            type Output = Series<'i, V, I>;
            fn $m(self, _rhs: V) -> Series<'i, V, I> {
                let new_values = self.values.iter().map(|x: &V| (*x).$m(_rhs)).collect();
                // error[E0495]: cannot infer an appropriate lifetime for autoref due to conflicting requirements
                Series::from_cow(new_values, self.index.to_owned())
            }
        }

        impl<'i, 'l, 'r, V, I> $t<&'r V> for &'l Series<'i, V, I>
            where V: Copy + Num,
                  I: Copy + Eq + Hash {

            type Output = Series<'i, V, I>;
            fn $m(self, _rhs: &'r V) -> Series<'i, V, I> {
                let new_values = self.values.iter().map(|x: &V| (*x).$m(*_rhs)).collect();
                // error[E0495]: cannot infer an appropriate lifetime for autoref due to conflicting requirements
                Series::from_cow(new_values, self.index.to_owned())
            }
        }

        // Element-wise
        impl<'li, 'ri, V, I> $t<Series<'ri, V, I>> for Series<'li, V, I>
            where V: Copy + Num,
                  I: Copy + Eq + Hash {

            type Output = Self;
            fn $m(self, _rhs: Series<'ri, V, I>) -> Self {
                self.assert_binop(&_rhs);
                let new_values = elemwise(&self.values, &_rhs.values,
                                          &|(x, y)| (*x).$m(*y));
                Series::from_cow(new_values, self.index)
            }
        }

        impl<'li, 'ri, 'r, V, I> $t<&'r Series<'ri, V, I>> for Series<'li, V, I>
            where V: Copy + Num,
                  I: Copy + Eq + Hash {

            type Output = Self;
            fn $m(self, _rhs: &'r Series<'ri, V, I>) -> Self {
                self.assert_binop(&_rhs);
                let new_values = elemwise(&self.values, &_rhs.values,
                                          &|(x, y)| (*x).$m(*y));
                Series::from_cow(new_values, self.index)
            }
        }

        impl<'li, 'ri, 'l, V, I> $t<Series<'ri, V, I>> for &'l Series<'li, V, I>
            where V: Copy + Num,
                  I: Copy + Eq + Hash {

            type Output = Series<'li, V, I>;
            fn $m(self, _rhs: Series<'ri, V, I>) -> Series<'li, V, I> {
                self.assert_binop(&_rhs);
                let new_values = elemwise(&self.values, &_rhs.values,
                                          &|(x, y)| (*x).$m(*y));
                // error[E0495]: cannot infer an appropriate lifetime for autoref due to conflicting requirements
                // Series::from_cow(new_values, Cow::Borrowed(self.index.borrow()))
                Series::from_cow(new_values, self.index.to_owned())
            }
        }

        impl<'li, 'ri, 'l, 'r, V, I> $t<&'r Series<'ri, V, I>> for &'l Series<'li, V, I>
            where V: Copy + Num,
                  I: Copy + Eq + Hash {

            type Output = Series<'li, V, I>;
            fn $m(self, _rhs: &'r Series<'ri, V, I>) -> Series<'li, V, I> {
                self.assert_binop(&_rhs);
                let new_values = elemwise(&self.values, &_rhs.values,
                                          &|(x, y)| (*x).$m(*y));

                // error[E0495]: cannot infer an appropriate lifetime for autoref due to conflicting requirements
                // Series::from_cow(new_values, Cow::Borrowed(self.index.borrow()))
                Series::from_cow(new_values, self.index.to_owned())
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

        let result = &s + 3;
        assert_eq!(result.values, vec![4, 5, 6]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

        let result = &s + &3;
        assert_eq!(result.values, vec![4, 5, 6]);
        assert_eq!(result.index.values, vec![10, 20, 30]);

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

        let result = s + &r;
        assert_eq!(result.values, vec![2., 5., 5.]);
        assert_eq!(result.index.values, vec![10, 20, 30]);
    }
}

