use std::borrow::Cow;
use std::cmp;
use std::hash::Hash;
use std::usize;

use super::set::{CowCollections, union};

const USIZE_MISSING: usize = usize::MAX;


pub struct HashJoin;

pub trait Join<T> {
    fn inner(left: &[T], right: &[T]) -> (Vec<T>, Vec<usize>, Vec<usize>);
    fn left(left: &[T], right: &[T]) -> (Vec<T>, Vec<usize>, Vec<usize>);
    fn right(left: &[T], right: &[T]) -> (Vec<T>, Vec<usize>, Vec<usize>);
    fn outer(left: &[T], right: &[T]) -> (Vec<T>, Vec<usize>, Vec<usize>);

    fn keep_first(keep: &[T], other: &[T]) -> (Vec<T>, Vec<usize>, Vec<usize>);
}

impl<T> Join<T> for HashJoin
    where T: Clone + Hash + Eq {

    fn inner(left: &[T], right: &[T]) -> (Vec<T>, Vec<usize>, Vec<usize>) {

        let exp_capacity = cmp::min(left.len(), right.len());

        let mut indexer: Vec<T> = Vec::with_capacity(exp_capacity);
        let mut lindexer: Vec<usize> = Vec::with_capacity(exp_capacity);
        let mut rindexer: Vec<usize> = Vec::with_capacity(exp_capacity);

        let map = CowCollections::to_enumhashmap(right);

        // keep left order
        for (i, key) in left.iter().enumerate() {
            // ToDo: sort?
            match map.get(&Cow::Borrowed(key)) {
                Some(val) => {
                    indexer.push((*key).clone());
                    lindexer.push(i);
                    rindexer.push(*val);
                },
                None => {}
            }
        }
        (indexer, lindexer, rindexer)
    }

    fn left(left: &[T], right: &[T]) -> (Vec<T>, Vec<usize>, Vec<usize>) {
        HashJoin::keep_first(&left, &right)
    }

    fn right(left: &[T], right: &[T]) -> (Vec<T>, Vec<usize>, Vec<usize>) {
        let res = HashJoin::keep_first(&right, &left);
        (res.0, res.2, res.1)
    }

    /// internal fn for left or right join
    /// values in keep is being kept
    fn keep_first(keep: &[T], other: &[T]) -> (Vec<T>, Vec<usize>, Vec<usize>) {

        let exp_capacity = keep.len();

        let mut indexer: Vec<T> = Vec::with_capacity(exp_capacity);
        let mut kindexer: Vec<usize> = Vec::with_capacity(exp_capacity);
        let mut oindexer: Vec<usize> = Vec::with_capacity(exp_capacity);

        let map = CowCollections::to_enumhashmap(other);

        for (i, key) in keep.iter().enumerate() {
            // ToDo: sort?
            match map.get(&Cow::Borrowed(key)) {
                Some(loc) => {
                    indexer.push((*key).clone());
                    kindexer.push(i);
                    oindexer.push(*loc);
                },
                None => {
                    indexer.push((*key).clone());
                    kindexer.push(i);
                    oindexer.push(USIZE_MISSING);
                }
            }
        }
        (indexer, kindexer, oindexer)
    }

    fn outer(left: &[T], right: &[T]) -> (Vec<T>, Vec<usize>, Vec<usize>) {

        let exp_capacity = cmp::max(left.len(), right.len());

        // let mut indexer: Vec<T> = Vec::with_capacity(exp_capacity);
        let mut lindexer: Vec<usize> = Vec::with_capacity(exp_capacity);
        let mut rindexer: Vec<usize> = Vec::with_capacity(exp_capacity);

        let lmap = CowCollections::to_enumhashmap(left);
        let rmap = CowCollections::to_enumhashmap(right);

        let indexer = union(left, right);

        for key in &indexer {
            // ToDo: sort?
            match lmap.get(&Cow::Borrowed(key)) {
                Some(loc) => {
                    lindexer.push(*loc);
                },
                None => {
                    lindexer.push(USIZE_MISSING)
                }
            }
            match rmap.get(&Cow::Borrowed(key)) {
                Some(loc) => {
                    rindexer.push(*loc);
                },
                None => {
                    rindexer.push(USIZE_MISSING)
                }
            }

        }

        (indexer, lindexer, rindexer)
    }
}

