use std::hash::Hash;
use std::ops::Index;

use super::Indexer;

//**********************************************
//*Location based indexing
//**********************************************

impl<U> Index<usize> for Indexer<U>
    where U: Clone + Eq + Hash
{
    type Output = U;

    fn index(&self, index: usize) -> &U {
        &self.values[index]
    }
}

#[cfg(test)]
mod tests {

    use super::super::Indexer;

    #[test]
    fn test_indexing_int64() {
        let values: Vec<i64> = vec![1, 2, 3];
        let idx = Indexer::<i64>::new(values);

        assert_eq!(&idx[2], &3);
        assert_eq!(&idx[0], &1);
    }

    #[test]
    #[should_panic]
    fn test_indexing_int64_invalid() {
        let values: Vec<i64> = vec![1, 2, 3];
        let idx = Indexer::<i64>::new(values);
        idx[5];
    }

    #[test]
    fn test_indexing_str() {
        let values: Vec<&str> = vec!["A", "B", "C"];
        let idx = Indexer::<&str>::new(values);

        assert_eq!(&idx[2], &"C");
        assert_eq!(&idx[0], &"A");
    }

    #[test]
    #[should_panic]
    fn test_indexing_str_invalid() {
        let values: Vec<&str> = vec!["A", "B", "C"];
        let idx = Indexer::<&str>::new(values);
        idx[5];
    }
}
