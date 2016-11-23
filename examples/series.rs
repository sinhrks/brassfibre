extern crate brassfibre;

use brassfibre::*;

fn main() {
    // cargo build --example series
    // ./target/debug/examples/series

    let values: Vec<i64> = vec![1, 2, 3, 4, 3];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let mut s = Series::<i64, i64>::new(values, index);

    println!("{:?}", &s);
    println!("{:?}", &s.locs(&vec![10, 40, 50]));

    println!("{:?}", &s.describe());
    println!("{:?}", s.value_counts());

    let sg = s.groupby(vec![1, 1, 1, 2, 2]);
    //println!("{:?}", sg.grouper);
    println!("{:?}", sg.get_group(&1));
    println!("{:?}", sg.sum());
}
