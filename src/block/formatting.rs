use std::fmt;
use std::hash::Hash;

use super::Block;
use formatting;


impl<'v, 'i, 'c, V, I, C> fmt::Display for Block<'v, 'i, 'c, V, I, C>
    where V: Clone,
          I: Clone + Eq + Hash,
          C: Clone + Eq + Hash + fmt::Debug {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Block(columns={:?})", &self.columns.values)
    }

}

impl<'v, 'i, 'c, V, I, C> fmt::Debug for Block<'v, 'i, 'c, V, I, C>
    where V: Clone + ToString,
          I: Clone + Eq + Hash + ToString,
          C: Clone + Eq + Hash + ToString {

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
        write!(f, "{:}", &result.join("\n"))
    }
}
