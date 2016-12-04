use std::hash::Hash;
use std::fmt;

use super::Series;
use super::super::formatting;

impl<'i, V, I> fmt::Display for Series<'i, V, I>
    where V: Copy + fmt::Debug,
          I: Clone + Eq + Hash {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Series({:?})", &self.values)
    }

}

impl<'i, V, I> fmt::Debug for Series<'i, V, I>
    where V: Copy + ToString,
          I: Clone + Eq + Hash + ToString {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_index = formatting::pad_string_vector(&self.index.values);
        let str_values = formatting::pad_string_vector(&self.values);

        let mut result = vec![];
        for (i, v) in str_index.into_iter().zip(str_values.into_iter()) {
            let row = vec![i.clone(), v.clone()];
            result.push(row.join(" "));
        }
        // debug expression {:?} outputs linesep as character, do not use
        write!(f, "{:}", &result.join("\n"))
    }
}
