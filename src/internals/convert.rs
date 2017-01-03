use super::{Array, Scalar};

/// /////////////////////////////////////////////////////////////////////////////
/// Vec to Array
/// /////////////////////////////////////////////////////////////////////////////

macro_rules! add_array_conversion {
    ($t:ident, $klass:ident) => {

        impl From<Vec<$t>> for Array {
            fn from(values: Vec<$t>) -> Self {
                Array::$klass(values)
            }
        }

        impl From<Array> for Vec<$t> {
            fn from(values: Array) -> Self {
                match values {
                    Array::$klass(vals) => vals,
                    _ => panic!("Unable to convert to Vec")
                }
            }
        }
    }
}
add_array_conversion!(i64, Int64Array);
add_array_conversion!(f64, Float64Array);
add_array_conversion!(bool, BoolArray);
add_array_conversion!(String, StringArray);

// &str handling
impl<'a> From<Vec<&'a str>> for Array {
    fn from(values: Vec<&'a str>) -> Self {
        let new_values: Vec<String> = values.iter().map(|&x| String::from(x)).collect();
        Array::StringArray(new_values)
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Scalar Vec to Array
/// /////////////////////////////////////////////////////////////////////////////

impl From<Vec<Scalar>> for Array {
    fn from(values: Vec<Scalar>) -> Self {
        assert!(values.len() > 0, "Unable to infer dtype");

        match &values[0] {
            &Scalar::i64(_) => Array::Int64Array(values.iter().map(|ref x| x.as_i64()).collect()),
            &Scalar::f64(_) => Array::Float64Array(values.iter().map(|ref x| x.as_f64()).collect()),
            &Scalar::bool(_) => Array::BoolArray(values.iter().map(|ref x| x.as_bool()).collect()),
            &Scalar::String(_) => {
                Array::StringArray(values.iter().map(|ref x| x.as_str()).collect())
            }
            // ToDo: impl usize
            _ => panic!(""),
        }
    }
}

impl From<Array> for Vec<Scalar> {
    fn from(values: Array) -> Self {
        match values {
            Array::Int64Array(vals) => vals.into_iter().map(|x| Scalar::i64(x)).collect(),
            Array::Float64Array(vals) => vals.into_iter().map(|x| Scalar::f64(x)).collect(),
            Array::BoolArray(vals) => vals.into_iter().map(|x| Scalar::bool(x)).collect(),
            Array::StringArray(vals) => vals.into_iter().map(|x| Scalar::String(x)).collect(),
        }
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Scalar to primitives
/// /////////////////////////////////////////////////////////////////////////////

macro_rules! add_scalar_conversion {
    ($t:ident) => {

        impl From<$t> for Scalar {
            fn from(value: $t) -> Self {
                Scalar::$t(value)
            }
        }

        impl From<Scalar> for $t {
            fn from(value: Scalar) -> Self {
                match value {
                    Scalar::$t(vals) => vals,
                    _ => panic!("Unable to convert to primitive")
                }
            }
        }
    }
}
add_scalar_conversion!(i64);
add_scalar_conversion!(usize);
add_scalar_conversion!(f64);
add_scalar_conversion!(bool);
add_scalar_conversion!(String);

// &str handling
impl<'a> From<&'a str> for Scalar {
    fn from(value: &'a str) -> Self {
        Scalar::String(value.to_string())
    }
}

#[cfg(test)]
mod tests {

    use super::super::{Array, Scalar};

    #[test]
    #[should_panic]
    fn test_empty_scalar_to_array() {
        let vals: Vec<Scalar> = vec![];
        let _: Array = vals.into();
    }

    #[test]
    fn test_i64_vec_to_array() {
        let exp: Array = Array::Int64Array(vec![1, 2]);

        // Into
        let vals: Vec<i64> = vec![1, 2];
        let res: Array = vals.into();
        assert_eq!(res, exp);

        let vals: Vec<Scalar> = vec![Scalar::i64(1), Scalar::i64(2)];
        let res: Array = vals.into();
        assert_eq!(res, exp);

        // From
        let vals: Vec<i64> = vec![1, 2];
        let res = Array::from(vals);
        assert_eq!(res, exp);

        let vals: Vec<Scalar> = vec![Scalar::i64(1), Scalar::i64(2)];
        let res = Array::from(vals);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_i64_array_to_vec() {
        let exp: Vec<i64> = vec![1, 2];
        let exps: Vec<Scalar> = vec![Scalar::i64(1), Scalar::i64(2)];

        // Into
        let vals = Array::Int64Array(vec![1, 2]);
        let res: Vec<i64> = vals.into();
        assert_eq!(res, exp);

        let vals = Array::Int64Array(vec![1, 2]);
        let res: Vec<Scalar> = vals.into();
        assert_eq!(res, exps);

        // From
        let vals = Array::Int64Array(vec![1, 2]);
        let res: Vec<i64> = Vec::from(vals);
        assert_eq!(res, exp);

        let vals = Array::Int64Array(vec![1, 2]);
        let res: Vec<Scalar> = Vec::from(vals);
        assert_eq!(res, exps);
    }

    #[test]
    fn test_f64_vec_to_array() {
        let exp: Array = Array::Float64Array(vec![1.1, 2.2]);

        // Into
        let vals: Vec<f64> = vec![1.1, 2.2];
        let res: Array = vals.into();
        assert_eq!(res, exp);

        let vals: Vec<Scalar> = vec![Scalar::f64(1.1), Scalar::f64(2.2)];
        let res: Array = vals.into();
        assert_eq!(res, exp);

        // From
        let vals: Vec<f64> = vec![1.1, 2.2];
        let res = Array::from(vals);
        assert_eq!(res, exp);

        let vals: Vec<Scalar> = vec![Scalar::f64(1.1), Scalar::f64(2.2)];
        let res = Array::from(vals);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_f64_array_to_vec() {
        let exp: Vec<f64> = vec![1.1, 2.2];
        let exps: Vec<Scalar> = vec![Scalar::f64(1.1), Scalar::f64(2.2)];

        // Into
        let vals = Array::Float64Array(vec![1.1, 2.2]);
        let res: Vec<f64> = vals.into();
        assert_eq!(res, exp);

        let vals = Array::Float64Array(vec![1.1, 2.2]);
        let res: Vec<Scalar> = vals.into();
        assert_eq!(res, exps);

        // From
        let vals = Array::Float64Array(vec![1.1, 2.2]);
        let res: Vec<f64> = Vec::from(vals);
        assert_eq!(res, exp);

        let vals = Array::Float64Array(vec![1.1, 2.2]);
        let res: Vec<Scalar> = Vec::from(vals);
        assert_eq!(res, exps);
    }

    #[test]
    fn test_bool_vec_to_array() {
        let exp: Array = Array::BoolArray(vec![true, false]);

        // Into
        let vals: Vec<bool> = vec![true, false];
        let res: Array = vals.into();
        assert_eq!(res, exp);

        let vals: Vec<Scalar> = vec![Scalar::bool(true), Scalar::bool(false)];
        let res: Array = vals.into();
        assert_eq!(res, exp);

        // From
        let vals: Vec<bool> = vec![true, false];
        let res = Array::from(vals);
        assert_eq!(res, exp);

        let vals: Vec<Scalar> = vec![Scalar::bool(true), Scalar::bool(false)];
        let res = Array::from(vals);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_bool_array_to_vec() {
        let exp: Vec<bool> = vec![true, false];
        let exps: Vec<Scalar> = vec![Scalar::bool(true), Scalar::bool(false)];

        // Into
        let vals = Array::BoolArray(vec![true, false]);
        let res: Vec<bool> = vals.into();
        assert_eq!(res, exp);

        let vals = Array::BoolArray(vec![true, false]);
        let res: Vec<Scalar> = vals.into();
        assert_eq!(res, exps);

        // From
        let vals = Array::BoolArray(vec![true, false]);
        let res: Vec<bool> = Vec::from(vals);
        assert_eq!(res, exp);

        let vals = Array::BoolArray(vec![true, false]);
        let res: Vec<Scalar> = Vec::from(vals);
        assert_eq!(res, exps);
    }

    #[test]
    fn test_str_vec_to_array() {
        let exp: Array = Array::StringArray(vec!["a".to_string(), "b".to_string()]);

        // Into
        let vals: Vec<String> = vec!["a".to_string(), "b".to_string()];
        let res: Array = vals.into();
        assert_eq!(res, exp);

        let vals: Vec<&str> = vec!["a", "b"];
        let res: Array = vals.into();
        assert_eq!(res, exp);

        let vals: Vec<Scalar> = vec![Scalar::String("a".to_string()),
                                     Scalar::String("b".to_string())];
        let res: Array = vals.into();
        assert_eq!(res, exp);

        // From
        let vals: Vec<String> = vec!["a".to_string(), "b".to_string()];
        let res = Array::from(vals);
        assert_eq!(res, exp);

        let vals: Vec<&str> = vec!["a", "b"];
        let res = Array::from(vals);
        assert_eq!(res, exp);

        let vals: Vec<Scalar> = vec![Scalar::String("a".to_string()),
                                     Scalar::String("b".to_string())];
        let res = Array::from(vals);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_str_array_to_vec() {
        let exp: Vec<String> = vec!["a".to_string(), "b".to_string()];
        let exps: Vec<Scalar> = vec![Scalar::String("a".to_string()),
                                     Scalar::String("b".to_string())];

        // Into
        let vals = Array::StringArray(vec!["a".to_string(), "b".to_string()]);
        let res: Vec<String> = vals.into();
        assert_eq!(res, exp);

        let vals = Array::StringArray(vec!["a".to_string(), "b".to_string()]);
        let res: Vec<Scalar> = vals.into();
        assert_eq!(res, exps);

        // From
        let vals = Array::StringArray(vec!["a".to_string(), "b".to_string()]);
        let res: Vec<String> = Vec::from(vals);
        assert_eq!(res, exp);

        let vals = Array::StringArray(vec!["a".to_string(), "b".to_string()]);
        let res: Vec<Scalar> = Vec::from(vals);
        assert_eq!(res, exps);
    }

    #[test]
    fn test_i64_primitives_to_scalar() {
        let exp = Scalar::i64(1);

        let res: Scalar = 1i64.into();
        assert_eq!(res, exp);

        let res: Scalar = Scalar::from(1i64);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_i64_scalar_to_primitives() {
        let res: i64 = Scalar::i64(1).into();
        assert_eq!(res, 1);

        let res: i64 = i64::from(Scalar::i64(1));
        assert_eq!(res, 1);
    }

    #[test]
    fn test_f64_primitives_to_scalar() {
        let exp = Scalar::f64(1.1);

        let res: Scalar = (1.1).into();
        assert_eq!(res, exp);

        let res: Scalar = Scalar::from(1.1);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_f64_scalar_to_primitives() {
        let res: f64 = Scalar::f64(1.1).into();
        assert_eq!(res, 1.1);

        let res: f64 = f64::from(Scalar::f64(1.1));
        assert_eq!(res, 1.1);
    }

    #[test]
    fn test_bool_primitives_to_scalar() {
        let exp = Scalar::bool(true);

        let res: Scalar = true.into();
        assert_eq!(res, exp);

        let res: Scalar = Scalar::from(true);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_bool_scalar_to_primitives() {
        let res: bool = Scalar::bool(true).into();
        assert_eq!(res, true);

        let res: bool = bool::from(Scalar::bool(true));
        assert_eq!(res, true);
    }

    #[test]
    fn test_str_primitives_to_scalar() {
        let exp = Scalar::String("a".to_string());

        let res: Scalar = "a".to_string().into();
        assert_eq!(res, exp);

        let res: Scalar = Scalar::from("a".to_string());
        assert_eq!(res, exp);

        // &str
        let res: Scalar = "a".into();
        assert_eq!(res, exp);

        let res: Scalar = Scalar::from("a");
        assert_eq!(res, exp);
    }

    #[test]
    fn test_str_scalar_to_primitives() {
        let res: String = Scalar::String("a".to_string()).into();
        assert_eq!(res, "a".to_string());

        let res: String = String::from(Scalar::String("a".to_string()));
        assert_eq!(res, "a".to_string());
    }
}
