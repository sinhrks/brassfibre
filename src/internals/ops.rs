use std::ops::{Add, Mul, Sub, Div, Rem, BitAnd, BitOr, BitXor};

use super::Array;
use algos::elemwise::Elemwise;

/// /////////////////////////////////////////////////////////////////////////////
/// Array broadcast ops
/// /////////////////////////////////////////////////////////////////////////////

macro_rules! define_primitive_op {
    ($tr:ident, $m:ident, $ty:ident, $ar:ident) => {

        // broadcast
        impl $tr<$ty> for Array {
            type Output = Array;
            fn $m(self, _rhs: $ty) -> Self::Output {
                let new_values = match self {
                    Array::$ar(vals) => Elemwise::broadcast_oo(vals, _rhs, |x, y| x.$m(y)),
                    _ => panic!("unable to perform op because dtypes are different")
                };
                Array::$ar(new_values)
            }
        }

        impl<'r> $tr<&'r $ty> for Array {
            type Output = Array;
            fn $m(self, _rhs: &'r $ty) -> Self::Output {
                let new_values = match self {
                    Array::$ar(vals) => Elemwise::broadcast_or(vals, _rhs, |x, y| x.$m(y)),
                    _ => panic!("unable to perform op because dtypes are different")
                };
                Array::$ar(new_values)
            }
        }

        impl<'l> $tr<$ty> for &'l Array {
            type Output = Array;
            fn $m(self, _rhs: $ty) -> Self::Output {
                let new_values = match self {
                    &Array::$ar(ref vals) => Elemwise::broadcast_ro(vals, _rhs, |x, y| x.$m(y)),
                    _ => panic!("unable to perform op because dtypes are different")
                };
                Array::$ar(new_values)
            }
        }

        impl<'l, 'r> $tr<&'r $ty> for &'l Array {
            type Output = Array;
            fn $m(self, _rhs: &'r $ty) -> Self::Output {
                let new_values = match self {
                    &Array::$ar(ref vals) => Elemwise::broadcast_rr(vals, _rhs, |x, y| x.$m(y)),
                    _ => panic!("unable to perform op because dtypes are different")
                };
                Array::$ar(new_values)
            }
        }
    }
}

define_primitive_op!(Add, add, i64, Int64Array);
define_primitive_op!(Mul, mul, i64, Int64Array);
define_primitive_op!(Sub, sub, i64, Int64Array);
define_primitive_op!(Div, div, i64, Int64Array);
define_primitive_op!(Rem, rem, i64, Int64Array);
define_primitive_op!(BitAnd, bitand, i64, Int64Array);
define_primitive_op!(BitOr, bitor, i64, Int64Array);
define_primitive_op!(BitXor, bitxor, i64, Int64Array);

define_primitive_op!(Add, add, f64, Float64Array);
define_primitive_op!(Mul, mul, f64, Float64Array);
define_primitive_op!(Sub, sub, f64, Float64Array);
define_primitive_op!(Div, div, f64, Float64Array);
define_primitive_op!(Rem, rem, f64, Float64Array);
// bitwise ops are undefined in float

// arithmetic ops are undefined for bool
define_primitive_op!(BitAnd, bitand, bool, BoolArray);
define_primitive_op!(BitOr, bitor, bool, BoolArray);
define_primitive_op!(BitXor, bitxor, bool, BoolArray);

/// /////////////////////////////////////////////////////////////////////////////
/// Array elementwise ops
/// /////////////////////////////////////////////////////////////////////////////

