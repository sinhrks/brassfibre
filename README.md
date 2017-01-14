# brassfibre

Provides multiple-dtype columner storage, known as DataFrame in pandas/R.

[![Release](https://img.shields.io/crates/v/brassfibre.svg)](https://crates.io/crates/brassfibre)
[![Documentation](https://docs.rs/brassfibre/badge.svg)](https://docs.rs/brassfibre/)
[![Build Status](https://travis-ci.org/sinhrks/brassfibre.svg?branch=master)](https://travis-ci.org/sinhrks/brassfibre)

## Series

Single-dtype 1-dimentional vector with label (index).

#### Creation

```rust
extern crate brassfibre;
use brassfibre::prelude::*;

let values: Vec<i64> = vec![1, 2, 3, 4, 3];
let index: Vec<i64> = vec![10, 20, 30, 40, 50];
let s = Series::<i64, i64>::new(values, index);

println!("{:?}", &s);
// 10 1
// 20 2
// 30 3
// 40 4
// 50 3
```

#### Selection

```rust
println!("{:?}", &s.locs(&vec![10, 40, 50]));
// 10 1
// 40 4
// 50 3

println!("{:?}", &s.ilocs(&vec![2, 3, 4]));
// 30 3
// 40 4
// 50 3
```

#### Calculation

```rust
println!("{:?}", &(&b + 5));
// 10 2
// 20 3
// 30 4
// 40 5
// 50 4

println!("{:?}", &s.sum());
// 13

println!("{:?}", &s.describe());
// count                 5
//  mean               2.6
//   std 1.019803902718557
//   min                 1
//   max                 4

println!("{:?}", s.value_counts());
// 3 2
// 2 1
// 1 1
// 4 1
```

#### Group By

```rust
let sg = s.groupby(vec![1, 1, 1, 2, 2]);

println!("{:?}", sg.get_group(&1));
// 10 1
// 20 2
// 30 3

println!("{:?}", sg.sum());
// 1 6
// 2 7
```

## Block

Single-dtype 2-dimentional vector with labels (index and columns).

#### Creation

```rust
extern crate brassfibre;
use brassfibre::prelude::*;

let values = vec![1, 2, 3, 4, 5,
                  6, 7, 8, 9, 10,
                  11, 12, 13, 14, 15];
let b = Block::from_col_vec(values,
                            vec![10, 20, 30, 40, 50],
                            vec!["X", "YYY", "ZZ"]);
println!("{:?}", &b);
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
println!("{:?}", &b.get(&"YYY"));
// 10  6
// 20  7
// 30  8
// 40  9
// 50 10
```

by single columns location

```rust
println!("{:?}", &b.iget(&0));
// 10 1
// 20 2
// 30 3
// 40 4
// 50 5
```

by multiple index labels

```rust
println!("{:?}", &b.locs(&vec![20, 30, 40]));
//    X YYY ZZ
// 20 2   7 12
// 30 3   8 13
// 40 4   9 14
```

by multiple index locations

```rust
println!("{:?}", &b.ilocs(&vec![0, 2, 1]));
//    X YYY ZZ
// 10 1   6 11
// 30 3   8 13
// 20 2   7 12
```

#### Calculation

elemwise, with another block

```rust
println!("{:?}", &(&b + 5));
//     X YYY ZZ
// 10  6  11 16
// 20  7  12 17
// 30  8  13 18
// 40  9  14 19
// 50 10  15 20

println!("{:?}", &(&b + (&b * 2)));
//     X YYY ZZ
// 10  3  18 33
// 20  6  21 36
// 30  9  24 39
// 40 12  27 42
// 50 15  30 45
```

aggregation

```rust
println!("{:?}", &b.sum());
//   X 15
// YYY 40
//  ZZ 65

println!("{:?}", &b.mean());
//   X  3
// YYY  8
//  ZZ 13
```

#### Group By

get group

```rust
let bg = b.groupby(vec!["A", "A", "B", "A", "B"]);
println!("{:?}", &bg.get_group(&"A"));
//    X YYY ZZ
// 10 1   6 11
// 20 2   7 12
// 40 4   9 14
```

aggregation

```rust
println!("{:?}", &bg.sum());
//   X YYY ZZ
// A 7  22 37
// B 8  18 28
```

## DataFrame

Multiple-dtype 2-dimentional vector with labels (index and columns).

#### Creation

```rust
#[macro_use]
extern crate brassfibre;
use brassfibre::prelude::*;

let values = vec![array![1, 2, 3, 4, 5],
                  array![6.1, 7.1, 8.1, 9.1, 10.1],
                  array![11, 12, 13, 14, 15]];
let df = DataFrame::from_vec(values,
                             vec![10, 20, 30, 40, 50],
                             vec!["X", "YYY", "ZZ"]);
println!("{:?}", &df);
//    X  YYY ZZ
// 10 1  6.1 11
// 20 2  7.1 12
// 30 3  8.1 13
// 40 4  9.1 14
// 50 5 10.1 15
```
#### Selection

by multiple index labels

```rust
println!("{:?}", &df.locs(&vec![20, 30, 40]));
//    X YYY ZZ
// 20 2 7.1 12
// 30 3 8.1 13
// 40 4 9.1 14
```

by multiple index locations

```rust
println!("{:?}", &df.ilocs(&vec![0, 2, 1]));
//    X YYY ZZ
// 10 1 6.1 11
// 30 3 8.1 13
// 20 2 7.1 12
```

#### Group By

get group

```rust
let dg = df.groupby(vec!["A", "A", "B", "A", "B"]);
println!("{:?}", &dg.get_group(&"A"));
//    X YYY ZZ
// 10 1 6.1 11
// 20 2 7.1 12
// 40 4 9.1 14
```

#### Reshaping

inner join

```rust
let values2 = vec![array![1.1, 2.1, 3.1],
                   array![6, 7, 8]];
let df2 = DataFrame::from_vec(values2,
                              vec![20, 30, 40],
                              vec!["X2", "Y2"]);
let j = df.join_inner(&df2);
println!("{:?}", &j);
//    X YYY ZZ  X2 Y2
// 20 2 7.1 12 1.1  6
// 30 3 8.1 13 2.1  7
// 40 4 9.1 14 3.1  8
```
