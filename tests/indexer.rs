extern crate brassfibre;
use brassfibre::*;

#[test]
fn test_index_creation_from_len() {
    let idx: Indexer<usize> = Indexer::<usize>::from_len(3);
    assert_eq!(idx.values, vec![0, 1, 2]);
    assert_eq!(idx.len(), 3);

    let idx: Indexer<usize> = Indexer::<usize>::from_len(0);
    assert_eq!(idx.values, vec![]);
    assert_eq!(idx.len(), 0);
}

#[test]
fn test_index_creation_int64() {
    let values: Vec<i64> = vec![1, 2, 3];
    let idx = Indexer::<i64>::new(values);

    let exp_index: Vec<i64> = vec![1, 2, 3];
    assert_eq!(idx.values, exp_index);
    assert_eq!(idx.len(), 3);
}

#[test]
fn test_index_loc_int64() {
    let values: Vec<i64> = vec![1, 2, 3];
    let idx = Indexer::<i64>::new(values);

    assert_eq!(idx.get_loc(&1), 0);
    assert_eq!(idx.get_loc(&3), 2);

    assert_eq!(idx.get_locs(&vec![1, 3]), vec![0, 2]);
    assert_eq!(idx.get_locs(&vec![3, 2]), vec![2, 1]);

    assert_eq!(idx.contains(&1), true);
    assert_eq!(idx.contains(&5), false);
}

#[test]
fn test_index_creation_str() {
    let values: Vec<&str> = vec!["A", "B", "C"];
    let idx = Indexer::<&str>::new(values);

    let exp_index: Vec<&str> = vec!["A", "B", "C"];
    assert_eq!(idx.values, exp_index);
    assert_eq!(idx.len(), 3);
}

#[test]
fn test_index_creation_string() {
    let values: Vec<String> = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    let idx = Indexer::<String>::new(values);

    let exp_index: Vec<String> = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    assert_eq!(idx.values, exp_index);
    assert_eq!(idx.len(), 3);
}

#[test]
fn test_index_loc_str() {
    let values: Vec<&str> = vec!["A", "B", "C"];
    let idx = Indexer::<&str>::new(values);

    assert_eq!(idx.get_loc(&"B"), 1);
    assert_eq!(idx.get_loc(&"C"), 2);

    assert_eq!(idx.get_locs(&vec!["B", "C"]), vec![1, 2]);
    assert_eq!(idx.get_locs(&vec!["A", "C"]), vec![0, 2]);

    assert_eq!(idx.contains(&"C"), true);
    assert_eq!(idx.contains(&"X"), false);
}

#[test]
fn test_copy() {
    let values: Vec<&str> = vec!["A", "B", "C"];
    let idx = Indexer::<&str>::new(values);

    // copy Indexer
    let copied = idx.clone();
    let exp_values: Vec<&str> = vec!["A", "B", "C"];
    assert_eq!(&copied.values, &exp_values);
}

#[test]
fn test_equals() {
    let idx = Indexer::<&str>::new(vec!["A", "B", "C"]);

    let other = Indexer::<&str>::new(vec!["A", "B"]);
    assert_eq!(idx == other, false);

    let other = Indexer::<&str>::new(vec!["A", "B", "X"]);
    assert_eq!(idx == other, false);

    let other = Indexer::<&str>::new(vec!["A", "B", "C"]);
    assert_eq!(idx == other, true);
    assert_eq!(idx, other);
}

#[test]
fn test_index_push() {
    let values: Vec<&str> = vec!["A", "B", "C"];
    let mut idx = Indexer::<&str>::new(values);

    let exp_index: Vec<&str> = vec!["A", "B", "C"];
    assert_eq!(idx.values, exp_index);
    assert_eq!(idx.len(), 3);
    assert_eq!(idx.get_loc(&"C"), 2);

    idx.push("D");
    assert_eq!(idx.len(), 4);
    assert_eq!(idx.get_loc(&"C"), 2);
    assert_eq!(idx.get_loc(&"D"), 3);

    idx.push("E");
    assert_eq!(idx.len(), 5);
    assert_eq!(idx.get_loc(&"D"), 3);
    assert_eq!(idx.get_loc(&"E"), 4);
}

#[test]
fn test_reindex() {
    let idx = Indexer::<&str>::new(vec!["A", "B", "C"]);

    let res = idx.reindex(&vec![1, 0, 2]);
    assert_eq!(res, Indexer::new(vec!["B", "A", "C"]));

    let res = idx.reindex(&vec![1, 0, 2]);
    assert_eq!(res, Indexer::new(vec!["B", "A", "C"]));
}

#[test]
fn test_index_append() {
    let index1: Indexer<i64> = Indexer::new(vec![1, 2, 3]);
    let index2: Indexer<i64> = Indexer::new(vec![4, 5, 6]);
    let res = index1.append(&index2);

    let exp: Indexer<i64> = Indexer::new(vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(res, exp)
}

#[test]
fn test_index_into_iter() {
    let index: Indexer<i64> = Indexer::new(vec![1, 2, 3]);
    let mut it = index.into_iter();
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), Some(3));
    assert_eq!(it.next(), None);
}

#[test]
fn test_index_iter() {
    let index: Indexer<i64> = Indexer::new(vec![1, 2, 3]);
    let mut it = index.iter();
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), None);
}

#[test]
fn test_index_from_iter() {
    let index: Indexer<i64> = (3..6).collect();
    let exp: Indexer<i64> = Indexer::new(vec![3, 4, 5]);
    assert_eq!(index, exp);
}