extern crate brassfibre;

use brassfibre::formatting;

fn main() {
    // cargo build --example formatting
    // ./target/debug/examples/formatting

    let values: Vec<i64> = vec![1, 2, 355, 4, 5];
    let strs = formatting::pad_string_vector(&values);

    println!("{:?}", &strs);
}
