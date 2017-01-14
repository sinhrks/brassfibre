use std::any::TypeId;

use nullvec::prelude::dev::algos::Indexing;
use traits::{Slicer, Append};

mod aggregation;
mod convert;
mod ops;
mod scalar;

/// Scalar
#[allow(non_camel_case_types)]
#[derive(RustcDecodable, RustcEncodable, Clone, PartialEq, Debug)]
pub enum Scalar {
    i64(i64),
    i32(i32),
    usize(usize),
    f64(f64),
    bool(bool),
    String(String),
}

/// Array
#[derive(Clone, PartialEq, Debug)]
pub enum Array {
    Int64Array(Vec<i64>),
    Int32Array(Vec<i32>),
    UsizeArray(Vec<usize>),
    Float64Array(Vec<f64>),
    BoolArray(Vec<bool>),
    StringArray(Vec<String>),
}

impl Array {
    pub fn new<T: Into<Array>>(values: T) -> Self {
        let arr: Array = values.into();
        arr
    }

    pub fn dtype(&self) -> String {
        match self {
            &Array::Int64Array(_) => "i64".to_string(),
            &Array::Int32Array(_) => "i32".to_string(),
            &Array::UsizeArray(_) => "usize".to_string(),
            &Array::Float64Array(_) => "f64".to_string(),
            &Array::BoolArray(_) => "bool".to_string(),
            &Array::StringArray(_) => "str".to_string(),
        }
    }

    pub fn is_numeric(&self) -> bool {
        match self {
            &Array::Int64Array(_) |
            &Array::Int32Array(_) |
            &Array::UsizeArray(_) |
            &Array::Float64Array(_) => true,
            _ => false,
        }
    }

    pub fn to_string_vec(&self) -> Vec<String> {
        match self {
            &Array::Int64Array(ref vals) => vals.iter().map(|x| x.to_string()).collect(),
            &Array::Int32Array(ref vals) => vals.iter().map(|x| x.to_string()).collect(),
            &Array::UsizeArray(ref vals) => vals.iter().map(|x| x.to_string()).collect(),
            &Array::Float64Array(ref vals) => vals.iter().map(|x| x.to_string()).collect(),
            &Array::BoolArray(ref vals) => vals.iter().map(|x| x.to_string()).collect(),
            &Array::StringArray(ref vals) => vals.iter().map(|x| x.to_string()).collect(),
        }
    }

    pub fn astype<V: 'static>(&self) -> Array {
        let typ = TypeId::of::<V>();

        let arr: Array;
        if typ == TypeId::of::<i64>() {
            arr = match self {
                &Array::Int64Array(ref vals) => vals.clone().into(),
                &Array::Int32Array(ref vals) => {
                    let new_vals: Vec<i64> = vals.iter()
                        .map(|&x| x as i64)
                        .collect();
                    new_vals.into()
                }
                &Array::UsizeArray(ref vals) => {
                    let new_vals: Vec<i64> = vals.iter()
                        .map(|&x| x as i64)
                        .collect();
                    new_vals.into()
                }
                &Array::Float64Array(ref vals) => {
                    let new_vals: Vec<i64> = vals.iter()
                        .map(|&x| x as i64)
                        .collect();
                    new_vals.into()
                }
                // ToDo: parse str
                _ => panic!("unablet to convert to specified type"),
            };
        } else if typ == TypeId::of::<i32>() {
            arr = match self {
                &Array::Int64Array(ref vals) => {
                    let new_vals: Vec<i32> = vals.iter()
                        .map(|&x| x as i32)
                        .collect();
                    new_vals.into()
                }
                &Array::Int32Array(ref vals) => vals.clone().into(),
                &Array::UsizeArray(ref vals) => {
                    let new_vals: Vec<i32> = vals.iter()
                        .map(|&x| x as i32)
                        .collect();
                    new_vals.into()
                }
                &Array::Float64Array(ref vals) => {
                    let new_vals: Vec<i32> = vals.iter()
                        .map(|&x| x as i32)
                        .collect();
                    new_vals.into()
                }
                // ToDo: parse str
                _ => panic!("unablet to convert to specified type"),
            };
        } else if typ == TypeId::of::<usize>() {
            arr = match self {
                &Array::Int64Array(ref vals) => {
                    let new_vals: Vec<usize> = vals.iter()
                        .map(|&x| x as usize)
                        .collect();
                    new_vals.into()
                }
                &Array::Int32Array(ref vals) => {
                    let new_vals: Vec<usize> = vals.iter()
                        .map(|&x| x as usize)
                        .collect();
                    new_vals.into()
                }
                &Array::UsizeArray(ref vals) => vals.clone().into(),
                &Array::Float64Array(ref vals) => {
                    let new_vals: Vec<usize> = vals.iter()
                        .map(|&x| x as usize)
                        .collect();
                    new_vals.into()
                }
                // ToDo: parse str
                _ => panic!("unablet to convert to specified type"),
            };
        } else if typ == TypeId::of::<f64>() {
            arr = match self {
                &Array::Int64Array(ref vals) => {
                    let new_vals: Vec<f64> = vals.iter()
                        .map(|&x| x as f64)
                        .collect();
                    new_vals.into()
                }
                &Array::Int32Array(ref vals) => {
                    let new_vals: Vec<f64> = vals.iter()
                        .map(|&x| x as f64)
                        .collect();
                    new_vals.into()
                }
                &Array::UsizeArray(ref vals) => {
                    let new_vals: Vec<f64> = vals.iter()
                        .map(|&x| x as f64)
                        .collect();
                    new_vals.into()
                }
                &Array::Float64Array(ref vals) => vals.clone().into(),
                // ToDo: parse str
                _ => panic!("unablet to convert to specified type"),
            };
        } else {
            // ToDo: support more types
            panic!("unablet to convert to specified type");
        }
        arr
    }
}

