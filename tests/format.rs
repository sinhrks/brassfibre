use std::io::Write;
use std::str;

#[macro_use]
extern crate brassfibre;
use brassfibre::prelude::*;

#[test]
fn test_series_format() {
    let s = Series::new(vec![1, 10, 100], vec!["XX", "Y", "ZZ"]);

    // better way?
    let mut buf = Vec::new();
    let _ = write!(&mut buf, "{}", s);
    assert_eq!(&buf, b"Series([1, 10, 100])");
}

#[test]
fn test_series_format_debug() {
    let s = Series::new(vec![1, 10, 100], vec!["XX", "Y", "ZZ"]);

    // better way?
    let mut buf = Vec::new();
    let _ = write!(&mut buf, "{:?}", s);
    assert_eq!(
        &buf,
        b"XX   1
 Y  10
ZZ 100"
    );
}

#[test]
fn test_dataframe_format() {
    let values = vec![
        array![1, 2, 3, 4, 5],
        array![6.1, 7.1, 8.1, 9.1, 10.1],
        array![11, 12, 13, 14, 15],
    ];
    let df = DataFrame::from_vec(values, vec![10, 20, 30, 40, 50], vec!["X", "YYY", "ZZ"]);

    // better way?
    let mut buf = Vec::new();
    let _ = write!(&mut buf, "{}", df);
    let res = str::from_utf8(&buf).unwrap();
    assert_eq!(res, "DataFrame(columns=[\"X\", \"YYY\", \"ZZ\"])");
}

#[test]
fn test_dataframe_format_debug() {
    let values = vec![
        array![1, 2, 3, 4, 5],
        array![6.1, 7.1, 8.1, 9.1, 10.1],
        array![11, 12, 13, 14, 15],
    ];
    let df = DataFrame::from_vec(values, vec![10, 20, 30, 40, 50], vec!["X", "YYY", "ZZ"]);

    // better way?
    let mut buf = Vec::new();
    let _ = write!(&mut buf, "{:?}", df);
    let res = str::from_utf8(&buf).unwrap();
    assert_eq!(
        res,
        "   X  YYY ZZ
10 1  6.1 11
20 2  7.1 12
30 3  8.1 13
40 4  9.1 14
50 5 10.1 15"
    );
}
