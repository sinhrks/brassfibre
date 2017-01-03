use std::borrow::Cow;
use std::hash::Hash;

use super::Indexer;

/// /////////////////////////////////////////////////////////////////////////////
/// From / Into
/// /////////////////////////////////////////////////////////////////////////////

impl<T: Clone + Eq + Hash> From<Vec<T>> for Indexer<T> {
    fn from(values: Vec<T>) -> Self {
        Indexer::new(values)
    }
}

impl<T: Clone + Hash> Into<Vec<T>> for Indexer<T> {
    fn into(self) -> Vec<T> {
        self.values
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Clone on Write
/// /////////////////////////////////////////////////////////////////////////////

impl<'a, T: Clone + Hash> Into<Cow<'a, Indexer<T>>> for Indexer<T> {
    fn into(self) -> Cow<'a, Self> {
        Cow::Owned(self)
    }
}

#[cfg(test)]
mod tests {

    use super::super::Indexer;

    #[test]
    fn test_i64_vec_to_indexer() {
        let exp: Indexer<i64> = Indexer::new(vec![1, 2, 3]);

        let idx: Indexer<i64> = vec![1, 2, 3].into();
        assert_eq!(idx, exp);

        let idx: Indexer<i64> = Indexer::from(vec![1, 2, 3]);
        assert_eq!(idx, exp);
    }

    #[test]
    fn test_i64_indexer_to_vec() {
        let exp: Vec<i64> = vec![1, 2, 3];

        let idx: Indexer<i64> = Indexer::new(vec![1, 2, 3]);
        let conv: Vec<i64> = idx.into();
        assert_eq!(conv, exp);

        // let idx: Indexer<i64> = Indexer::new(vec![1, 2, 3]);
        // let conv: Vec<i64> = Vec::from(idx.into());
        // assert_eq!(conv, exp);
        //
    }

    #[test]
    fn test_str_vec_to_indexer() {
        let exp: Indexer<&str> = Indexer::new(vec!["a", "b", "c"]);

        let idx: Indexer<&str> = vec!["a", "b", "c"].into();
        assert_eq!(idx, exp);

        let idx: Indexer<&str> = Indexer::from(vec!["a", "b", "c"]);
        assert_eq!(idx, exp);
    }

    #[test]
    fn test_str_indexer_to_vec() {
        let exp: Vec<&str> = vec!["a", "b", "c"];

        let idx: Indexer<&str> = Indexer::new(vec!["a", "b", "c"]);
        let conv: Vec<&str> = idx.into();
        assert_eq!(conv, exp);

        // let idx: Indexer<&str> = Indexer::new(vec!["a", "b", "c"]);
        // let conv: Vec<&str> = Vec::from(idx.into());
        // assert_eq!(conv, exp);
        //
    }

    #[test]
    fn test_string_vec_to_indexer() {
        let exp: Indexer<String> =
            Indexer::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);

        let idx: Indexer<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()].into();
        assert_eq!(idx, exp);

        let idx: Indexer<String> =
            Indexer::from(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        assert_eq!(idx, exp);
    }

    #[test]
    fn test_string_indexer_to_vec() {
        let exp: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];

        let idx: Indexer<String> =
            Indexer::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        let conv: Vec<String> = idx.into();
        assert_eq!(conv, exp);

    }
}
