extern crate brassfibre;
use brassfibre::*;

fn main() {
    // cargo build --example indexer
    // ./target/debug/examples/indexer

    let values: Vec<i64> = vec![1, 2, 3, 4, 5];
    let indexer: Indexer<i64> = Indexer::new(values);
    // println!("{:}", &s);
    println!("{:?}", &indexer);
    println!("{:?}", &indexer[2]);

    println!("{:?}", &(indexer + 1));
}
