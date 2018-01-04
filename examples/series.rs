extern crate brassfibre;
use brassfibre::prelude::*;

fn main() {
    // cargo build --example series
    // ./target/debug/examples/series

    println!("** Creation **");
    let values: Vec<i64> = vec![1, 2, 3, 4, 3];
    let index: Vec<i64> = vec![10, 20, 30, 40, 50];
    let s = Series::<i64, i64>::new(values, index);

    println!("{:?}\n", &s);

    println!("** Selection **");
    println!("by multiple index labels\n{:?}\n", &s.locs(&[10, 40, 50]));
    println!("by multiple index locations\n{:?}\n", &s.ilocs(&[2, 3, 4]));

    println!("** Calculation **");
    println!("elemwise\n{:?}\n", &(&s + 1));
    println!("sum\n{:?}\n", &s.sum());
    println!("describe\n{:?}\n", &s.describe());
    println!("histogram\n{:?}\n", s.value_counts());

    println!("** Group By **");
    let sg = s.groupby(&[1, 1, 1, 2, 2]);
    // println!("{:?}", sg.grouper);
    println!("{:?}\n", sg.get_group(&1));
    println!("{:?}\n", sg.sum());
}
