use std::borrow::{Borrow, Cow};
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

pub struct Counter<'a, T>
    where T: 'a + Clone + Hash + Eq {

    counts : HashMap<Cow<'a, T>, usize>,
}

impl<'a, T> Counter<'a, T>
    where T: Clone + Hash + Eq {

    pub fn new(values: &'a [T]) -> Counter<'a, T> {

        // ToDo: allocation is too much?
        let mut map: HashMap<Cow<T>, usize> = HashMap::with_capacity(values.len() / 4);

        for v in values.iter() {
            let e = map.entry(Cow::Borrowed(v)).or_insert(0);
            *e += 1;
        }
        Counter { counts: map }
    }

    pub fn len(&self) -> usize {
        self.counts.len()
    }

    pub fn get_results(&self) -> (Vec<T>, Vec<usize>) {
        // sort by value, not using Sorter::sort_by for Vec
        let mut map: BTreeMap<usize, Vec<Cow<T>>> = BTreeMap::new();

        for (ref k, ref c) in self.counts.iter() {
            let e = map.entry(**c).or_insert(Vec::<Cow<T>>::new());
            e.push(Cow::Borrowed(k));
        }

        let mut keys: Vec<T> = Vec::with_capacity(self.len());
        let mut counts: Vec<usize> = Vec::with_capacity(self.len());

        // Btreemap.iter implements DoubleEndedIterator
        for (c, key) in map.iter().rev() {
            for k in key {
                counts.push(*c);
                // Clone to return value
                keys.push(k.clone().into_owned());
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
        let vals: Vec<i64> = vec![];
        let c: Counter<i64> = Counter::new(&vals);
        assert_eq!(c.len(), 0);

        let (keys, counts) = c.get_results();
        assert_eq!(keys, vec![]);
        assert_eq!(counts, vec![]);
    }

    #[test]
    fn test_counter_int() {
        let vals: Vec<i64> = vec![2, 2, 2, 3, 3];
        // vals must live long until get results
        let c: Counter<i64> = Counter::new(&vals);
        assert_eq!(c.len(), 2);

        let (keys, counts) = c.get_results();
        assert_eq!(keys, vec![2, 3]);
        assert_eq!(counts, vec![3, 2]);
    }

    #[test]
    fn test_counter_int_dup() {
        let vals: Vec<i64> = vec![2, 2, 3, 3];
        let c: Counter<i64> = Counter::new(&vals);
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
        let vals: Vec<&str> = vec!["a", "b", "b", "a", "b", "c"];
        let c: Counter<&str> = Counter::new(&vals);
        assert_eq!(c.len(), 3);

        let (keys, counts) = c.get_results();
        assert_eq!(keys, vec!["b", "a", "c"]);
        assert_eq!(counts, vec![3, 2, 1]);
    }

    #[test]
    fn test_counter_string() {
        let vals: Vec<String> = vec!["a".to_string(), "b".to_string(),
                                     "b".to_string(), "a".to_string(),
                                     "b".to_string(), "c".to_string()];
        let c: Counter<String> = Counter::new(&vals);
        assert_eq!(c.len(), 3);

        let (keys, counts) = c.get_results();
        assert_eq!(keys, vec!["b".to_string(), "a".to_string(),
                              "c".to_string()]);
        assert_eq!(counts, vec![3, 2, 1]);
    }
}