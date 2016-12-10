use super::algos::indexing::Indexing;
use super::algos::sort::Sorter;
use super::traits::{Slicer, Append};

mod aggregation;
mod convert;
mod ops;
mod scalar;

////////////////////////////////////////////////////////////////////////////////
// Scalar
////////////////////////////////////////////////////////////////////////////////

#[allow(non_camel_case_types)]
#[derive(RustcDecodable, Clone, PartialEq, Debug)]
pub enum Scalar {
    i64(i64),
    f64(f64),
    bool(bool),
    String(String)
}

////////////////////////////////////////////////////////////////////////////////
// Array
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Debug)]
pub enum Array {
    Int64Array(Vec<i64>),
    Float64Array(Vec<f64>),
    BoolArray(Vec<bool>),
    StringArray(Vec<String>)
}

impl Array {

    pub fn new<T: Into<Array>>(values: T) -> Self {
        let arr: Array = values.into();
        arr
    }

    pub fn dtype(&self) -> String {
        match self {
            &Array::Int64Array(_) => "i64".to_string(),
            &Array::Float64Array(_) => "f64".to_string(),
            &Array::BoolArray(_) => "bool".to_string(),
            &Array::StringArray(_) => "str".to_string(),
        }
    }

    pub fn is_numeric(&self) -> bool {
        match self {
            &Array::Int64Array(_) | &Array::Float64Array(_) => true,
            _ => false,
        }
    }

    pub fn to_string_vec(&self) -> Vec<String> {
        match self {
            &Array::Int64Array(ref vals) => {
                vals.iter().map(|x| x.to_string()).collect()
            },
            &Array::Float64Array(ref vals) => {
                vals.iter().map(|x| x.to_string()).collect()
            },
            &Array::BoolArray(ref vals) => {
                vals.iter().map(|x| x.to_string()).collect()
            },
            &Array::StringArray(ref vals) => {
                vals.iter().map(|x| x.to_string()).collect()
            }
        }
    }
}

impl Slicer for Array {

    fn len(&self) -> usize {
        match self {
            &Array::Int64Array(ref vals) => vals.len(),
            &Array::Float64Array(ref vals) => vals.len(),
            &Array::BoolArray(ref vals) => vals.len(),
            &Array::StringArray(ref vals) => vals.len(),
        }
    }

    fn ilocs(&self, locations: &[usize]) -> Self {
        match self {
            &Array::Int64Array(ref vals) => {
                Array::Int64Array(Sorter::reindex(vals, locations))
            },
            &Array::Float64Array(ref vals) => {
                Array::Float64Array(Sorter::reindex(vals, locations))
            },
            &Array::BoolArray(ref vals) => {
                Array::BoolArray(Sorter::reindex(vals, locations))
            },
            &Array::StringArray(ref vals) => {
                Array::StringArray(Sorter::reindex(vals, locations))
            }
        }
    }

    fn blocs(&self, flags: &[bool]) -> Self {
        match self {
            &Array::Int64Array(ref vals) => {
                Array::Int64Array(Indexing::blocs(vals, flags))
            },
            &Array::Float64Array(ref vals) => {
                Array::Float64Array(Indexing::blocs(vals, flags))
            },
            &Array::BoolArray(ref vals) => {
                Array::BoolArray(Indexing::blocs(vals, flags))
            },
            &Array::StringArray(ref vals) => {
                Array::StringArray(Indexing::blocs(vals, flags))
            }
        }
    }
}

impl<'a> Append<'a> for Array {

    fn append(&self, other: &Self) -> Self {
        match (self, other) {
            (&Array::Int64Array(ref svals), &Array::Int64Array(ref ovals)) => {
                let mut new_values = svals.clone();
                new_values.append(&mut ovals.clone());
                Array::Int64Array(new_values)
            },
            (&Array::Float64Array(ref svals), &Array::Float64Array(ref ovals)) => {
                let mut new_values = svals.clone();
                new_values.append(&mut ovals.clone());
                Array::Float64Array(new_values)
            },
            (&Array::BoolArray(ref svals), &Array::BoolArray(ref ovals)) => {
                let mut new_values = svals.clone();
                new_values.append(&mut ovals.clone());
                Array::BoolArray(new_values)
            },
            (&Array::StringArray(ref svals), &Array::StringArray(ref ovals)) => {
                let mut new_values = svals.clone();
                new_values.append(&mut ovals.clone());
                Array::StringArray(new_values)
            },
            _ => panic!("Unable to append different dtype")
        }
    }
}