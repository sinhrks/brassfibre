use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::hash::Hash;

/// mark duplicates to drop in each columns
pub enum Duplicates {
    First,
    Last,
    None
}

impl Duplicates {

    pub fn duplicated<T>(a: &Vec<T>, how: Duplicates) -> Vec<bool>
        where T: Hash + Eq + Copy {

        match how {
            Duplicates::First => { Duplicates::duplicated_keepfirst(a) },
            Duplicates::Last => { Duplicates::duplicated_keeplast(a) },
            Duplicates::None => { Duplicates::duplicated_keepnone(a) }
        }
    }

    fn duplicated_keepfirst<T>(a: &Vec<T>) -> Vec<bool>
        where T: Hash + Eq + Copy {
        // ToDo: Change return value to BitVec
        let mut res: Vec<bool> = Vec::with_capacity(a.len());
        let mut set: HashSet<T> = HashSet::with_capacity(a.len());

        for v in a.iter() {
            if set.contains(v) {
                res.push(true)
            } else {
                set.insert(*v);
                res.push(false);
            }
        }
        res
    }

    fn duplicated_keeplast<T>(a: &Vec<T>) -> Vec<bool>
        where T: Hash + Eq + Copy {

        let mut res: Vec<bool> = Vec::with_capacity(a.len());
        let mut map: HashMap<T, usize> = HashMap::with_capacity(a.len());

        for (i, v) in a.iter().enumerate() {
            match map.entry(*v) {
                // Do nothing if already occupied
                Entry::Occupied(mut e) => {
                    let idx = e.insert(i);
                    res[idx] = true;
                    res.push(false);
                },
                Entry::Vacant(e) => {
                    e.insert(i);
                    res.push(false);
                }
            }
        }
        res
    }

    fn duplicated_keepnone<T>(a: &Vec<T>) -> Vec<bool>
        where T: Hash + Eq + Copy {

        let mut res: Vec<bool> = Vec::with_capacity(a.len());
        let mut map: HashMap<T, usize> = HashMap::with_capacity(a.len());

        for (i, v) in a.iter().enumerate() {
            match map.entry(*v) {
                // Do nothing if already occupied
                Entry::Occupied(e) => {
                    res[*e.get()] = true;
                    res.push(true);
                },
                Entry::Vacant(e) => {
                    e.insert(i);
                    res.push(false);
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {

    use super::Duplicates;

    #[test]
    fn test_vec_duplicates_int() {
        let key = vec![1, 1, 2, 3, 1, 2, 1];

        let res = Duplicates::duplicated(&key, Duplicates::First);
        assert_eq!(res, vec![false, true, false, false, true, true, true]);

        let res = Duplicates::duplicated(&key, Duplicates::Last);
        assert_eq!(res, vec![true, true, true, false, true, false, false]);

        let res = Duplicates::duplicated(&key, Duplicates::None);
        assert_eq!(res, vec![true, true, true, false, true, true, true]);
    }

    #[test]
    fn test_vec_duplicates_str() {
        let key = vec!["a", "b", "c", "b", "a", "c", "d", "b"];

        let res = Duplicates::duplicated(&key, Duplicates::First);
        assert_eq!(res, vec![false, false, false, true, true, true, false, true]);

        let res = Duplicates::duplicated(&key, Duplicates::Last);
        assert_eq!(res, vec![true, true, true, true, false, false, false, false]);

        let res = Duplicates::duplicated(&key, Duplicates::None);
        assert_eq!(res, vec![true, true, true, true, true, true, false, true]);
    }
}