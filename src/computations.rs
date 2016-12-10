
use std::cmp;
use num::{Num, Zero, Float, ToPrimitive};

// Aggregation

pub fn vec_sum<T>(values: &Vec<T>) -> T
    where T: Clone + Num + Zero {

    // ToDo: Use AsRef
    values.iter().fold(T::zero(), |a, b| a + b.clone())
}

pub fn vec_count<T>(values: &Vec<T>) -> usize {
    values.len()
}

pub fn vec_count_as_f64<T>(values: &Vec<T>) -> f64 {
    ToPrimitive::to_f64(&vec_count(values)).unwrap()
}

pub fn vec_mean<T>(values: &Vec<T>) -> f64
    where T: Clone + Num + Zero + ToPrimitive {

    ToPrimitive::to_f64(&vec_sum(values)).unwrap() /
                        vec_count_as_f64(values)
}

fn mean_sq<T>(values: &Vec<T>) -> f64
    where T: Clone + Num + Zero + ToPrimitive {
    // use two pass algorithm, assuming data is not large
    let mean = vec_mean(values);
    values.iter()
          .map(|x| ToPrimitive::to_f64(x).unwrap())
          .fold(0., |a, b| a + (b - mean) * (b - mean))
}

pub fn vec_var<T>(values: &Vec<T>) -> f64
    where T: Clone + Num + Zero + ToPrimitive {

    mean_sq(values) / vec_count_as_f64(values)
}

pub fn vec_unbiased_var<T>(values: &Vec<T>) -> f64
    where T: Clone + Num + Zero + ToPrimitive {

    mean_sq(values) / (vec_count_as_f64(values) - 1.)
}

pub fn vec_std<T>(values: &Vec<T>) -> f64
    where T: Clone + Num + Zero + ToPrimitive {

    vec_var(values).sqrt()
}

pub fn vec_unbiased_std<T>(values: &Vec<T>) -> f64
    where T: Clone + Num + Zero + ToPrimitive {

    vec_unbiased_var(values).sqrt()
}

//**********************************************
// Min, Max
//**********************************************

pub trait NanMinMax<A> {
    fn nanmin(&self, n: A) -> A;
    fn nanmax(&self, n: A) -> A;
    fn nanmin_value() -> A;
    fn nanmax_value() -> A;
}

macro_rules! define_int_stats {
    ($t:ident) => {
        impl NanMinMax<$t> for $t {
            fn nanmin(&self, n: $t) -> $t {
                cmp::min(*self, n)
            }
            fn nanmax(&self, n: $t) -> $t {
                cmp::max(*self, n)
            }
            fn nanmin_value() -> $t {
                $t::min_value()
            }
            fn nanmax_value() -> $t {
                $t::max_value()
            }
        }
    }
}

macro_rules! define_float_stats {
    ($t:ident) => {
        impl NanMinMax<$t> for $t {
            fn nanmin(&self, n: $t) -> $t {
                self.min(n)
            }
            fn nanmax(&self, n: $t) -> $t {
                self.max(n)
            }
            fn nanmin_value() -> $t {
                $t::min_value()
            }
            fn nanmax_value() -> $t {
                $t::max_value()
            }
        }
    }
}

define_int_stats!(i64);
define_int_stats!(i32);
define_int_stats!(i16);
define_int_stats!(i8);
define_int_stats!(isize);
define_int_stats!(u64);
define_int_stats!(u32);
define_int_stats!(u16);
define_int_stats!(u8);
define_int_stats!(usize);
define_float_stats!(f64);
define_float_stats!(f32);


pub fn vec_min<T>(values: &Vec<T>) -> T
    where T: Clone + Num + NanMinMax<T> {

    // can't use normal min(a, b), because it can't handle NaN
    values.iter().fold(T::nanmax_value(), |a, b| a.nanmin((*b).clone()))
}

pub fn vec_max<T>(values: &Vec<T>) -> T
    where T: Clone + Num + NanMinMax<T> {

    values.iter().fold(T::nanmin_value(), |a, b| a.nanmax((*b).clone()))
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_vec_sum_f64() {
        let values: Vec<f64> = vec![1., 2., 3.];
        assert_eq!(super::vec_sum(&values), 6.);
        assert_eq!(super::vec_sum(&values), 6.);
    }

    #[test]
    fn test_vec_mean_f64() {
        let values: Vec<f64> = vec![1., 2., 3., 4.];
        assert_eq!(super::vec_mean(&values), 2.5);
    }

    #[test]
    fn test_vec_sum_i64() {
        let values: Vec<i64> = vec![1, 2, 3, 5];
        assert_eq!(super::vec_sum(&values), 11);
    }

    #[test]
    fn test_vec_mean_i64() {
        let values: Vec<i64> = vec![1, 2, 3, 4];
        assert_eq!(super::vec_mean(&values), 2.5);
    }

    #[test]
    fn test_vec_count_f64() {
        let values: Vec<f64> = vec![1., 2., 3.];
        assert_eq!(super::vec_count(&values), 3);
        assert_eq!(super::vec_count_as_f64(&values), 3.);
    }

    #[test]
    fn test_vec_count_str() {
        let values: Vec<&str> = vec!["A", "B", "C"];
        assert_eq!(super::vec_count(&values), 3);
        assert_eq!(super::vec_count_as_f64(&values), 3.);
    }

    #[test]
    fn test_vec_var() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];

        assert_eq!(super::vec_mean(&values), 3.0);
        assert_eq!(super::vec_var(&values), 2.0);
        assert_eq!(super::vec_unbiased_var(&values), 2.5);
    }

    #[test]
    fn test_vec_std() {
        let values: Vec<i64> = vec![11, 12, 11, 14, 12];
        assert_eq!(super::vec_var(&values), 1.2);
        assert_eq!(super::vec_unbiased_var(&values), 1.5);

        assert_eq!(super::vec_std(&values), 1.0954451150103321);
        assert_eq!(super::vec_unbiased_std(&values), 1.2247448713915889);
    }

    #[test]
    fn test_scalar_minmax() {
        use super::NanMinMax;
        assert_eq!(3.nanmax(4), 4);
        assert_eq!(3.nanmin(4), 3);

        assert_eq!(3.1.nanmax(4.1), 4.1);
        assert_eq!(3.1.nanmin(4.1), 3.1);
    }

    #[test]
    fn test_vec_mimnax() {
        let values: Vec<i64> = vec![3, 2, 1, 5, 2, 6, 3];
        assert_eq!(super::vec_min(&values), 1);
        assert_eq!(super::vec_max(&values), 6);
    }

    #[test]
    fn test_vec_mimnax_float() {
        let values: Vec<f64> = vec![3., 2., 1., 5., 2., 6., 3.];
        assert_eq!(super::vec_min(&values), 1.);
        assert_eq!(super::vec_max(&values), 6.);
    }
}
