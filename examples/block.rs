extern crate brassfibre;

use brassfibre::block::Block;
use brassfibre::series::Series;

fn main() {
    // cargo build --example block
    // ./target/debug/examples/block

    println!("** Creation **");
    let values: Vec<i64> = vec![1, 2, 3, 4, 5];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let s = Series::<i64, i64>::new(values, index);
    let mut b = Block::from_series(s, "X");
    // println!("{:}", &b);
    // println!("{:?}", &b);

    let new_values: Vec<i64> = vec![6, 7, 8, 9, 10];
    b.add_columns(new_values, "YYY");
    // println!("{:}", &b);

    let values = vec![1, 2, 3, 4, 5,
                      6, 7, 8, 9, 10,
                      11, 12, 13, 14, 15];
    let mut b = Block::from_col_vec(values,
                                    vec![10, 20, 30, 40, 50],
                                    vec!["X", "YYY", "ZZ"]);
    println!("{:?}", &b);

    println!("** Slicing / Calculation **");
    println!("{:?}", &b.get_column_by_label(&"YYY"));
    println!("{:?}", &b.sum());
    println!("{:?}", &b.mean());

    println!("** GroupBy **");
    let bg = b.groupby(vec!["A", "A", "B", "A", "B"]);
    let a = bg.get_group(&"A");
    println!("{:?}", &a);
    println!("{:?}", &a.sum());
}