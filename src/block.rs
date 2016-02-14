extern crate num;

use num::{Num, Zero, Float, ToPrimitive};
use std::fmt;
use std::hash::Hash;

use super::blockgroupby::BlockGroupBy;
use super::computations;
use super::formatting;
use super::index::Indexer;
use super::series::Series;

pub struct Block<T, U: Hash, V: Hash> {
    /// 2-dimentional block contains a single type.
    /// T: type of values
    /// U: type of indexer
    /// V: type of columns

    // ToDo: may be simpler to use 1-d Vec?
    pub values: Vec<Vec<T>>,
    pub index: Indexer<U>,
    pub columns: Indexer<V>,
}

// Indexing

impl<T: Copy, U: Copy + Eq + Hash, V: Copy + Eq + Hash> Block<T, U, V> {

    pub fn from_col_vec(values: Vec<T>, index: Vec<U>, columns: Vec<V>) -> Block<T, U, V> {
        let len: usize = index.len();
        let cols: usize = columns.len();

        if values.len() != len * cols {
            panic!("Length mismatch!");
        }

        let mut new_values: Vec<Vec<T>> = vec![];
        for value in values.chunks(len) {
            let v: Vec<T> = value.iter().map(|x| *x).collect();
            new_values.push(v);
        }
        Block {
            values: new_values,
            index: Indexer::new(index),
            columns: Indexer::new(columns),
        }
    }

    pub fn from_series(series: Series<T, U>, name: V) -> Block<T, U, V> {
        let mut values: Vec<Vec<T>> = vec![];
        values.push(series.values);

        // mapper is not updated properly by vec![name]
        let mut columns = Indexer::new(vec![]);
        columns.push(name);

        Block {
            values: values,
            index: series.index,
            columns: columns,
        }
    }

    fn from_internal(values: Vec<Vec<T>>, index: Indexer<U>,
                     columns: Indexer<V>) -> Block<T, U, V> {
        /*
        Instanciate from instanciated MultiMap and Indexer. Used internally
        */
        Block {
            values: values,
            index: index,
            columns: columns,
        }
    }

    pub fn add_columns(&mut self, values: Vec<T>, name: V) {
        if self.len() != values.len() {
            panic!("Length mismatch!");
        }
        self.values.push(values);
        self.columns.push(name);
    }

    pub fn len(&self) -> usize {
        return self.index.len();
    }

    pub fn copy(&self) -> Block<T, U, V> {
        let mut new_values: Vec<Vec<T>> = vec![];
        for value in self.values.iter() {
            let new_value = value.iter().map(|x| *x).collect();
            new_values.push(new_value);
        }
        return Block::from_internal(new_values,
                                    self.index.copy(),
                                    self.columns.copy());
    }

    pub fn get_column_by_label(&mut self, label: &V) -> Series<T, U> {
        let loc = self.columns.get_label_loc(label);
        let mut new_values: Vec<T> = vec![];
        for v in self.values[loc].iter() {
            new_values.push(*v);
        }
        return Series::new(new_values, self.index.copy_values());
    }

    pub fn slice_by_label(&mut self, labels: &Vec<U>) -> Block<T, U, V> {
        let locs = self.index.slice_label_loc(labels);
        return self.slice_by_index(&locs);
    }

    pub fn slice_by_index(&self, locations: &Vec<usize>) -> Block<T, U, V> {

        let new_index: Vec<U> = locations.iter()
                                         .map(|loc| self.index.values[*loc])
                                         .collect();

        let mut new_values: Vec<Vec<T>> = vec![];
        for current in self.values.iter() {
            let new_value = locations.iter().map(|x| current[*x]).collect();
            new_values.push(new_value);
        }
        return Block::<T, U, V>::from_internal(new_values,
                                               Indexer::new(new_index),
                                               self.columns.copy());
    }

    pub fn groupby<G: Copy + Eq + Hash + Ord>(&self, other: Vec<G>) -> BlockGroupBy<T, U, V, G> {
        return BlockGroupBy::new(self.copy(), other);
    }
}


// Formatting

impl<T, U, V: fmt::Debug> fmt::Display for Block<T, U, V> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Block(columns={:?})", &self.columns.values);
    }

}

impl<T: Copy + ToString,
     U: Copy + Eq + Hash + ToString,
     V: Copy + Eq + Hash + ToString> fmt::Debug for Block<T, U, V> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut str_values: Vec<Vec<String>> = vec![];

        for (i, column) in self.columns.values.iter().enumerate() {
            let current = self.values[i].iter().map(|x| *x).collect();
            let column_str = formatting::pad_string_vector_with_header(&current, column.to_string());
            str_values.push(column_str);
        }
        let str_index = formatting::pad_string_vector_with_header(&self.index.values,
                                                                  "".to_string());

        let mut result = "".to_string();
        for (i, label) in str_index.iter().enumerate() {
            result.push_str(label);
            result.push(' ');
            for column in str_values.iter() {
                result.push_str(&column[i]);
                result.push(' ');
            }
            // result.push_str(v);
            result.push_str("\n");
        }
        // debug expression {:?} outputs linesep as character, do not use
        return write!(f, "{:}", &result);
    }

}