impl Slicer for Array {
    type Scalar = Scalar;

    fn len(&self) -> usize {
        match self {
            &Array::Int64Array(ref vals) => vals.len(),
            &Array::Int32Array(ref vals) => vals.len(),
            &Array::UsizeArray(ref vals) => vals.len(),
            &Array::Float64Array(ref vals) => vals.len(),
            &Array::BoolArray(ref vals) => vals.len(),
            &Array::StringArray(ref vals) => vals.len(),
        }
    }

    fn iloc(&self, location: &usize) -> Self::Scalar {
        match self {
            &Array::Int64Array(ref vals) => Scalar::i64(vals[*location]),
            &Array::Int32Array(ref vals) => Scalar::i32(vals[*location]),
            &Array::UsizeArray(ref vals) => Scalar::usize(vals[*location]),
            &Array::Float64Array(ref vals) => Scalar::f64(vals[*location]),
            &Array::BoolArray(ref vals) => Scalar::bool(vals[*location]),
            &Array::StringArray(ref vals) => Scalar::String(vals[*location].clone()),
        }
    }

    unsafe fn iloc_unchecked(&self, location: &usize) -> Self::Scalar {
        unimplemented!()
    }

    fn ilocs(&self, locations: &[usize]) -> Self {
        match self {
            &Array::Int64Array(ref vals) => Indexing::reindex(vals, locations).into(),
            &Array::Int32Array(ref vals) => Indexing::reindex(vals, locations).into(),
            &Array::UsizeArray(ref vals) => Indexing::reindex(vals, locations).into(),
            &Array::Float64Array(ref vals) => Indexing::reindex(vals, locations).into(),
            &Array::BoolArray(ref vals) => Indexing::reindex(vals, locations).into(),
            &Array::StringArray(ref vals) => Indexing::reindex(vals, locations).into(),
        }
    }

    unsafe fn ilocs_unchecked(&self, locations: &[usize]) -> Self {
        match self {
            &Array::Int64Array(ref vals) => {
                Array::Int64Array(Indexing::reindex_unchecked(vals, locations))
            }
            &Array::Int32Array(ref vals) => {
                Array::Int32Array(Indexing::reindex_unchecked(vals, locations))
            }
            &Array::UsizeArray(ref vals) => {
                Array::UsizeArray(Indexing::reindex_unchecked(vals, locations))
            }
            &Array::Float64Array(ref vals) => {
                Array::Float64Array(Indexing::reindex_unchecked(vals, locations))
            }
            &Array::BoolArray(ref vals) => {
                Array::BoolArray(Indexing::reindex_unchecked(vals, locations))
            }
            &Array::StringArray(ref vals) => {
                Array::StringArray(Indexing::reindex_unchecked(vals, locations))
            }
        }
    }

    fn ilocs_forced(&self, locations: &[usize]) -> Self {
        unimplemented!()
    }

    fn blocs(&self, flags: &[bool]) -> Self {
        match self {
            &Array::Int64Array(ref vals) => Indexing::blocs(vals, flags).into(),
            &Array::Int32Array(ref vals) => Indexing::blocs(vals, flags).into(),
            &Array::UsizeArray(ref vals) => Indexing::blocs(vals, flags).into(),
            &Array::Float64Array(ref vals) => Indexing::blocs(vals, flags).into(),
            &Array::BoolArray(ref vals) => Indexing::blocs(vals, flags).into(),
            &Array::StringArray(ref vals) => Indexing::blocs(vals, flags).into(),
        }
    }
}

impl<'a> Append<'a> for Array {
    fn append(&self, other: &Self) -> Self {
        match (self, other) {
            (&Array::Int64Array(ref svals), &Array::Int64Array(ref ovals)) => {
                let mut new_values = svals.clone();
                new_values.append(&mut ovals.clone());
                new_values.into()
            }
            (&Array::Int32Array(ref svals), &Array::Int32Array(ref ovals)) => {
                let mut new_values = svals.clone();
                new_values.append(&mut ovals.clone());
                new_values.into()
            }
            (&Array::UsizeArray(ref svals), &Array::UsizeArray(ref ovals)) => {
                let mut new_values = svals.clone();
                new_values.append(&mut ovals.clone());
                new_values.into()
            }
            (&Array::Float64Array(ref svals), &Array::Float64Array(ref ovals)) => {
                let mut new_values = svals.clone();
                new_values.append(&mut ovals.clone());
                new_values.into()
            }
            (&Array::BoolArray(ref svals), &Array::BoolArray(ref ovals)) => {
                let mut new_values = svals.clone();
                new_values.append(&mut ovals.clone());
                new_values.into()
            }
            (&Array::StringArray(ref svals), &Array::StringArray(ref ovals)) => {
                let mut new_values = svals.clone();
                new_values.append(&mut ovals.clone());
                new_values.into()
            }
            _ => panic!("Unable to append different dtype"),
        }
    }
}
