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

#[cfg(test)]
mod tests {

    use super::Array;

    #[test]
    fn test_creation() {
        let iarr = Array::Int64Array(vec![1, 2, 3]);
        assert_eq!(iarr.dtype(), "i64");
        assert_eq!(iarr.len(), 3);

        let farr = Array::Float64Array(vec![1.1, 2.1, 3.1, 4.1]);
        assert_eq!(farr.dtype(), "f64");
        assert_eq!(farr.len(), 4);
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
    fn test_from() {
        let iarr: Array = vec![1, 2, 3].into();
        assert_eq!(iarr.dtype(), "i64");

        let farr: Array = vec![1.1, 2.1, 3.1].into();
        assert_eq!(farr.dtype(), "f64");
    }

    #[test]
    fn test_ilocs() {
        let iarr = Array::Int64Array(vec![1, 2, 3, 4, 5]);
        assert_eq!(iarr.dtype(), "i64");
        let ires: Vec<i64> = iarr.ilocs(&vec![1, 4, 0]).into();
        assert_eq!(ires, vec![2, 5, 1]);

        let farr = Array::Float64Array(vec![1.1, 2.1, 3.1, 4.1, 5.1]);
        assert_eq!(farr.dtype(), "f64");
        let fres: Vec<f64> = farr.ilocs(&vec![1, 4, 0]).into();
        assert_eq!(fres, vec![2.1, 5.1, 1.1]);
    }

    #[test]
    fn test_container() {
        let iarr: Array = vec![1, 2, 3].into();
        let farr: Array = vec![1.1, 2.1, 3.1].into();
        assert_eq!(iarr.dtype(), "i64");
        assert_eq!(farr.dtype(), "f64");

        let container: Vec<Array> = vec![iarr, farr];
        assert_eq!(container.len(), 2);
        let dtypes: Vec<String> = container.iter().map(|ref x| x.dtype()).collect();
        assert_eq!(dtypes, vec!["i64", "f64"]);
    }
}