#[cfg(test)]
mod tests {

    use super::{Join, HashJoin, USIZE_MISSING};

    #[test]
    fn test_vec_inner_join() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![2, 3, 4];

        let res = HashJoin::inner(&v1, &v2);

        assert_eq!(res.0, vec![2, 3]);
        assert_eq!(res.1, vec![1, 2]);
        assert_eq!(res.2, vec![0, 1]);

        let v3 = vec![1, 2, 3];
        let v4 = vec![2, 1, 0];

        let res = HashJoin::inner(&v3, &v4);

        assert_eq!(res.0, vec![1, 2]);
        assert_eq!(res.1, vec![0, 1]);
        assert_eq!(res.2, vec![1, 0]);
    }

    #[test]
    fn test_vec_inner_join_string() {
        let v1: Vec<String> = vec!["a".to_string(), "b".to_string(),
                                   "c".to_string(), "d".to_string()];
        let v2: Vec<String> = vec!["d".to_string(), "c".to_string(),
                                   "e".to_string(), "a".to_string()];

        let res = HashJoin::inner(&v1, &v2);

        let exp: Vec<String> = vec!["a".to_string(), "c".to_string(), "d".to_string()];
        assert_eq!(res.0, exp);
        assert_eq!(res.1, vec![0, 2, 3]);
        assert_eq!(res.2, vec![3, 1, 0]);
    }

    #[test]
    fn test_vec_left_join() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![2, 3, 4];
        let res = HashJoin::left(&v1, &v2);

        assert_eq!(res.0, v1);
        assert_eq!(res.1, vec![0, 1, 2]);
        assert_eq!(res.2, vec![USIZE_MISSING, 0, 1]);
    }

    #[test]
    fn test_vec_left_join_string() {
        let v1: Vec<String> = vec!["a".to_string(), "b".to_string(),
                                   "c".to_string(), "d".to_string()];
        let v2: Vec<String> = vec!["d".to_string(), "c".to_string(),
                                   "e".to_string(), "a".to_string()];

        let res = HashJoin::left(&v1, &v2);

        assert_eq!(res.0, v1);
        assert_eq!(res.1, vec![0, 1, 2, 3]);
        assert_eq!(res.2, vec![3, USIZE_MISSING, 1, 0]);
    }

    #[test]
    fn test_vec_right_join() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![2, 3, 4];
        let res = HashJoin::right(&v1, &v2);

        assert_eq!(res.0, v2);
        assert_eq!(res.1, vec![1, 2, USIZE_MISSING]);
        assert_eq!(res.2, vec![0, 1, 2]);
    }

    #[test]
    fn test_vec_right_join_string() {
        let v1: Vec<String> = vec!["a".to_string(), "b".to_string(),
                                   "c".to_string(), "d".to_string()];
        let v2: Vec<String> = vec!["d".to_string(), "c".to_string(),
                                   "e".to_string(), "a".to_string()];

        let res = HashJoin::right(&v1, &v2);

        assert_eq!(res.0, v2);
        assert_eq!(res.1, vec![3, 2, USIZE_MISSING, 0]);
        assert_eq!(res.2, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_vec_outer_join() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![2, 3, 4];
        let res = HashJoin::outer(&v1, &v2);

        assert_eq!(res.0, vec![1, 2, 3, 4]);
        assert_eq!(res.1, vec![0, 1, 2, USIZE_MISSING]);
        assert_eq!(res.2, vec![USIZE_MISSING, 0, 1, 2]);
    }

    #[test]
    fn test_vec_outer_join_string() {
        let v1: Vec<String> = vec!["a".to_string(), "b".to_string(),
                                   "c".to_string(), "d".to_string()];
        let v2: Vec<String> = vec!["d".to_string(), "c".to_string(),
                                   "e".to_string(), "a".to_string()];

        let res = HashJoin::outer(&v1, &v2);

        let exp = vec!["a".to_string(), "b".to_string(), "c".to_string(),
                       "d".to_string(), "e".to_string()];
        assert_eq!(res.0, exp);
        assert_eq!(res.1, vec![0, 1, 2, 3, USIZE_MISSING]);
        assert_eq!(res.2, vec![3, USIZE_MISSING, 1, 0, 2]);
    }
}