// Aggregation

impl<T: Copy + Num + Zero + ToPrimitive,
     U: Copy + Eq + Hash,
     V: Copy + Eq + Hash>

     Block<T, U, V> {

    pub fn apply(&self, func: &Fn(&Vec<T>) -> T) -> Series<T, V> {
        /*
        Apply passed function to each columns.
        */
        let mut new_values = vec![];
        for current in self.values.iter() {
            new_values.push(func(&current));
        }
        return Series::new(new_values, self.columns.copy_values());
    }

    fn apply_f64(&self, func: &Fn(&Vec<T>) -> f64) -> Series<f64, V> {
        /*
        Apply passed function to each columns.
        */

        // ToDo: merge the logic to apply
        let mut new_values = vec![];
        for current in self.values.iter() {
            new_values.push(func(&current));
        }
        return Series::new(new_values, self.columns.copy_values());
    }

    pub fn sum(&self) -> Series<T, V> {
        return self.apply(&|x| computations::vec_sum(x));
    }

    pub fn count(&self) -> Series<usize, V> {
        // ToDo: merge the logic to apply
        let mut new_values = vec![];
        for current in self.values.iter() {
            new_values.push(computations::vec_count(&current));
        }
        return Series::new(new_values, self.columns.copy_values());
    }

    pub fn mean(&self) -> Series<f64, V> {
        return self.apply_f64(&|x| computations::vec_mean(x));
    }

    pub fn var(&self) -> Series<f64, V> {
        return self.apply_f64(&|x| computations::vec_var(x));
    }

    pub fn unbiased_var(&self) -> Series<f64, V> {
        return self.apply_f64(&|x| computations::vec_unbiased_var(x));
    }

    pub fn std(&self) -> Series<f64, V> {
        return self.apply_f64(&|x| computations::vec_std(x));
    }

    pub fn unbiased_std(&self) -> Series<f64, V> {
        return self.apply_f64(&|x| computations::vec_unbiased_std(x));
    }
}


#[cfg(test)]
mod tests {

    use super::Block;
    use super::super::series::Series;


    #[test]
    fn test_block_creation_from_series() {
        let values = vec![1, 2, 3, 4, 5,
                          6, 7, 8, 9, 10,
                          11, 12, 13, 14, 15];
        let mut b = Block::from_col_vec(values,
                                        vec!["A", "BB", "CC", "D", "EEE"],
                                        vec!["X", "YYY", "ZZ"]);
        assert_eq!(&b.len(), &5);

        let exp_index: Vec<&str> = vec!["A", "BB", "CC", "D", "EEE"];
        let exp_columns: Vec<i64> = vec!["X", "YYY", "ZZ"];
        assert_eq!(&b.index.values, &exp_index);
        assert_eq!(&b.columns.values, &exp_columns);

        let c = b.get_column_by_label(&"X");
        let exp_values: Vec<i64> = vec![1, 2, 3, 4, 5];
        assert_eq!(&c.values, &exp_values);
        assert_eq!(&c.index.values, &exp_index);

        let c = b.get_column_by_label(&"YYY");
        let exp_values: Vec<i64> = vec![6, 7, 8, 9, 10];
        assert_eq!(&c.values, &exp_values);
        assert_eq!(&c.index.values, &exp_index);

        let c = b.get_column_by_label(&"ZZ");
        let exp_values: Vec<i64> = vec![11, 12, 13, 14, 15];
        assert_eq!(&c.values, &exp_values);
        assert_eq!(&c.index.values, &exp_index);
    }

    #[test]
    fn test_block_creation_from_series() {
        let values: Vec<f64> = vec![1., 2., 3.];
        let index: Vec<&str> = vec!["A", "B", "C"];
        let s = Series::<f64, &str>::new(values, index);

        let mut b = Block::<f64, &str, i64>::from_series(s, 1);
        assert_eq!(&b.len(), &3);

        let exp_index: Vec<&str> = vec!["A", "B", "C"];
        let exp_columns: Vec<i64> = vec![1];
        assert_eq!(&b.index.values, &exp_index);
        assert_eq!(&b.columns.values, &exp_columns);

        let c = b.get_column_by_label(&1);
        let exp_values: Vec<f64> = vec![1., 2., 3.];
        assert_eq!(&c.values, &exp_values);
        assert_eq!(&c.index.values, &exp_index);
    }

