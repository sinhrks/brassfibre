use super::{Array, Scalar};

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
add_conversion!(bool, BoolArray);
add_conversion!(String, StringArray);

// &str handling
impl<'a> From<Vec<&'a str>> for Array {
    fn from(values: Vec<&'a str>) -> Self {
        let new_values: Vec<String> = values.iter().map(|&x| String::from(x)).collect();
        Array::StringArray(new_values)
    }
}


////////////////////////////////////////////////////////////////////////////////
// Scalar to Array
////////////////////////////////////////////////////////////////////////////////


impl From<Vec<Scalar>> for Array {
    fn from(values: Vec<Scalar>) -> Self {
        assert!(values.len() > 0, "Unable to infer dtype");

        match &values[0] {
            &Scalar::i64(_) => Array::Int64Array(values.iter().map(|ref x| x.as_i64()).collect()),
            &Scalar::f64(_) => Array::Float64Array(values.iter().map(|ref x| x.as_f64()).collect()),
            &Scalar::bool(_) => Array::BoolArray(values.iter().map(|ref x| x.as_bool()).collect()),
            &Scalar::String(_) => Array::StringArray(values.iter().map(|ref x| x.as_str()).collect())
        }
    }
}

impl Into<Vec<Scalar>> for Array {
    fn into(self) -> Vec<Scalar> {
        match self {
            Array::Int64Array(vals) => vals.into_iter().map(|x| Scalar::i64(x)).collect(),
            Array::Float64Array(vals) => vals.into_iter().map(|x| Scalar::f64(x)).collect(),
            Array::BoolArray(vals) => vals.into_iter().map(|x| Scalar::bool(x)).collect(),
            Array::StringArray(vals) => vals.into_iter().map(|x| Scalar::String(x)).collect(),
        }
    }
}


#[cfg(test)]
mod tests {

    use super::super::{Array, Scalar};

    #[test]
    #[should_panic]
    fn test_empty_scalar_to_array() {
        let vals: Vec<Scalar> = vec![];
        let arr: Array = vals.into();
    }

    #[test]
    fn test_i64_scalar_to_array() {
        let vals: Vec<Scalar> = vec![Scalar::i64(1), Scalar::i64(2)];
        let arr: Array = vals.into();
        assert_eq!(arr, Array::Int64Array(vec![1, 2]));
    }

    #[test]
    fn test_f64_scalar_to_array() {
        let vals: Vec<Scalar> = vec![Scalar::f64(1.1), Scalar::f64(2.2)];
        let arr: Array = vals.into();
        assert_eq!(arr, Array::Float64Array(vec![1.1, 2.2]));
    }

    #[test]
    fn test_bool_scalar_to_array() {
        let vals: Vec<Scalar> = vec![Scalar::bool(true), Scalar::bool(false)];
        let arr: Array = vals.into();
        assert_eq!(arr, Array::BoolArray(vec![true, false]));
    }

    #[test]
    fn test_str_scalar_to_array() {
        let vals: Vec<Scalar> = vec![Scalar::String("a".to_string()), Scalar::String("b".to_string())];
        let arr: Array = vals.into();
        assert_eq!(arr, Array::StringArray(vec!["a".to_string(), "b".to_string()]));
    }
}