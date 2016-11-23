extern crate brassfibre;

use brassfibre::*;

fn main() {
    // cargo build --example block
    // ./target/debug/examples/block

    println!("** Creation **");
    let values = vec![1, 2, 3, 4, 5,
                      6, 7, 8, 9, 10,
                      11, 12, 13, 14, 15];
    let mut b = Block::from_col_vec(values,
                                    vec![10, 20, 30, 40, 50],
                                    vec!["X", "YYY", "ZZ"]);
    println!("{:?}\n", &b);
    println!("** Slicing / Calculation **");

    println!("{:?}\n", &b.locs(&vec![20, 30, 40]));
    println!("{:?}\n", &b.transpose());

    println!("{:?}\n", &b.get(&"YYY"));
    println!("{:?}\n", &b.sum());
    println!("{:?}\n", &b.mean());

    println!("** GroupBy **");
    let bg = b.groupby(vec!["A", "A", "B", "A", "B"]);
    let a = bg.get_group(&"A");
    println!("{:?}\n", &a);

    println!("{:?}\n", &bg.sum());

    println!("** Numeric Op **");
    println!("{:?}\n", &(&b + 5));
}