macro_rules! define_binary_op {
    ($tr:ident, $m:ident, $ot:ident) => {

        // elemwise
        impl $tr<Array> for Array {
            type Output = Array;
            fn $m(self, _rhs: Array) -> Self::Output {
                match (self, _rhs) {
                    (Array::Int64Array(lvals), Array::Int64Array(rvals)) => {
                        Array::Int64Array(Elemwise::elemwise_oo(lvals, rvals, |x, y| x.$m(y)))
                    },
                    (Array::$ot(lvals), Array::$ot(rvals)) => {
                        Array::$ot(Elemwise::elemwise_oo(lvals, rvals, |x, y| x.$m(y)))
                    },
                    (_, _) => panic!("unable to perform op because dtypes are different")
                }
            }
        }

        impl<'r> $tr<&'r Array> for Array {
            type Output = Array;
            fn $m(self, _rhs: &Array) -> Self::Output {
                match (self, _rhs) {
                    // ToDo: Can use elemwise_or? define fn to return ref to Array?
                    (Array::Int64Array(ref lvals), &Array::Int64Array(ref rvals)) => {
                        Array::Int64Array(Elemwise::elemwise_rr(lvals, &rvals, |x, y| x.$m(y)))
                    },

                    (Array::$ot(ref lvals), &Array::$ot(ref rvals)) => {
                        Array::$ot(Elemwise::elemwise_rr(lvals, &rvals, |x, y| x.$m(y)))
                    },
                    (_, _) => panic!("unable to perform op because dtypes are different")
                }
            }
        }

        impl<'l> $tr<Array> for &'l Array {
            type Output = Array;
            fn $m(self, _rhs: Array) -> Self::Output {
                match (self, _rhs) {
                    // ToDo: Can use elemwise_or?
                    (&Array::Int64Array(ref lvals), Array::Int64Array(ref rvals)) => {
                        Array::Int64Array(Elemwise::elemwise_rr(lvals, &rvals, |x, y| x.$m(y)))
                    },
                    (&Array::$ot(ref lvals), Array::$ot(ref rvals)) => {
                        Array::$ot(Elemwise::elemwise_rr(lvals, &rvals, |x, y| x.$m(y)))
                    },
                    (_, _) => panic!("unable to perform op because dtypes are different")
                }
            }
        }

        impl<'l, 'r> $tr<&'r Array> for &'l Array {
            type Output = Array;
            fn $m(self, _rhs: &Array) -> Self::Output {
                match (self, _rhs) {
                    (&Array::Int64Array(ref lvals), &Array::Int64Array(ref rvals)) => {
                        Array::Int64Array(Elemwise::elemwise_rr(lvals, &rvals, |x, y| x.$m(y)))
                    },
                    (&Array::$ot(ref lvals), &Array::$ot(ref rvals)) => {
                        Array::$ot(Elemwise::elemwise_rr(lvals, &rvals, |x, y| x.$m(y)))
                    },
                    (_, _) => panic!("unable to perform op because dtypes are different")
                }
            }
        }
    }
}

// bitwise ops are undefined in float
define_binary_op!(Add, add, Float64Array);
define_binary_op!(Mul, mul, Float64Array);
define_binary_op!(Sub, sub, Float64Array);
define_binary_op!(Div, div, Float64Array);
define_binary_op!(Rem, rem, Float64Array);

// arithmetic ops are undefined for bool
define_binary_op!(BitAnd, bitand, BoolArray);
define_binary_op!(BitOr, bitor, BoolArray);
define_binary_op!(BitXor, bitxor, BoolArray);

#[cfg(test)]
mod tests {

    use super::super::Array;

    /// /////////////////////////////////////////////////////////////////////////
    /// arithmetic op
    /// /////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_array_ops_i64_broadcast() {
        // arr moves by ops
        let arr = Array::Int64Array(vec![1, 2, 3]);
        assert_eq!(arr + 3, Array::Int64Array(vec![4, 5, 6]));

        let arr = Array::Int64Array(vec![1, 2, 3]);
        assert_eq!(arr * 2, Array::Int64Array(vec![2, 4, 6]));

        let arr = Array::Int64Array(vec![1, 2, 3]);
        assert_eq!(arr - 3, Array::Int64Array(vec![-2, -1, 0]));

        let arr = Array::Int64Array(vec![1, 2, 3]);
        assert_eq!(arr / 2, Array::Int64Array(vec![0, 1, 1]));

