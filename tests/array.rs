#[macro_use]
extern crate brassfibre;
use brassfibre::*;

#[test]
fn test_creation() {
    let iarr = Array::Int64Array(vec![1, 2, 3]);
    assert_eq!(iarr.dtype(), "i64");
    assert_eq!(iarr.len(), 3);

    let farr = Array::Float64Array(vec![1.1, 2.1, 3.1, 4.1]);
    assert_eq!(farr.dtype(), "f64");
    assert_eq!(farr.len(), 4);

    let barr = Array::BoolArray(vec![true, false, true]);
    assert_eq!(barr.dtype(), "bool");
    assert_eq!(barr.len(), 3);

    let sarr = Array::StringArray(vec!["a".to_string(), "b".to_string()]);
    assert_eq!(sarr.dtype(), "str");
    assert_eq!(sarr.len(), 2);
}

#[test]
fn test_creation_new() {
    let iarr = Array::new(vec![1, 2, 3]);
    assert_eq!(iarr.dtype(), "i64");
    assert_eq!(iarr.len(), 3);

    let farr = Array::new(vec![1.1, 2.1, 3.1, 4.1]);
    assert_eq!(farr.dtype(), "f64");
    assert_eq!(farr.len(), 4);

    let barr = Array::new(vec![true, false, true]);
    assert_eq!(barr.dtype(), "bool");
    assert_eq!(barr.len(), 3);

    let sarr = Array::new(vec!["a".to_string(), "b".to_string()]);
    assert_eq!(sarr.dtype(), "str");
    assert_eq!(sarr.len(), 2);

    // &str
    let sarr2 = Array::new(vec!["a", "b"]);
    assert_eq!(sarr2.dtype(), "str");
    assert_eq!(sarr2.len(), 2);
}

#[test]
fn test_creation_macros() {
    let iarr = array![1, 2, 3];
    assert_eq!(iarr.dtype(), "i64");
    assert_eq!(iarr.len(), 3);

    let farr = array![1.1, 2.1, 3.1, 4.1];
    assert_eq!(farr.dtype(), "f64");
    assert_eq!(farr.len(), 4);

    let barr = array![true, false, true];
    assert_eq!(barr.dtype(), "bool");
    assert_eq!(barr.len(), 3);

    let str_arr = array!["A", "B", "C"];
    assert_eq!(str_arr.dtype(), "str");
    assert_eq!(str_arr.len(), 3);

    let string_arr = array!["A".to_string(), "B".to_string(), "C".to_string()];
    assert_eq!(string_arr.dtype(), "str");
    assert_eq!(string_arr.len(), 3);
}

#[test]
fn test_dtype_property() {
    let iarr = Array::Int64Array(vec![1, 2, 3]);
    assert_eq!(iarr.dtype(), "i64");
    assert_eq!(iarr.is_numeric(), true);

    let farr = Array::Float64Array(vec![1.1, 2.1, 3.1, 4.1]);
    assert_eq!(farr.dtype(), "f64");
    assert_eq!(farr.is_numeric(), true);

    let barr = Array::BoolArray(vec![true, false, true]);
    assert_eq!(barr.dtype(), "bool");
    assert_eq!(barr.is_numeric(), false);

    let sarr = Array::StringArray(vec!["a".to_string(), "b".to_string()]);
    assert_eq!(sarr.dtype(), "str");
    assert_eq!(sarr.is_numeric(), false);
}

#[test]
fn test_eq() {
    let iarr1 = Array::Int64Array(vec![1, 2, 3]);
    let iarr2 = Array::Int64Array(vec![2, 3, 4]);
    let iarr3 = Array::Int64Array(vec![1, 2, 3, 4, 5]);
    let iarr4 = Array::Int64Array(vec![1, 2, 3]);
    assert_eq!(iarr1, iarr1);
    assert_eq!(iarr1 == iarr2, false);
    assert_eq!(iarr1 == iarr3, false);
    assert_eq!(iarr1, iarr4);

    let farr1 = Array::Float64Array(vec![1., 2., 3.]);
    let farr2 = Array::Float64Array(vec![2., 3., 4.]);
    let farr3 = Array::Float64Array(vec![1., 2., 3., 4., 5.]);
    let farr4 = Array::Float64Array(vec![1., 2., 3.]);
    assert_eq!(farr1, farr1);
    assert_eq!(farr1 == farr2, false);
    assert_eq!(farr1 == farr3, false);
    assert_eq!(farr1, farr4);

    // different types
    assert_eq!(iarr1 == farr1, false);
    assert_eq!(iarr2 == farr2, false);
    assert_eq!(iarr3 == farr3, false);
    assert_eq!(iarr4 == farr4, false);
}

