use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;


pub struct HashGroupBy<T, U> {
    pub groups: HashMap<T, Vec<U>>
}

trait GroupBy<T, U> {
    // ToDo: Implement efficient multimap
    fn groupby(key: &Vec<T>, values: &Vec<U>) -> HashGroupBy<T, U>;
    fn get(&self, key: &T) -> Option<&Vec<U>>;
}

impl<T, U> GroupBy<T, U> for HashGroupBy<T, U>
    where T: Hash + Eq + Copy, U: Copy {

    fn groupby(key: &Vec<T>, values: &Vec<U>) -> HashGroupBy<T, U> {

        let mut map: HashMap<T, Vec<U>> = HashMap::new();

        for (k, v) in key.iter().zip(values) {
            match map.entry(*k) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(*v);
                },
                Entry::Vacant(e) => {
                    e.insert(vec![*v]);
                }
            }
        }
        HashGroupBy { groups: map }
    }

    fn get(&self, key: &T) -> Option<&Vec<U>> {
        self.groups.get(key)
    }
}


#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use super::{GroupBy, HashGroupBy};

    #[test]
    fn test_vec_groupby_int() {
        let key = vec![1, 1, 2, 2];
        let values = vec![1, 2, 3, 4];

        let res = HashGroupBy::groupby(&key, &values);

        let mut exp: HashMap<i32, Vec<i32>> = HashMap::new();
        exp.insert(1, vec![1, 2]);
        exp.insert(2, vec![3, 4]);
        assert_eq!(res.groups, exp);

        assert_eq!(res.get(&1), Some(&vec![1, 2]));
        assert_eq!(res.get(&2), Some(&vec![3, 4]));
    }

    #[test]
    fn test_vec_groupby_str() {
        let key = vec!["a", "b", "a", "b"];
        let values = vec![1, 2, 3, 4];

        let res = HashGroupBy::groupby(&key, &values);

        let mut exp: HashMap<&str, Vec<i32>> = HashMap::new();
        exp.insert("a", vec![1, 3]);
        exp.insert("b", vec![2, 4]);
        assert_eq!(res.groups, exp);

        assert_eq!(res.get(&"a"), Some(&vec![1, 3]));
        assert_eq!(res.get(&"b"), Some(&vec![2, 4]));
    }

}
