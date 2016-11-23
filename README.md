# brassfibre

Rust labeled vector experiment.

## Series

Single-dtype 1-dimentional vector with label (index).

#### Creation

```rust
use brassfibre::*;

let values: Vec<i64> = vec![1, 2, 3, 4, 3];
let index: Vec<i64> = vec![10, 20, 30, 40, 50];
let s = Series::<i64, i64>::new(values, index);

println!("{:?}\n", &s);
// 10 1
// 20 2
// 30 3
// 40 4
// 50 3
```

#### Selection

```rust
println!("by multiple index labels\n{:?}\n", &s.locs(&vec![10, 40, 50]));
// 10 1
// 40 4
// 50 3

println!("by multiple index locations\n{:?}\n", &s.ilocs(&vec![2, 3, 4]));
// 30 3
// 40 4
// 50 3
```

#### Calculation

```rust
println!("{:?}\n", &(&b + 5));
// 10 2
// 20 3
// 30 4
// 40 5
// 50 4

println!("sum\n{:?}\n", &s.sum());
// 13

println!("{:?}\n", &s.describe());
// count                 5
//  mean               2.6
//   std 1.019803902718557
//   min                 1
//   max                 4

println!("{:?}\n", s.value_counts());
// 3 2
// 2 1
// 1 1
// 4 1
```

#### Group By

```rust
let sg = s.groupby(vec![1, 1, 1, 2, 2]);

println!("{:?}\n", sg.get_group(&1));
// 10 1
// 20 2
// 30 3

println!("{:?}\n", sg.sum());
// 1 6
// 2 7
```

## Block

Single-dtype 2-dimentional vector with labels (index and columns).

#### Creation

```rust
use brassfibre::*;

let values = vec![1, 2, 3, 4, 5,
                  6, 7, 8, 9, 10,
                  11, 12, 13, 14, 15];
let b = Block::from_col_vec(values,
                            vec![10, 20, 30, 40, 50],
                            vec!["X", "YYY", "ZZ"]);
println!("{:?}\n", &b);
//    X YYY ZZ
// 10 1   6 11
// 20 2   7 12
// 30 3   8 13
// 40 4   9 14
// 50 5  10 15
```

#### Selection

by single columns label

```rust
println!("{:?}\n", &b.get(&"YYY"));
// 10  6
// 20  7
// 30  8
// 40  9
// 50 10
```

by single columns location

```rust
println!("{:?}\n", &b.iget(&0));
// 10 1
// 20 2
// 30 3
// 40 4
// 50 5
```

by multiple index labels

```rust
println!("{:?}\n", &b.locs(&vec![20, 30, 40]));
//    X YYY ZZ
// 20 2   7 12
// 30 3   8 13
// 40 4   9 14
```

by multiple index locations

```rust
println!("{:?}\n", &b.ilocs(&vec![0, 2, 1]));
//    X YYY ZZ
// 10 1   6 11
// 30 3   8 13
// 20 2   7 12
```

#### Calculation

elemwise, with another block

```rust
println!("{:?}\n", &(&b + 5));
//     X YYY ZZ
// 10  6  11 16
// 20  7  12 17
// 30  8  13 18
// 40  9  14 19
// 50 10  15 20

println!("{:?}\n", &(&b + (&b * 2)));
//     X YYY ZZ
// 10  3  18 33
// 20  6  21 36
// 30  9  24 39
// 40 12  27 42
// 50 15  30 45
```

aggregation

```
println!("{:?}\n", &b.sum());
//   X 15
// YYY 40
//  ZZ 65

println!("{:?}\n", &b.mean());
//   X  3
// YYY  8
//  ZZ 13
```

#### Group By

get group

```rust
let bg = b.groupby(vec!["A", "A", "B", "A", "B"]);
println!("{:?}\n", &bg.get_group(&"A"));
//    X YYY ZZ
// 10 1   6 11
// 20 2   7 12
// 40 4   9 14
```

aggregation

```rust
println!("{:?}\n", &bg.sum());
//   X YYY ZZ
// A 7  22 37
// B 8  18 28
```