#[test]
fn test_clone() {
    let iarr1 = Array::Int64Array(vec![1, 2, 3]);
    assert_eq!(iarr1, iarr1.clone());
}

#[test]
fn test_astype() {
    let iarr = Array::Int64Array(vec![1, 2, 3]);
    assert_eq!(iarr.astype::<f64>(), Array::Float64Array(vec![1., 2., 3.]));
    assert_eq!(iarr.astype::<i64>(), Array::Int64Array(vec![1, 2, 3]));

    let farr = Array::Float64Array(vec![1.1, 2.1, 3.1]);
    assert_eq!(farr.astype::<f64>(),
               Array::Float64Array(vec![1.1, 2.1, 3.1]));
    assert_eq!(farr.astype::<i64>(), Array::Int64Array(vec![1, 2, 3]));
}


#[test]
fn test_ilocs() {
    let iarr = Array::Int64Array(vec![1, 2, 3, 4, 5]);
    assert_eq!(iarr.dtype(), "i64");
    let ires = iarr.ilocs(&vec![1, 4, 0]);
    assert_eq!(ires, Array::Int64Array(vec![2, 5, 1]));

    let farr = Array::Float64Array(vec![1.1, 2.1, 3.1, 4.1, 5.1]);
    assert_eq!(farr.dtype(), "f64");
    let fres = farr.ilocs(&vec![1, 4, 0]);
    assert_eq!(fres, Array::Float64Array(vec![2.1, 5.1, 1.1]));

    let barr = Array::BoolArray(vec![true, false, true, true]);
    let bres = barr.ilocs(&vec![1, 2]);
    assert_eq!(bres, Array::BoolArray(vec![false, true]));

    let sarr = Array::StringArray(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    let sres = sarr.ilocs(&vec![2, 0]);
    assert_eq!(sres,
               Array::StringArray(vec!["c".to_string(), "a".to_string()]));
}

#[test]
#[should_panic]
fn test_ilocs_panic() {
    let iarr = Array::Int64Array(vec![1, 2, 3, 4, 5]);
    iarr.ilocs(&vec![1, 5, 0]);
}

#[test]
fn test_blocs() {
    let iarr = Array::Int64Array(vec![1, 2, 3]);
    assert_eq!(iarr.dtype(), "i64");
    let ires = iarr.blocs(&vec![true, false, true]);
    assert_eq!(ires, Array::Int64Array(vec![1, 3]));

    let farr = Array::Float64Array(vec![1.1, 2.1, 3.1]);
    assert_eq!(farr.dtype(), "f64");
    let fres = farr.blocs(&vec![true, false, true]);
    assert_eq!(fres, Array::Float64Array(vec![1.1, 3.1]));

    let barr = Array::BoolArray(vec![true, false, true]);
    let bres = barr.blocs(&vec![true, false, true]);
    assert_eq!(bres, Array::BoolArray(vec![true, true]));

    let sarr = Array::StringArray(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    let sres = sarr.blocs(&vec![true, false, true]);
    assert_eq!(sres,
               Array::StringArray(vec!["a".to_string(), "c".to_string()]));
}

#[test]
fn test_container() {
    let iarr: Array = vec![1, 2, 3].into();
    let farr: Array = vec![1.1, 2.1, 3.1].into();
    let barr: Array = vec![true, false, true].into();
    let sarr: Array = vec!["a", "b", "c"].into();
    assert_eq!(iarr.dtype(), "i64");
    assert_eq!(farr.dtype(), "f64");
    assert_eq!(barr.dtype(), "bool");
    assert_eq!(sarr.dtype(), "str");

    let container: Vec<Array> = vec![iarr, farr, barr, sarr];
    assert_eq!(container.len(), 4);
    let dtypes: Vec<String> = container.iter().map(|ref x| x.dtype()).collect();
    assert_eq!(dtypes, vec!["i64", "f64", "bool", "str"]);
}

#[test]
fn test_append() {
    let iarr1 = Array::Int64Array(vec![1, 2, 3]);
    assert_eq!(iarr1.dtype(), "i64");

    let iarr2 = Array::Int64Array(vec![1, 2, 3]);
    assert_eq!(iarr2.dtype(), "i64");

    let res = iarr1.append(&iarr2);
    assert_eq!(res.dtype(), "i64");

    let exp = Array::Int64Array(vec![1, 2, 3, 1, 2, 3]);
    assert_eq!(res, exp);
}
