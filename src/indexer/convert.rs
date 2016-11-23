use std::hash::Hash;

use super::Indexer;

impl<T: Copy + Hash + Eq> From<Vec<T>> for Indexer<T> {
    fn from(values: Vec<T>) -> Self {
        Indexer::new(values)
    }
}

impl<T: Hash> Into<Vec<T>> for Indexer<T> {
    fn into(self) -> Vec<T> {
        self.values
    }
}

#[cfg(test)]
mod tests {

    use super::super::Indexer;

    #[test]
    fn test_from_vec_int() {
        let idx: Indexer<i64> = Indexer::from(vec![1, 2, 3]);
        let exp: Indexer<i64> = Indexer::new(vec![1, 2, 3]);
        assert_eq!(idx, exp);
    }

    #[test]
    fn test_into_vec_int() {
        let idx = Indexer::<i64>::new(vec![1, 2, 3]);
        let conv: Vec<i64> = idx.into();
        assert_eq!(conv, vec![1, 2, 3]);
    }

    #[test]
    fn test_from_vec_str() {
        let idx: Indexer<&str> = Indexer::from(vec!["a", "b", "c"]);
        let exp: Indexer<&str> = Indexer::new(vec!["a", "b", "c"]);
        assert_eq!(idx, exp);
    }

    #[test]
    fn test_into_vec_str() {
        let idx = Indexer::<&str>::new(vec!["a", "b", "c"]);
        let conv: Vec<&str> = idx.into();
        assert_eq!(conv, vec!["a", "b", "c"]);
    }
}