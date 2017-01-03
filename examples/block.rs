extern crate brassfibre;
use brassfibre::*;

fn main() {
    // cargo build --example block
    // ./target/debug/examples/block

    println!("** Creation **");
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let b = Block::from_col_vec(values, vec![10, 20, 30, 40, 50], vec!["X", "YYY", "ZZ"]);
    println!("{:?}\n", &b);

    println!("** Selection **");

    println!("by single columns label\n{:?}\n", &b.get(&"YYY"));
    println!("by single columns location\n{:?}\n", &b.iget(&0));
    println!("by multiple index labels\n{:?}\n",
             &b.locs(&vec![20, 30, 40]));
    println!("by multiple index locations\n{:?}\n",
             &b.ilocs(&vec![0, 2, 1]));

    println!("** Calculation **");
    println!("elemwise\n{:?}\n", &(&b + 5));
    println!("with another Block\n{:?}\n", &(&b + (&b * 2)));
    println!("sum\n{:?}\n", &b.sum());
    println!("mean\n{:?}\n", &b.mean());

    println!("** GroupBy **");
    let bg = b.groupby(vec!["A", "A", "B", "A", "B"]);
    println!("get group\n{:?}\n", &bg.get_group(&"A"));

    println!("grouped sum\n{:?}\n", &bg.sum());
}
