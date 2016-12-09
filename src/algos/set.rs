use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::iter::FromIterator;


fn to_hashset<T>(a: &Vec<T>) -> HashSet<T>
    where T: Clone + Hash + Eq {

    HashSet::from_iter(a.iter().cloned())
}

/// Create HashMap<T, usize> from Vec<T> which value is appearance
/// location
pub fn to_enumhashmap<T>(v: &Vec<T>) -> HashMap<T, usize>
    where T: Clone + Hash + Eq {
    // ToDo: Handle duplicates

    let mut map: HashMap<T, usize> = HashMap::with_capacity(v.len());
    for (i, ref key) in v.iter().enumerate() {
        // ToDo: Handle duplicates
        match map.entry((*key).clone()) {
            Entry::Occupied(_) => panic!("duplicates are not allowed"),
            Entry::Vacant(e) => e.insert(i),
        };
    }
    map
}


pub fn union<T>(a: &Vec<T>, b: &Vec<T>) -> Vec<T>
    where T: Clone + Hash + Eq {
    // Use HashMap to keep the order

    // copy
    let mut res: Vec<T> = a.clone();
    let set = to_hashset(a);

    for ref key in b.iter() {
        if !set.contains(key) {
            // do not clone if no need to insert
            res.push((*key).clone());
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