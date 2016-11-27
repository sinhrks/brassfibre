use std::cmp;
use std::hash::Hash;
use std::usize;

use super::set::{union, to_enumhashmap};

const USIZE_MISSING: usize = usize::MAX;


pub struct HashJoin;

pub trait Join<T> {
    fn inner(left: &Vec<T>, right: &Vec<T>) -> (Vec<T>, Vec<usize>, Vec<usize>);
    fn left(left: &Vec<T>, right: &Vec<T>) -> (Vec<T>, Vec<usize>, Vec<usize>);
    fn right(left: &Vec<T>, right: &Vec<T>) -> (Vec<T>, Vec<usize>, Vec<usize>);
    fn outer(left: &Vec<T>, right: &Vec<T>) -> (Vec<T>, Vec<usize>, Vec<usize>);

    fn keep_first(keep: &Vec<T>, other: &Vec<T>) -> (Vec<T>, Vec<usize>, Vec<usize>);
}

impl<T> Join<T> for HashJoin where T: Hash + Eq + Copy {

    fn inner(left: &Vec<T>, right: &Vec<T>) -> (Vec<T>, Vec<usize>, Vec<usize>) {

        let exp_capacity = cmp::min(left.len(), right.len());

        let mut indexer: Vec<T> = Vec::with_capacity(exp_capacity);
        let mut lindexer: Vec<usize> = Vec::with_capacity(exp_capacity);
        let mut rindexer: Vec<usize> = Vec::with_capacity(exp_capacity);

        let map = to_enumhashmap(right);

        // keep left order
        for (i, key) in left.iter().enumerate() {
            // ToDo: sort?
            match map.get(&key) {
                Some(val) => {
                    indexer.push(*key);
                    lindexer.push(i);
                    rindexer.push(*val);
                },
                None => {}
            }
        }
        (indexer, lindexer, rindexer)
    }

    fn left(left: &Vec<T>, right: &Vec<T>) -> (Vec<T>, Vec<usize>, Vec<usize>) {
        HashJoin::keep_first(&left, &right)
    }

    fn right(left: &Vec<T>, right: &Vec<T>) -> (Vec<T>, Vec<usize>, Vec<usize>) {
        let res = HashJoin::keep_first(&right, &left);
        (res.0, res.2, res.1)
    }

    /// internal fn for left or right join
    /// values in keep is being kept
    fn keep_first(keep: &Vec<T>, other: &Vec<T>) -> (Vec<T>, Vec<usize>, Vec<usize>) {


        let exp_capacity = keep.len();

        let mut indexer: Vec<T> = Vec::with_capacity(exp_capacity);
        let mut kindexer: Vec<usize> = Vec::with_capacity(exp_capacity);
        let mut oindexer: Vec<usize> = Vec::with_capacity(exp_capacity);

        let map = to_enumhashmap(other);

        for (i, key) in keep.iter().enumerate() {
            // ToDo: sort?
            match map.get(key) {
                Some(loc) => {
                    indexer.push(*key);
                    kindexer.push(i);
                    oindexer.push(*loc);
                },
                None => {
                    indexer.push(*key);
                    kindexer.push(i);
                    oindexer.push(USIZE_MISSING);
                }
            }
        }
        (indexer, kindexer, oindexer)
    }

    fn outer(left: &Vec<T>, right: &Vec<T>) -> (Vec<T>, Vec<usize>, Vec<usize>) {

        let exp_capacity = cmp::max(left.len(), right.len());

        // let mut indexer: Vec<T> = Vec::with_capacity(exp_capacity);
        let mut lindexer: Vec<usize> = Vec::with_capacity(exp_capacity);
        let mut rindexer: Vec<usize> = Vec::with_capacity(exp_capacity);

        let lmap = to_enumhashmap(left);
        let rmap = to_enumhashmap(right);

        let indexer = union(left, right);

        for key in &indexer {
            // ToDo: sort?
            match lmap.get(key) {
                Some(loc) => {
                    lindexer.push(*loc);
                },
                None => {
                    lindexer.push(USIZE_MISSING)
                }
            }
            match rmap.get(key) {
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
    fn test_vec_left_join() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![2, 3, 4];
        let res = HashJoin::left(&v1, &v2);

        assert_eq!(res.0, vec![1, 2, 3]);
        assert_eq!(res.1, vec![0, 1, 2]);
        assert_eq!(res.2, vec![USIZE_MISSING, 0, 1]);
    }

    #[test]
    fn test_vec_right_join() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![2, 3, 4];
        let res = HashJoin::right(&v1, &v2);

        assert_eq!(res.0, vec![2, 3, 4]);
        assert_eq!(res.1, vec![1, 2, USIZE_MISSING]);
        assert_eq!(res.2, vec![0, 1, 2]);
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

}
