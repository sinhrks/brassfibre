use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::iter::FromIterator;

pub struct CowCollections;

impl CowCollections {
    /// Create hashset to detect colision, without clone
    fn to_hashset<T>(a: &[T]) -> HashSet<Cow<T>>
        where T: Clone + Hash + Eq
    {

        HashSet::from_iter(a.iter().map(|x| Cow::Borrowed(x)))
    }

    /// Create HashMap<T, usize> from Vec<T> which value is appearance
    /// location
    pub fn to_enumhashmap<T>(v: &[T]) -> HashMap<Cow<T>, usize>
        where T: Clone + Hash + Eq
    {
        // ToDo: Handle duplicates

        let mut map: HashMap<Cow<T>, usize> = HashMap::with_capacity(v.len());
        for (i, key) in v.iter().enumerate() {
            // ToDo: Handle duplicates
            match map.entry(Cow::Borrowed(key)) {
                Entry::Occupied(_) => panic!("duplicates are not allowed"),
                Entry::Vacant(e) => e.insert(i),
            };
        }
        map
    }
}


pub fn union<T>(a: &[T], b: &[T]) -> Vec<T>
    where T: Clone + Hash + Eq
{
    // Use HashMap to keep the order

    // Clone for result Vec
    let mut res: Vec<T> = a.iter().cloned().collect();

    let set = CowCollections::to_hashset(a);

    for key in b.iter() {
        if !set.contains(&Cow::Borrowed(key)) {
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
