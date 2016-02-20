use std::fmt;
use std::hash::Hash;

use super::Block;
use super::super::formatting;


impl<T, U, V> fmt::Display for Block<T, U, V>
    where T: Copy,
          U: Copy + Eq + Hash,
          V: Copy + Eq + Hash + fmt::Debug {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Block(columns={:?})", &self.columns.values);
    }

}

impl<T, U, V> fmt::Debug for Block<T, U, V>
    where T: Copy + ToString,
          U: Copy + Eq + Hash + ToString,
          V: Copy + Eq + Hash + ToString {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut str_values: Vec<Vec<String>> = vec![];

        for (i, column) in self.columns.values.iter().enumerate() {
            let current = self.values[i].clone();
            let column_str = formatting::pad_string_vector_with_header(&current, column.to_string());
            str_values.push(column_str);
        }
        let str_index = formatting::pad_string_vector_with_header(&self.index.values,
                                                                  "".to_string());

        let mut result = vec![];
        for (i, label) in str_index.iter().enumerate() {
            let mut row_vec = vec![];
            row_vec.push(label.clone());
            for column in str_values.iter() {
                row_vec.push(column[i].clone());
            }
            result.push(row_vec.join(" "));
        }
        // debug expression {:?} outputs linesep as character, do not use
        return write!(f, "{:}", &result.join("\n"));
    }
}