        let arr = Array::Int64Array(vec![1, 2, 3]);
        assert_eq!(arr % 2, Array::Int64Array(vec![1, 0, 1]));
    }

    #[test]
    fn test_array_ops_i64_broadcast_refs() {
        let arr = Array::Int64Array(vec![1, 2, 3]);
        assert_eq!(&arr + 3, Array::Int64Array(vec![4, 5, 6]));

        let arr = Array::Int64Array(vec![1, 2, 3]);
        assert_eq!(&arr + &3, Array::Int64Array(vec![4, 5, 6]));

        let arr = Array::Int64Array(vec![1, 2, 3]);
        assert_eq!(arr + &3, Array::Int64Array(vec![4, 5, 6]));
    }

    #[test]
    fn test_array_ops_f64_broadcast() {
        let arr = Array::Float64Array(vec![1., 2., 3.]);
        assert_eq!(arr + 3., Array::Float64Array(vec![4., 5., 6.]));

        let arr = Array::Float64Array(vec![1., 2., 3.]);
        assert_eq!(arr * 2., Array::Float64Array(vec![2., 4., 6.]));

        let arr = Array::Float64Array(vec![1., 2., 3.]);
        assert_eq!(arr - 3., Array::Float64Array(vec![-2., -1., 0.]));

        let arr = Array::Float64Array(vec![1., 2., 3.]);
        assert_eq!(arr / 2., Array::Float64Array(vec![0.5, 1., 1.5]));

        let arr = Array::Float64Array(vec![1., 2., 3.]);
        assert_eq!(arr % 2., Array::Float64Array(vec![1., 0., 1.]));
    }

    // ToDo
    // #[test]
    // fn test_index_ops_str_broadcast() {
    // let idx = Indexer::<String>::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    // idx moves by ops
    // let exp = Indexer::<String>::new(vec!["ax".to_string(), "bx".to_string(), "cx".to_string()]);
    // assert_eq!(idx + "x".to_string(), exp);
    // }
    //

    #[test]
    fn test_array_ops_i64_elemwise() {
        let l = Array::Int64Array(vec![1, 2, 3]);
        let r = Array::Int64Array(vec![1, 3, 2]);
        assert_eq!(l + r, Array::Int64Array(vec![2, 5, 5]));

        let l = Array::Int64Array(vec![1, 2, 3]);
        let r = Array::Int64Array(vec![1, 3, 2]);
        assert_eq!(l * r, Array::Int64Array(vec![1, 6, 6]));

        let l = Array::Int64Array(vec![1, 2, 3]);
        let r = Array::Int64Array(vec![1, 3, 2]);
        assert_eq!(l - r, Array::Int64Array(vec![0, -1, 1]));

        let l = Array::Int64Array(vec![1, 2, 3]);
        let r = Array::Int64Array(vec![1, 3, 2]);
        assert_eq!(l / r, Array::Int64Array(vec![1, 0, 1]));

        let l = Array::Int64Array(vec![1, 2, 3]);
        let r = Array::Int64Array(vec![1, 3, 2]);
        assert_eq!(l % r, Array::Int64Array(vec![0, 2, 1]));
    }

    #[test]
    fn test_array_ops_i64_elemwise_refs() {
        let l = Array::Int64Array(vec![1, 2, 3]);
        let r = Array::Int64Array(vec![1, 3, 2]);
        assert_eq!(&l + r, Array::Int64Array(vec![2, 5, 5]));

        let l = Array::Int64Array(vec![1, 2, 3]);
        let r = Array::Int64Array(vec![1, 3, 2]);
        assert_eq!(&l + &r, Array::Int64Array(vec![2, 5, 5]));

        let l = Array::Int64Array(vec![1, 2, 3]);
        let r = Array::Int64Array(vec![1, 3, 2]);
        assert_eq!(l + &r, Array::Int64Array(vec![2, 5, 5]));
    }

    #[test]
    fn test_array_ops_f64_elemwise() {
        let l = Array::Float64Array(vec![1., 2., 3.]);
        let r = Array::Float64Array(vec![1., 3., 2.]);
        assert_eq!(l + r, Array::Float64Array(vec![2., 5., 5.]));

        let l = Array::Float64Array(vec![1., 2., 3.]);
        let r = Array::Float64Array(vec![1., 3., 2.]);
        assert_eq!(l * r, Array::Float64Array(vec![1., 6., 6.]));

        let l = Array::Float64Array(vec![1., 2., 3.]);
        let r = Array::Float64Array(vec![1., 3., 2.]);
        assert_eq!(l - r, Array::Float64Array(vec![0., -1., 1.]));

        let l = Array::Float64Array(vec![1., 2., 3.]);
        let r = Array::Float64Array(vec![1., 3., 2.]);
        assert_eq!(l / r,
                   Array::Float64Array(vec![1., 0.6666666666666666, 1.5]));

        let l = Array::Float64Array(vec![1., 2., 3.]);
        let r = Array::Float64Array(vec![1., 3., 2.]);
        assert_eq!(l % r, Array::Float64Array(vec![0., 2., 1.]));
    }

    /// /////////////////////////////////////////////////////////////////////////
    /// bitwise op
    /// /////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_array_bit_i64_broadcast() {
        let arr = Array::Int64Array(vec![1, 2, 3]);
        assert_eq!(arr & 3, Array::Int64Array(vec![1 & 3, 2 & 3, 3 & 3]));

        let arr = Array::Int64Array(vec![1, 2, 3]);
        assert_eq!(arr | 3, Array::Int64Array(vec![1 | 3, 2 | 3, 3 | 3]));

        let arr = Array::Int64Array(vec![1, 2, 3]);
        assert_eq!(arr ^ 3, Array::Int64Array(vec![1 ^ 3, 2 ^ 3, 3 ^ 3]));
    }

    #[test]
    fn test_array_bit_bool_broadcast() {
        let arr = Array::BoolArray(vec![true, false, true]);
        assert_eq!(arr & true, Array::BoolArray(vec![true, false, true]));

        let arr = Array::BoolArray(vec![true, false, true]);
        assert_eq!(arr | false, Array::BoolArray(vec![true, false, true]));

        let arr = Array::BoolArray(vec![true, false, true]);
        assert_eq!(arr ^ true, Array::BoolArray(vec![false, true, false]));
    }

    #[test]
    fn test_array_bit_i64_elemwise() {
        let l = Array::Int64Array(vec![1, 2, 3]);
        let r = Array::Int64Array(vec![1, 3, 2]);
        assert_eq!(l & r, Array::Int64Array(vec![1 & 1, 2 & 3, 3 & 2]));

        let l = Array::Int64Array(vec![1, 2, 3]);
        let r = Array::Int64Array(vec![1, 3, 2]);
        assert_eq!(l | r, Array::Int64Array(vec![1 | 1, 2 | 3, 3 | 2]));

        let l = Array::Int64Array(vec![1, 2, 3]);
        let r = Array::Int64Array(vec![1, 3, 2]);
        assert_eq!(l ^ r, Array::Int64Array(vec![1 ^ 1, 2 ^ 3, 3 ^ 2]));
    }

    #[test]
    fn test_array_bit_bool_elemwise() {
        let l = Array::BoolArray(vec![true, false, true]);
        let r = Array::BoolArray(vec![false, true, true]);
        assert_eq!(l & r, Array::BoolArray(vec![false, false, true]));

        let l = Array::BoolArray(vec![true, false, true]);
        let r = Array::BoolArray(vec![false, true, true]);
        assert_eq!(l | r, Array::BoolArray(vec![true, true, true]));

        let l = Array::BoolArray(vec![true, false, true]);
        let r = Array::BoolArray(vec![false, true, true]);
        assert_eq!(l ^ r, Array::BoolArray(vec![true, true, false]));
    }
}
