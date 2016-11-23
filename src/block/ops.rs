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

macro_rules! define_numeric_op(
  ($t:ident $m:ident) => (

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

  );
);

define_numeric_op!(Add add);
define_numeric_op!(Mul mul);
define_numeric_op!(Sub sub);
define_numeric_op!(Div div);
define_numeric_op!(Rem rem);


#[cfg(test)]
mod tests {

    use super::super::Block;

    #[test]
    fn test_block_ops_i64_broadcast() {
        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        // b moves by ops
        let mut result = b + 3;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![4, 5, 6]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![7, 8, 9]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = b * 2;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![2, 4, 6]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![8, 10, 12]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = b - 3;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![-2, -1, 0]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![1, 2, 3]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = b / 2;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![0, 1, 1]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![2, 2, 3]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = b % 2;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![1, 0, 1]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![0, 1, 0]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);
    }

    #[test]
    fn test_block_ops_i64_broadcast_refs() {
        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = &b + 3;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![4, 5, 6]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![7, 8, 9]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let mut result = &b + &3;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![4, 5, 6]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![7, 8, 9]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let mut result = b + &3;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![4, 5, 6]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![7, 8, 9]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);
    }

    #[test]
    fn test_block_ops_f64_broadcast() {
        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        // b moves by ops
        let mut result = b + 3.;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![4., 5., 6.]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![7., 8., 9.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = b * 2.;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![2., 4., 6.]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![8., 10., 12.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = b - 3.;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![-2., -1., 0.]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![1., 2., 3.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = b / 2.;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![0.5, 1., 1.5]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![2., 2.5, 3.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = b % 2.;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![1., 0., 1.]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![0., 1., 0.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);
    }

    #[test]
    fn test_block_ops_f64_broadcast_refs() {
        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = &b + 3.;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![4., 5., 6.]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![7., 8., 9.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let mut result = &b + &3.;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![4., 5., 6.]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![7., 8., 9.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let mut result = b + &3.;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![4., 5., 6.]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![7., 8., 9.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);
    }

    #[test]
    fn test_block_ops_i64_elemwise() {
        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        // b moves by ops
        let mut result = b + r;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![3, 5, 4]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![6, 8, 7]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = b * r;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![2, 6, 3]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![8, 15, 6]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = b - r;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![-1, -1, 2]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![2, 2, 5]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = b / r;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![0, 0, 3]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![2, 1, 6]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = b % r;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![1, 2, 0]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![0, 2, 0]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);
    }

    #[test]
    fn test_block_ops_i64_elemwise_refs() {
        let b = Block::from_col_vec(vec![1, 2, 3, 4, 5, 6],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = &b + r;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![3, 5, 4]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![6, 8, 7]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let r = Block::from_col_vec(vec![2, 3, 1, 2, 3, 1],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = &b + &r;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![3, 5, 4]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![6, 8, 7]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let mut result = b + &r;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![3, 5, 4]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![6, 8, 7]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);
    }

    #[test]
    fn test_block_ops_f64_elemwise() {
        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        // b moves by ops
        let mut result = b + r;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![3., 5., 4.]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![6., 8., 7.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = b * r;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![2., 6., 3.]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![8., 15., 6.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = b - r;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![-1., -1., 2.]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![2., 2., 5.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = b / r;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![0.5, 0.6666666666666666, 3.]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![2., 1.6666666666666667, 6.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = b % r;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![1., 2., 0.]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![0., 2., 0.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);
    }

    #[test]
    fn test_block_ops_f64_elemwise_refs() {
        let b = Block::from_col_vec(vec![1., 2., 3., 4., 5., 6.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);

        let mut result = &b + r;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![3., 5., 4.]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![6., 8., 7.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let r = Block::from_col_vec(vec![2., 3., 1., 2., 3., 1.],
                                    vec![10, 20, 30], vec!["X", "Y"]);
        let mut result = &b + &r;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![3., 5., 4.]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![6., 8., 7.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);

        let mut result = b + &r;
        assert_eq!(&result.get_column_by_label(&"X").values, &vec![3., 5., 4.]);
        assert_eq!(&result.get_column_by_label(&"Y").values, &vec![6., 8., 7.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
        assert_eq!(&result.columns.values, &vec!["X", "Y"]);
    }
}
