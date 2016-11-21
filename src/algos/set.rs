use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::FromIterator;


fn to_hashset<T>(a: &Vec<T>) -> HashSet<T> where T: Hash + Eq + Clone {
    HashSet::from_iter(a.iter().cloned())
}

/// Create HashMap<T, usize> from Vec<T> which value is appearance
/// location
pub fn to_enumhashmap<T>(v: &Vec<T>) -> HashMap<T, usize> where T: Hash + Eq + Copy {
    // ToDo: Handle duplicates

    let mut map: HashMap<T, usize> = HashMap::with_capacity(v.len());
    for (i, key) in v.iter().enumerate() {
        // ToDo: Handle duplicates
        if map.contains_key(key) {
            panic!("duplicates are not allowed");
        } else {
            map.insert(*key, i);
        }
    }
    map
}


pub fn union<T>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> where T: Hash + Eq + Copy {
    // Use HashMap to keep the order

    // copy
    let mut res: Vec<T> = a.clone();
    let set = to_hashset(a);

    for key in b.iter() {
        if !set.contains(key) {
            res.push(*key);
        }
    }
    res
}

#[cfg(test)]
mod tests {

    use super::union;

    #[test]
    fn test_union() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![2, 3, 4];

        let res = union(&v1, &v2);

        assert_eq!(res, vec![1, 2, 3, 4]);
    }
}