use csv;
use std::hash::Hash;
use std::io::{Read, Write};
use std::string::ToString;

use nullvec::prelude::{Array, Scalar};

use frame::DataFrame;
use indexer::Indexer;
use traits::{Slicer, RowIndex};

fn default_columns(n: usize) -> Vec<String> {
    let columns: Vec<usize> = (0..n).collect();
    columns.into_iter().map(|x| x.to_string()).collect()
}

impl<'a, I, C> DataFrame<'a, 'a, 'a, I, C>
where
    I: Clone + Eq + Hash,
    C: Clone + Eq + Hash,
{
    pub fn read_csv<R: Read>(
        mut reader: csv::Reader<R>,
    ) -> Result<DataFrame<'a, 'a, 'a, usize, String>, csv::Error> {

        // headers read 1st row regardless of has_headers property
        let header: Vec<String> = try!(reader.headers());
        let columns: Vec<String> = if reader.has_headers {
            header
        } else {
            default_columns(header.len())
        };
        let ncols = columns.len();

        let mut records: Vec<Vec<Scalar>> = vec![];
        for record in reader.decode() {
            let values: Vec<Scalar> = try!(record);
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

        // ToDo: compare perf to disable boundary check / avoid clone
        // let mut arrays: Vec<Array> = Vec::with_capacity(ncols);
        // for i in 0..ncols {
        // let mut new_value: Vec<Scalar> = Vec::with_capacity(len);
        // for j in 0..len {
        // unsafe {
        // avoid index boundary check
        // new_value.push((*records.get_unchecked(j * ncols + i)).clone());
        // }
        // }
        // let array: Array = new_value.into();
        // arrays.push(array);
        // }
        //

        Ok(DataFrame::from_vec(arrays, index, columns))
    }
}

impl<'a, I, C> DataFrame<'a, 'a, 'a, I, C>
where
    I: Clone + Eq + Hash,
    C: Clone + Eq + Hash + ToString,
{
    pub fn write_csv<W: Write>(&self, writer: &mut csv::Writer<W>) -> Result<(), csv::Error> {

        // output columns
        let mut columns: Vec<Scalar> = Vec::with_capacity(self.values.len() + 1);
        // pad
        // columns.push(Scalar::String("".to_string()));
        for i in self.columns.values.iter() {
            let s: Scalar = i.to_string().into();
            columns.push(s);
        }
        try!(writer.encode(columns));

        for i in 0..self.len() {
            let mut row: Vec<Scalar> = Vec::with_capacity(self.values.len() + 1);

            // let s: Scalar = self.index.values[i].clone().into();
            // row.push(s);

            for col in self.values.iter() {
                row.push(col.iloc(&i));
            }
            try!(writer.encode(row));
        }
        Ok(())
    }
}
