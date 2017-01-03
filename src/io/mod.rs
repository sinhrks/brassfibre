use csv::Reader;
use std::io::Read;

use frame::DataFrame;
use indexer::Indexer;
use internals::{Array, Scalar};

fn default_columns(n: usize) -> Vec<String> {
    let columns: Vec<usize> = (0..n).collect();
    columns.into_iter().map(|x| x.to_string()).collect()
}

pub fn read_csv<'a, R: Read>(mut reader: Reader<R>) -> DataFrame<'a, 'a, 'a, usize, String> {

    // headers read 1st row regardless of has_headers property
    let header : Vec<String> = reader.headers().unwrap();
    let columns: Vec<String> = if reader.has_headers {
        header
    } else {
        default_columns(header.len())
    };
    let ncols = columns.len();

    let mut records: Vec<Vec<Scalar>> = vec![];
    for record in reader.decode() {
        let values: Vec<Scalar> = record.unwrap();
        assert!(ncols == values.len(), "column length mismatch!");
        records.push(values);
    }

    let index: Indexer<usize> = Indexer::<usize>::from_len(records.len());

    // column-wise vec of scalar
    let mut colvecs: Vec<Vec<Scalar>> = Vec::with_capacity(ncols);
    for _ in 0..columns.len() {
        colvecs.push(Vec::with_capacity(records.len()));
    }
    for record in records {
        for (column, val) in colvecs.iter_mut().zip(record) {
            column.push(val);
        }
    }
    // column-wise vec of Array
    let mut arrays: Vec<Array> = Vec::with_capacity(ncols);
    for column in colvecs {
        let array: Array = column.into();
        arrays.push(array);
    }

    /*
    ToDo: compare perf to disable boundary check / avoid clone
    let mut arrays: Vec<Array> = Vec::with_capacity(ncols);
    for i in 0..ncols {
        let mut new_value: Vec<Scalar> = Vec::with_capacity(len);
        for j in 0..len {
            unsafe {
                // avoid index boundary check
                new_value.push((*records.get_unchecked(j * ncols + i)).clone());
            }
        }
        let array: Array = new_value.into();
        arrays.push(array);
    }
    */

    DataFrame::from_vec(arrays, index, columns)
}
