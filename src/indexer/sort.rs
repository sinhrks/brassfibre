use std::hash::Hash;

use super::Indexer;
use super::super::algos::sort::Sorter;

//**********************************************
// Soat
//**********************************************

impl<U> Indexer<U> where U: Copy + Eq + Hash + Ord {

    pub fn argsort(&self) -> (Vec<usize>, Self) {
        let (indexer, sorted) = Sorter::argsort(&self.values);
        let sorted = Indexer::new(sorted);
        (indexer, sorted)
    }

    pub fn sort(&self) -> Self {
        Indexer::new(Sorter::sort(&self.values))
    }
}


#[cfg(test)]
mod tests {

    use super::super::Indexer;

    #[test]
    fn test_index_argsort_int() {
        let idx = Indexer::new(vec![5, 4, 3, 2, 1]);
        let (indexer, sorted) = idx.argsort();
        assert_eq!(indexer, vec![4, 3, 2, 1, 0]);
        assert_eq!(sorted, Indexer::new(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_index_sort_int() {
        let idx = Indexer::new(vec![5, 4, 3, 2, 1]);
        let sorted = idx.sort();
        assert_eq!(sorted, Indexer::new(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_index_argsort_str() {
        let idx = Indexer::new(vec!["d", "b", "a", "c"]);
        let (indexer, sorted) = idx.argsort();
        assert_eq!(indexer, vec![2, 1, 3, 0]);
        assert_eq!(sorted, Indexer::new(vec!["a", "b", "c", "d"]));
    }

    #[test]
    fn test_index_sort_str() {
        let idx = Indexer::new(vec!["d", "b", "a", "c"]);
        let sorted = idx.sort();
        assert_eq!(sorted, Indexer::new(vec!["a", "b", "c", "d"]));
    }
}
