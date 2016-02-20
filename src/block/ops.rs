extern crate num;

use num::{Num};
use std::hash::Hash;
use std::ops::{Add, Mul, Sub, Div, Rem};

use super::Block;

macro_rules! define_numric_op(
  ($t:ident $m:ident) => (

    // Broadcast
    impl<T, U, V> $t<T> for Block<T, U, V>
        where T: Copy + Num,
              U: Copy + Eq + Hash,
              V: Copy + Eq + Hash {

        type Output = Block<T, U, V>;
        fn $m(self, _rhs: T) -> Block<T, U, V> {
            let mut new_values: Vec<Vec<T>> = vec![];
            for value in self.values {
                let new_value = value.iter().map(|x: &T| (*x).$m(_rhs)).collect();
                new_values.push(new_value);
            }
            return Block::from_nested_vec(new_values,
                                          self.index.copy_values(),
                                          self.columns.copy_values());
        }
    }

    // Element-wise
    impl<T, U, V> $t<Block<T, U, V>> for Block<T, U, V>
        where T: Copy + Num,
              U: Copy + Eq + Hash,
            V: Copy + Eq + Hash {

        type Output = Block<T, U, V>;
        fn $m(self, _rhs: Block<T, U, V>) -> Block<T, U, V> {
            if !self.index.equals(&_rhs.index) {
                panic!("index must be the same!");
            }
            if !self.columns.equals(&_rhs.columns) {
                panic!("columns must be the same!");
            }
            let mut new_values: Vec<Vec<T>> = vec![];
            for (value, rvalue) in self.values.iter().zip(_rhs.values.iter()) {
                let new_value = value.iter()
                                     .zip(rvalue.iter())
                                     .map(|(x, y)| (*x).$m(*y)).collect();
                new_values.push(new_value);
            }
            return Block::from_nested_vec(new_values,
                                          self.index.copy_values(),
                                          self.columns.copy_values());
        }
    }

  );
);

define_numric_op!(Add add);
define_numric_op!(Mul mul);
define_numric_op!(Sub sub);
define_numric_op!(Div div);
define_numric_op!(Rem rem);

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
}