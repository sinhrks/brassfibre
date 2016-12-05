use super::Array;
use super::super::computations;
use super::super::traits::Aggregator;


impl<'s> Aggregator<'s, 's> for Array {

    type Kept = f64;
    type Counted = usize;
    type Coerced = f64;

    fn sum(&'s self) -> Self::Kept {
        match self {
            &Array::Int64Array(ref vals) => computations::vec_sum(&vals) as f64,
            &Array::Float64Array(ref vals) => computations::vec_sum(&vals),
            _ => panic!("unable to aggregate non-numeric values"),
        }
    }

    fn count(&'s self) -> Self::Counted {
        match self {
            &Array::Int64Array(ref vals) => computations::vec_count(&vals),
            &Array::Float64Array(ref vals) => computations::vec_count(&vals),
            _ => panic!("unable to aggregate non-numeric values"),
        }
    }

    fn mean(&'s self) -> Self::Coerced {
        match self {
            &Array::Int64Array(ref vals) => computations::vec_mean(&vals) as f64,
            &Array::Float64Array(ref vals) => computations::vec_mean(&vals),
            _ => panic!("unable to aggregate non-numeric values"),
        }
    }

    fn var(&'s self) -> Self::Coerced {
        match self {
            &Array::Int64Array(ref vals) => computations::vec_var(&vals),
            &Array::Float64Array(ref vals) => computations::vec_var(&vals),
            _ => panic!("unable to aggregate non-numeric values"),
        }
    }

    fn unbiased_var(&'s self) -> Self::Coerced {
        match self {
            &Array::Int64Array(ref vals) => computations::vec_unbiased_var(&vals),
            &Array::Float64Array(ref vals) => computations::vec_unbiased_var(&vals),
            _ => panic!("unable to aggregate non-numeric values"),
        }
    }

    fn std(&'s self) -> Self::Coerced {
        match self {
            &Array::Int64Array(ref vals) => computations::vec_std(&vals),
            &Array::Float64Array(ref vals) => computations::vec_std(&vals),
            _ => panic!("unable to aggregate non-numeric values"),
        }
    }

    fn unbiased_std(&'s self) -> Self::Coerced {
        match self {
            &Array::Int64Array(ref vals) => computations::vec_unbiased_std(&vals) as f64,
            &Array::Float64Array(ref vals) => computations::vec_unbiased_std(&vals),
            _ => panic!("unable to aggregate non-numeric values"),
        }
    }
}
