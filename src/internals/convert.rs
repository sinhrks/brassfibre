use super::Array;

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