use super::Array;
use algos::computation::Aggregation;
use traits::{BasicAggregation, NumericAggregation,
             ComparisonAggregation};


impl<'s> BasicAggregation<'s> for Array {

    type Kept = f64;
    type Counted = usize;

    fn sum(&'s self) -> Self::Kept {
        match self {
            &Array::Int64Array(ref vals) => Aggregation::vec_sum(&vals) as f64,
            &Array::Float64Array(ref vals) => Aggregation::vec_sum(&vals),
            _ => panic!("unable to aggregate non-numeric values"),
        }
    }

    fn count(&'s self) -> Self::Counted {
        match self {
            &Array::Int64Array(ref vals) => Aggregation::vec_count(&vals),
            &Array::Float64Array(ref vals) => Aggregation::vec_count(&vals),
            _ => panic!("unable to aggregate non-numeric values"),
        }
    }
}

impl<'s> NumericAggregation<'s> for Array {

    type Coerced = f64;

    fn mean(&'s self) -> Self::Coerced {
        match self {
            &Array::Int64Array(ref vals) => Aggregation::vec_mean(&vals) as f64,
            &Array::Float64Array(ref vals) => Aggregation::vec_mean(&vals),
            _ => panic!("unable to aggregate non-numeric values"),
        }
    }

    fn var(&'s self) -> Self::Coerced {
        match self {
            &Array::Int64Array(ref vals) => Aggregation::vec_var(&vals),
            &Array::Float64Array(ref vals) => Aggregation::vec_var(&vals),
            _ => panic!("unable to aggregate non-numeric values"),
        }
    }

    fn unbiased_var(&'s self) -> Self::Coerced {
        match self {
            &Array::Int64Array(ref vals) => Aggregation::vec_unbiased_var(&vals),
            &Array::Float64Array(ref vals) => Aggregation::vec_unbiased_var(&vals),
            _ => panic!("unable to aggregate non-numeric values"),
        }
    }

    fn std(&'s self) -> Self::Coerced {
        match self {
            &Array::Int64Array(ref vals) => Aggregation::vec_std(&vals),
            &Array::Float64Array(ref vals) => Aggregation::vec_std(&vals),
            _ => panic!("unable to aggregate non-numeric values"),
        }
    }

    fn unbiased_std(&'s self) -> Self::Coerced {
        match self {
            &Array::Int64Array(ref vals) => Aggregation::vec_unbiased_std(&vals) as f64,
            &Array::Float64Array(ref vals) => Aggregation::vec_unbiased_std(&vals),
            _ => panic!("unable to aggregate non-numeric values"),
        }
    }
}

impl<'s> ComparisonAggregation<'s> for Array {

    type Kept = f64;

    fn min(&'s self) -> Self::Kept {
        match self {
            &Array::Int64Array(ref vals) => Aggregation::vec_min(&vals) as f64,
            &Array::Float64Array(ref vals) => Aggregation::vec_min(&vals),
            _ => panic!("unable to aggregate non-numeric values"),
        }
    }

    fn max(&'s self) -> Self::Kept {
        match self {
            &Array::Int64Array(ref vals) => Aggregation::vec_max(&vals) as f64,
            &Array::Float64Array(ref vals) => Aggregation::vec_max(&vals),
            _ => panic!("unable to aggregate non-numeric values"),
        }
    }
}
