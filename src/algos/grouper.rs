use std::collections::HashMap;
use std::hash::Hash;

// ToDo: use Cow?

pub struct HashGrouper<T> {
    pub groups: HashMap<T, Vec<usize>>
}

pub trait Grouper<T> {
    // ToDo: Implement efficient multimap
    fn groupby(key: &[T]) -> HashGrouper<T>;
    fn get(&self, key: &T) -> Option<&Vec<usize>>;
    fn keys(&self) -> Vec<T>;
    fn len(&self) -> usize;
}

impl<T> Grouper<T> for HashGrouper<T>
    where T: Clone + Hash + Eq {

    fn groupby(key: &[T]) -> HashGrouper<T> {

        let mut map: HashMap<T, Vec<usize>> = HashMap::new();

        for (i, k) in key.iter().enumerate() {
            let e = map.entry(k.clone()).or_insert(Vec::<usize>::new());
            e.push(i);
        }
        HashGrouper { groups: map }
    }

    fn get(&self, key: &T) -> Option<&Vec<usize>> {
        self.groups.get(key)
    }

    fn keys(&self) -> Vec<T> {
        let keys: Vec<T> = self.groups.keys().cloned().collect();
        keys
    }

    fn len(&self) -> usize {
        self.groups.len()
    }
}


#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use super::{Grouper, HashGrouper};

    #[test]
    fn test_vec_groupby_int() {
        let key = vec![1, 1, 2, 2];
        let res = HashGrouper::groupby(&key);

        let mut exp: HashMap<i32, Vec<usize>> = HashMap::new();
        exp.insert(1, vec![0, 1]);
        exp.insert(2, vec![2, 3]);
        assert_eq!(res.groups, exp);
        assert_eq!(res.len(), 2);

        assert_eq!(res.get(&1), Some(&vec![0, 1]));
        assert_eq!(res.get(&2), Some(&vec![2, 3]));
    }

    #[test]
    fn test_vec_groupby_str() {
        let key = vec!["a", "b", "a", "b"];
        let res = HashGrouper::groupby(&key);

        let mut exp: HashMap<&str, Vec<usize>> = HashMap::new();
        exp.insert("a", vec![0, 2]);
        exp.insert("b", vec![1, 3]);
        assert_eq!(res.groups, exp);
        assert_eq!(res.len(), 2);

        assert_eq!(res.get(&"a"), Some(&vec![0, 2]));
        assert_eq!(res.get(&"b"), Some(&vec![1, 3]));
    }

}
