extern crate itertools;

use itertools::Zip;
use std::hash::Hash;
use std::fmt;

use super::Series;
use super::super::formatting;

impl<T, U> fmt::Display for Series<T, U>
    where T: Copy + fmt::Debug,
          U: Copy + Eq + Hash {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Series({:?})", &self.values);
    }

}

impl<T, U> fmt::Debug for Series<T, U>
    where T: Copy + ToString,
          U: Copy + Eq + Hash + ToString {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_index = formatting::pad_string_vector(&self.index.values);
        let str_values = formatting::pad_string_vector(&self.values);

        let mut result = vec![];
        for (i, v) in Zip::new((&str_index, &str_values)) {
            let row = vec![i.clone(), v.clone()];
            result.push(row.join(" "));
        }
        // debug expression {:?} outputs linesep as character, do not use
        return write!(f, "{:}", &result.join("\n"));
    }

}
