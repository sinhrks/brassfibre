use super::algos::sort::Sorter;

#[derive(Clone, PartialEq, Debug)]
pub enum Array {
    Int64Array(Vec<i64>),
    Float64Array(Vec<f64>)
}

// ToDo: define macro
macro_rules! add_conversion {
    ($t:ident, $klass:ident) => {

        impl From<Vec<$t>> for Array {
            fn from(values: Vec<$t>) -> Self {
                Array::$klass(values)
            }
        }

        impl Into<Vec<$t>> for Array {
            fn into(self) -> Vec<$t> {
                match self {
                    Array::$klass(vals) => vals,
                    _ => panic!("Unable to convert to Vec")
                }
            }
        }
    }
}
add_conversion!(i64, Int64Array);
add_conversion!(f64, Float64Array);


impl Array {

    pub fn new<T: Into<Array>>(values: T) -> Self {
        let arr: Array = values.into();
        arr
    }

    pub fn dtype(&self) -> String {
        match self {
            &Array::Int64Array(_) => "i64".to_string(),
            &Array::Float64Array(_) => "f64".to_string(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            &Array::Int64Array(ref vals) => vals.len(),
            &Array::Float64Array(ref vals) => vals.len(),
        }
    }

    pub fn ilocs(&self, locations: &Vec<usize>) -> Self {
        match self {
            &Array::Int64Array(ref vals) => {
                Array::Int64Array(Sorter::reindex(vals, locations))
            },
            &Array::Float64Array(ref vals) => {
                Array::Float64Array(Sorter::reindex(vals, locations))
            }
        }
    }
}