#[macro_use]
extern crate brassfibre;
use brassfibre::prelude::*;

fn main() {
    // cargo build --example frame
    // ./target/debug/examples/frame

    println!("** Creation **");
    let values = vec![
        array![1, 2, 3, 4, 5],
        array![6.1, 7.1, 8.1, 9.1, 10.1],
        array![11, 12, 13, 14, 15],
    ];
    let df = DataFrame::from_vec(values, vec![10, 20, 30, 40, 50], vec!["X", "YYY", "ZZ"]);
    println!("{:?}\n", &df);

    println!("** Selection **");

    println!("by multiple index labels\n{:?}\n", &df.locs(&[20, 30, 40]));
    println!("by multiple index locations\n{:?}\n", &df.ilocs(&[0, 2, 1]));

    println!("** GroupBy **");
    let dg = df.groupby(&["A", "A", "B", "A", "B"]);
    println!("get group\n{:?}\n", &dg.get_group(&"A"));

    println!("** Reshaping **");
    let values2 = vec![array![1.1, 2.1, 3.1], array![6, 7, 8]];
    let df2 = DataFrame::from_vec(values2, vec![20, 30, 40], vec!["X2", "Y2"]);
    let j = df.join_inner(&df2);
    println!("inner join\n{:?}\n", &j);
}
