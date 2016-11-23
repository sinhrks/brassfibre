use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

pub struct Counter<T: Hash + Eq + Copy> {
    counts : HashMap<T, usize>,
}

impl<T: Hash + Eq + Copy> Counter<T> {

    pub fn new(values: &Vec<T>) -> Counter<T> {

        // ToDo: allocation is too much?
        let mut map: HashMap<T, usize> = HashMap::with_capacity(values.len() / 4);

        for v in values.iter() {
            let e = map.entry(*v).or_insert(0);
            *e += 1;
        }
        Counter { counts: map }
    }

    pub fn len(&self) -> usize {
        self.counts.len()
    }

    pub fn get_results(&self) -> (Vec<T>, Vec<usize>) {
        // sort by value, not using Sorter::sort_by for Vec
        let mut map: BTreeMap<usize, Vec<T>> = BTreeMap::new();

        for (&k, &c) in &self.counts {
            let e = map.entry(c).or_insert(Vec::<T>::new());
            e.push(k);
        }

        let mut keys: Vec<T> = Vec::with_capacity(self.len());
        let mut counts: Vec<usize> = Vec::with_capacity(self.len());

        // Btreemap.iter implements DoubleEndedIterator
        for (c, key) in map.iter().rev() {
            for k in key {
                counts.push(*c);
                keys.push(*k);
            }
        }
        (keys, counts)
    }
}

#[cfg(test)]
mod tests {

    use super::Counter;

    #[test]
    fn test_counter_empty() {
        let c: Counter<i64> = Counter::new(&vec![]);
        assert_eq!(c.len(), 0);

        let (keys, counts) = c.get_results();
        assert_eq!(keys, vec![]);
        assert_eq!(counts, vec![]);
    }

    #[test]
    fn test_counter_int() {
        let c: Counter<i64> = Counter::new(&vec![2, 2, 2, 3, 3]);
        assert_eq!(c.len(), 2);

        let (keys, counts) = c.get_results();
        assert_eq!(keys, vec![2, 3]);
        assert_eq!(counts, vec![3, 2]);
    }

    #[test]
    fn test_counter_int_dup() {
        let c: Counter<i64> = Counter::new(&vec![2, 2, 3, 3]);
        assert_eq!(c.len(), 2);

        let (keys, counts) = c.get_results();
        // same counts doesn't guarantee the order
        if keys[0] == 2 {
            assert_eq!(keys, vec![2, 3]);
            assert_eq!(counts, vec![2, 2]);
        } else {
            assert_eq!(keys, vec![3, 2]);
            assert_eq!(counts, vec![2, 2]);
        }
    }

    #[test]
    fn test_counter_str() {
        let c: Counter<&str> = Counter::new(&vec!["a", "b", "b", "a", "b", "c"]);
        assert_eq!(c.len(), 3);

        let (keys, counts) = c.get_results();
        assert_eq!(keys, vec!["b", "a", "c"]);
        assert_eq!(counts, vec![3, 2, 1]);
    }
}