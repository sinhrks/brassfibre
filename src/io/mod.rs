use csv::Reader;
use std::io::Read;

use super::frame::DataFrame;
use super::indexer::Indexer;
use super::internals::{Array, Scalar};

fn default_columns(n: usize) -> Vec<String> {
    let columns: Vec<usize> = (0..n).collect();
    columns.into_iter().map(|x| x.to_string()).collect()
}

pub fn read_csv<'a, R: Read>(mut reader: Reader<R>) -> DataFrame<'a, 'a, usize, String> {

    let mut records: Vec<Vec<Scalar>> = vec![];
    for record in reader.decode() {
        let values: Vec<Scalar> = record.unwrap();
        records.push(values);
    }

    assert!(records.len() > 0, "input is empty!");

    // headers read 1st row regardless of has_headers property
    let columns: Vec<String> = if reader.has_headers {
        reader.headers().unwrap()
    } else {
        default_columns(records[0].len())
    };

    let index: Indexer<usize> = Indexer::<usize>::from_len(records.len());

    // column-wise vec of scalar
    let mut colvecs: Vec<Vec<Scalar>> = Vec::with_capacity(columns.len());
    for _ in 0..columns.len() {
        colvecs.push(Vec::with_capacity(records.len()));
    }
    for record in records {
        for (column, val) in colvecs.iter_mut().zip(record) {
            column.push(val);
        }
    }
    // column-wise vec of Array
    let mut arrays: Vec<Array> = Vec::with_capacity(columns.len());
    for column in colvecs {
        let array: Array = column.into();
        arrays.push(array);
    }
    DataFrame::from_vec(arrays, index, columns)
}