    #[test]
    fn test_add_columns() {
        let values: Vec<f64> = vec![1., 2., 3.];
        let index: Vec<&str> = vec!["A", "B", "C"];
        let s = Series::<f64, &str>::new(values, index);

        let mut b = Block::<f64, &str, i64>::from_series(s, 1);

        assert_eq!(&b.len(), &3);
        let exp_index: Vec<&str> = vec!["A", "B", "C"];
        let exp_columns: Vec<i64> = vec![1];
        assert_eq!(&b.index.values, &exp_index);
        assert_eq!(&b.columns.values, &exp_columns);

        // add columns
        let values2: Vec<f64> = vec![4., 5., 6.];
        b.add_columns(values2, 3);
        assert_eq!(&b.len(), &3);
        let exp_columns: Vec<i64> = vec![1, 3];
        assert_eq!(&b.index.values, &exp_index);
        assert_eq!(&b.columns.values, &exp_columns);

        assert_eq!(&b.columns.get_label_loc(&1), &0);
        assert_eq!(&b.columns.get_label_loc(&3), &1);
        let c = b.get_column_by_label(&1);
        let exp_values: Vec<f64> = vec![1., 2., 3.];
        assert_eq!(&c.values, &exp_values);
        assert_eq!(&c.index.values, &exp_index);

        let c = b.get_column_by_label(&3);
        let exp_values: Vec<f64> = vec![4., 5., 6.];
        assert_eq!(&c.values, &exp_values);
        assert_eq!(&c.index.values, &exp_index);
    }

    #[test]
    fn test_slice_by_index() {
        let values: Vec<f64> = vec![1., 2., 3.];
        let index: Vec<&str> = vec!["A", "B", "C"];
        let s = Series::<f64, &str>::new(values, index);
        let mut b = Block::<f64, &str, i64>::from_series(s, 1);
        // add columns
        let values2: Vec<f64> = vec![4., 5., 6.];
        b.add_columns(values2, 3);
        assert_eq!(&b.len(), &3);

        // slice
        let mut sliced = b.slice_by_index(&vec![0, 2]);
        let exp_index: Vec<&str> = vec!["A", "C"];
        let exp_columns: Vec<i64> = vec![1, 3];
        assert_eq!(&sliced.index.values, &exp_index);
        assert_eq!(&sliced.columns.values, &exp_columns);

        // compare columns
        let c = sliced.get_column_by_label(&1);
        let exp_values: Vec<f64> = vec![1., 3.];
        assert_eq!(&c.values, &exp_values);
        let c = sliced.get_column_by_label(&3);
        let exp_values: Vec<f64> = vec![4., 6.];
        assert_eq!(&c.values, &exp_values);
    }

    #[test]
    fn test_slice_by_label() {
        let values: Vec<f64> = vec![1., 2., 3.];
        let index: Vec<&str> = vec!["A", "B", "C"];
        let s = Series::<f64, &str>::new(values, index);
        let mut b = Block::<f64, &str, i64>::from_series(s, 1);
        // add columns
        let values2: Vec<f64> = vec![4., 5., 6.];
        b.add_columns(values2, 3);
        assert_eq!(&b.len(), &3);

        // slice
        let mut sliced = b.slice_by_label(&vec!["B", "C"]);
        let exp_index: Vec<&str> = vec!["B", "C"];
        let exp_columns: Vec<i64> = vec![1, 3];
        assert_eq!(&sliced.index.values, &exp_index);
        assert_eq!(&sliced.columns.values, &exp_columns);

        // compare columns
        let c = sliced.get_column_by_label(&1);
        let exp_values: Vec<f64> = vec![2., 3.];
        assert_eq!(&c.values, &exp_values);
        let c = sliced.get_column_by_label(&3);
        let exp_values: Vec<f64> = vec![5., 6.];
        assert_eq!(&c.values, &exp_values);
    }

    #[test]
    fn test_block_sum() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<i64, i64>::new(values, index);
        let mut b = Block::from_series(s, "X");

        let new_values: Vec<i64> = vec![6, 7, 8, 9, 10];
        b.add_columns(new_values, "Y");

        let sum = b.sum();

        let exp_values: Vec<i64> = vec![15, 40];
        let exp_index: Vec<&str> = vec!["X", "Y"];
        assert_eq!(&sum.values, &exp_values);
        assert_eq!(&sum.index.values, &exp_index);
    }

    #[test]
    fn test_block_mean() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];
        let index: Vec<i64> = vec![10, 20, 30, 40, 50];
        let s = Series::<i64, i64>::new(values, index);
        let mut b = Block::from_series(s, "X");

        let new_values: Vec<i64> = vec![6, 7, 8, 9, 10];
        b.add_columns(new_values, "Y");

        let mean = b.mean();

        let exp_values: Vec<f64> = vec![3., 8.];
        let exp_index: Vec<&str> = vec!["X", "Y"];
        assert_eq!(&mean.values, &exp_values);
        assert_eq!(&mean.index.values, &exp_index);
    }
}