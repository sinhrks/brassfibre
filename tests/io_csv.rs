#[macro_use]
extern crate brassfibre;
use brassfibre::prelude::*;

extern crate csv;

#[test]
fn test_read_csv_no_header() {
    let data = "x,true,7,1.1
y,false,3,2.2
z,true,1,4.5";

    let rdr = csv::Reader::from_string(data).has_headers(false);
    let res = DataFrame::<usize, String>::read_csv(rdr).unwrap();

    let exp_dtypes: Vec<String> =
        vec!["str".to_string(), "bool".to_string(), "i64".to_string(), "f64".to_string()];
    assert_eq!(res.dtypes(), exp_dtypes);

    let exp_values = vec![array!["x".to_string(), "y".to_string(), "z".to_string()],
                          array![true, false, true],
                          array![7i64, 3, 1],
                          array![1.1, 2.2, 4.5]];
    let exp = DataFrame::from_vec(exp_values,
                                  vec![0, 1, 2],
                                  vec!["0".to_string(),
                                       "1".to_string(),
                                       "2".to_string(),
                                       "3".to_string()]);

    assert_eq!(res, exp);
}

#[test]
fn test_read_csv_with_header() {
    let data = "A,B,C,D
x,true,7,1.1
y,false,3,2.2
z,true,1,4.5";

    let rdr = csv::Reader::from_string(data).has_headers(true);
    let res = DataFrame::<usize, String>::read_csv(rdr).unwrap();

    let exp_dtypes: Vec<String> =
        vec!["str".to_string(), "bool".to_string(), "i64".to_string(), "f64".to_string()];
    assert_eq!(res.dtypes(), exp_dtypes);

    let exp_values = vec![array!["x".to_string(), "y".to_string(), "z".to_string()],
                          array![true, false, true],
                          array![7i64, 3, 1],
                          array![1.1, 2.2, 4.5]];
    let exp = DataFrame::from_vec(exp_values,
                                  vec![0, 1, 2],
                                  vec!["A".to_string(),
                                       "B".to_string(),
                                       "C".to_string(),
                                       "D".to_string()]);

    assert_eq!(res, exp);
}

#[test]
fn test_empty() {
    let data = "";

    let rdr = csv::Reader::from_string(data).has_headers(false);
    let res = DataFrame::<usize, String>::read_csv(rdr).unwrap();

    let exp_dtypes: Vec<String> = vec![];
    assert_eq!(res.dtypes(), exp_dtypes);

    let exp_values: Vec<Array> = vec![];
    let exp = DataFrame::from_vec(exp_values, vec![], vec![]);
    assert_eq!(res, exp);
}

#[test]
fn test_read_csv_error_different_items() {
    let data = "A,B,C
1,7,1.1
1,3
1,1,4.5";
    let rdr = csv::Reader::from_string(data).has_headers(true);
    let res = DataFrame::<usize, String>::read_csv(rdr);
    assert!(res.is_err())
}

#[test]
fn test_write_csv() {

    let values = vec![array!["x".to_string(), "y".to_string(), "z".to_string()],
                      array![true, false, true],
                      array![7i64, 3, 1],
                      array![1.1, 2.2, 4.5]];
    let df = DataFrame::from_vec(values,
                                 vec![0, 1, 2],
                                 vec!["A".to_string(),
                                      "B".to_string(),
                                      "C".to_string(),
                                      "D".to_string()]);

    let mut wtr = csv::Writer::from_memory();
    df.write_csv(&mut wtr).unwrap();
    let res = wtr.as_string();
    assert_eq!(res, "A,B,C,D\nx,true,7,1.1\ny,false,3,2.2\nz,true,1,4.5\n");

    // test round-trip
    let rdr = csv::Reader::from_string(res).has_headers(true);
    let res = DataFrame::<usize, String>::read_csv(rdr).unwrap();
    assert_eq!(res, df);
}

#[test]
fn test_file_io() {
    let values = vec![array!["x".to_string(), "y".to_string(), "z".to_string()],
                      array![true, false, true],
                      array![7i64, 3, 1],
                      array![1.1, 2.2, 4.5]];
    let df = DataFrame::from_vec(values,
                                 vec![0, 1, 2],
                                 vec!["A".to_string(),
                                      "B".to_string(),
                                      "C".to_string(),
                                      "D".to_string()]);
    let mut wtr = csv::Writer::from_file("./data.csv").unwrap();
    df.write_csv(&mut wtr).unwrap();
    wtr.flush().unwrap();

    let rdr = csv::Reader::from_file("./data.csv").unwrap().has_headers(true);
    let res = DataFrame::<usize, String>::read_csv(rdr).unwrap();
    println!("{:?}", df.dtypes());
    println!("{:?}", res.dtypes());
    assert_eq!(res, df);

    // remove file;
    use std::fs;
    fs::remove_file("./data.csv").unwrap();
}
