# brassfibre

Rust labeled vector experiment.

## Series

Single-dtype 1-dimentional vector with label (index).

~~~~
let values: Vec<i64> = vec![1, 2, 3, 4, 5];
let index: Vec<i64> = vec![10, 20, 30, 40, 50];
let mut s = Series::<i64, i64>::new(values, index);

println!("{:?}", &s);
// 10 1
// 20 2
// 30 3
// 40 4
// 50 5

println!("{:?}", &s.slice_by_label(&vec![10, 40, 50]));
// 10 1
// 40 4
// 50 5

println!("{:?}", &s.describe());
// count                  5
//  mean                  3
//   std 1.4142135623730951

let sg = s.groupby(vec![1, 1, 1, 2, 2]);
println!("{:?}", sg.get_group(&1));
// 10 1
// 20 2
// 30 3
~~~~  
  
## Block

Single-dtype 2-dimentional vector with labels (index and columns).

~~~~
let values = vec![1, 2, 3, 4, 5,
                  6, 7, 8, 9, 10,
                  11, 12, 13, 14, 15];
let mut b = Block::from_col_vec(values,
                                vec![10, 20, 30, 40, 50],
                                vec!["X", "YYY", "ZZ"]);
println!("{:?}", &b);
//    X YYY ZZ 
// 10 1   6 11 
// 20 2   7 12 
// 30 3   8 13 
// 40 4   9 14 
// 50 5  10 15 

println!("{:?}", &b.slice_by_label(&vec![20, 30, 40]));
//    X YYY ZZ 
// 20 2   7 12 
// 30 3   8 13 
// 40 4   9 14 

println!("{:?}", &b.get_column_by_label(&"YYY"));
// 10  6
// 20  7
// 30  8
// 40  9
// 50 10

println!("{:?}\n", &b.transpose());
//     10 20 30 40 50
//   X  1  2  3  4  5
// YYY  6  7  8  9 10
//  ZZ 11 12 13 14 15

println!("{:?}", &b.sum());
//   X 15
// YYY 40
//  ZZ 65

println!("{:?}", &b.mean());
//   X  3
// YYY  8
//  ZZ 13

let bg = b.groupby(vec!["A", "A", "B", "A", "B"]);
println!("{:?}\n", &bg.sum());
//   X YYY ZZ
// A 7  22 37
// B 8  18 28

let a = bg.get_group(&"A");
println!("{:?}", &a);
//    X YYY ZZ 
// 10 1   6 11 
// 20 2   7 12 
// 40 4   9 14 

println!("{:?}\n", &(b + 5));
//     X YYY ZZ
// 10  6  11 16
// 20  7  12 17
// 30  8  13 18
// 40  9  14 19
// 50 10  15 20
~~~~
