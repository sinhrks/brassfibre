extern crate num;

use num::{Num, Zero, Float, ToPrimitive};

// Aggregation

pub fn vec_sum<T: Copy + Num + Zero>(values: &Vec<T>) -> T {
    return values.iter().fold(T::zero(), |a, b| a + *b);
}

pub fn vec_count<T>(values: &Vec<T>) -> usize {
    return values.len();
}

pub fn vec_count_as_f64<T>(values: &Vec<T>) -> f64 {
    return ToPrimitive::to_f64(&vec_count(values)).unwrap();
}

pub fn vec_mean<T: Copy + Num + Zero + ToPrimitive>(values: &Vec<T>) -> f64 {
    return ToPrimitive::to_f64(&vec_sum(values)).unwrap() /
           vec_count_as_f64(values);
}

fn mean_sq<T: Copy + Num + Zero + ToPrimitive>(values: &Vec<T>) -> f64 {
    // use two pass algorithm, assuming data is not large
    let mean = vec_mean(values);

    return values.iter()
                 .map(|x| ToPrimitive::to_f64(x).unwrap())
                 .fold(0., |a, b| a + (b - mean) * (b - mean));
}

pub fn vec_var<T: Copy + Num + Zero + ToPrimitive>(values: &Vec<T>) -> f64 {
    return mean_sq(values) / vec_count_as_f64(values);
}

pub fn vec_unbiased_var<T: Copy + Num + Zero + ToPrimitive>(values: &Vec<T>) -> f64 {
    return mean_sq(values) / (vec_count_as_f64(values) - 1.);
}

pub fn vec_std<T: Copy + Num + Zero + ToPrimitive>(values: &Vec<T>) -> f64 {
    return vec_var(values).sqrt();
}

pub fn vec_unbiased_std<T: Copy + Num + Zero + ToPrimitive>(values: &Vec<T>) -> f64 {
    return vec_unbiased_var(values).sqrt();
}

// MIN / MAX

pub fn vec_min<T: Copy + Num + Ord>(values: &Vec<T>) -> T {
    return *(values.iter().min().unwrap());
}

pub fn vec_max<T: Copy + Num + Ord>(values: &Vec<T>) -> T {
    return *(values.iter().max().unwrap());
}

pub fn vec_min_float<T: Copy + Num + Float>(values: &Vec<T>) -> T {
    // can't use normal min(a, b), because it can't handle NaN
    return values.iter().fold(Float::max_value(), |a, b| a.min(*b));
}

pub fn vec_max_float<T: Copy + Num + Float>(values: &Vec<T>) -> T {
    return values.iter().fold(Float::min_value(), |a, b| a.max(*b));
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_vec_sum_f64() {
        let values: Vec<f64> = vec![1., 2., 3.];
        assert_eq!(&super::vec_sum(&values), &6.);
    }

    #[test]
    fn test_vec_mean_f64() {
        let values: Vec<f64> = vec![1., 2., 3., 4.];
        assert_eq!(&super::vec_mean(&values), &2.5);
    }

    #[test]
    fn test_vec_sum_i64() {
        let values: Vec<i64> = vec![1, 2, 3, 5];
        assert_eq!(&super::vec_sum(&values), &11);
    }

    #[test]
    fn test_vec_mean_i64() {
        let values: Vec<i64> = vec![1, 2, 3, 4];
        assert_eq!(&super::vec_mean(&values), &2.5);
    }

    #[test]
    fn test_vec_count_f64() {
        let values: Vec<f64> = vec![1., 2., 3.];
        assert_eq!(&super::vec_count(&values), &3);
        assert_eq!(&super::vec_count_as_f64(&values), &3.);
    }

    #[test]
    fn test_vec_count_str() {
        let values: Vec<&str> = vec!["A", "B", "C"];
        assert_eq!(&super::vec_count(&values), &3);
        assert_eq!(&super::vec_count_as_f64(&values), &3.);
    }

    #[test]
    fn test_vec_var() {
        let values: Vec<i64> = vec![1, 2, 3, 4, 5];

        assert_eq!(&super::vec_mean(&values), &3.0);
        assert_eq!(&super::vec_var(&values), &2.0);
        assert_eq!(&super::vec_unbiased_var(&values), &2.5);
    }

    #[test]
    fn test_vec_std() {
        let values: Vec<i64> = vec![11, 12, 11, 14, 12];
        assert_eq!(&super::vec_var(&values), &1.2);
        assert_eq!(&super::vec_unbiased_var(&values), &1.5);

        assert_eq!(&super::vec_std(&values), &1.0954451150103321);
        assert_eq!(&super::vec_unbiased_std(&values), &1.2247448713915889);
    }

    #[test]
    fn test_vec_mimnax() {
        let values: Vec<i64> = vec![3, 2, 1, 5, 2, 6, 3];
        assert_eq!(&super::vec_min(&values), &1);
        assert_eq!(&super::vec_max(&values), &6);
    }

    #[test]
    fn test_vec_mimnax_float() {
        let values: Vec<f64> = vec![3., 2., 1., 5., 2., 6., 3.];
        assert_eq!(&super::vec_min_float(&values), &1.);
        assert_eq!(&super::vec_max_float(&values), &6.);
    }
}
