use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;


pub struct HashGroupBy<T> {
    pub groups: HashMap<T, Vec<usize>>
}

pub trait GroupBy<T> {
    // ToDo: Implement efficient multimap
    fn groupby(key: &Vec<T>) -> HashGroupBy<T>;
    fn get(&self, key: &T) -> Option<&Vec<usize>>;
    fn keys(&self) -> Vec<T>;
}

impl<T> GroupBy<T> for HashGroupBy<T>
    where T: Hash + Eq + Copy {

    fn groupby(key: &Vec<T>) -> HashGroupBy<T> {

        let mut map: HashMap<T, Vec<usize>> = HashMap::new();

        for (i, k) in key.iter().enumerate() {
            match map.entry(*k) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(i);
                },
                Entry::Vacant(e) => {
                    e.insert(vec![i]);
                }
            }
        }
        HashGroupBy { groups: map }
    }

    fn get(&self, key: &T) -> Option<&Vec<usize>> {
        self.groups.get(key)
    }

    fn keys(&self) -> Vec<T> {
        let keys: Vec<T> = self.groups.keys().cloned().collect();
        keys
    }
}


#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use super::{GroupBy, HashGroupBy};

    #[test]
    fn test_vec_groupby_int() {
        let key = vec![1, 1, 2, 2];
        let res = HashGroupBy::groupby(&key);

        let mut exp: HashMap<i32, Vec<usize>> = HashMap::new();
        exp.insert(1, vec![0, 1]);
        exp.insert(2, vec![2, 3]);
        assert_eq!(res.groups, exp);

        assert_eq!(res.get(&1), Some(&vec![0, 1]));
        assert_eq!(res.get(&2), Some(&vec![2, 3]));
    }

    #[test]
    fn test_vec_groupby_str() {
        let key = vec!["a", "b", "a", "b"];
        let res = HashGroupBy::groupby(&key);

        let mut exp: HashMap<&str, Vec<usize>> = HashMap::new();
        exp.insert("a", vec![0, 2]);
        exp.insert("b", vec![1, 3]);
        assert_eq!(res.groups, exp);

        assert_eq!(res.get(&"a"), Some(&vec![0, 2]));
        assert_eq!(res.get(&"b"), Some(&vec![1, 3]));
    }

}
