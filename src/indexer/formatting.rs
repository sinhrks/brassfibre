use std::fmt;
use std::hash::Hash;

use super::Indexer;

// Formatting

impl<U> fmt::Display for Indexer<U>
    where U: Copy + Eq + Hash + fmt::Debug {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Index({:?})", &self.values);
    }

}

impl<U> fmt::Debug for Indexer<U>
    where U: Copy + Eq + Hash + fmt::Debug {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Index({:?})", &self.values);
    }
}
