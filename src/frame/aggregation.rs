use std::borrow::Cow;
use std::hash::Hash;

use nullvec::prelude::{Array, Scalar, Nullable, NullVec};
use nullvec::prelude::BasicAggregation as NBasicAggregation;
use nullvec::prelude::NumericAggregation as NNumericAggregation;
use nullvec::prelude::ComparisonAggregation as NComparisonAggregation;


use super::DataFrame;
use indexer::Indexer;
use series::Series;
use traits::{BasicAggregation, NumericAggregation, ComparisonAggregation, Description};

impl<'v, 'i, 'c, I, C> BasicAggregation<'c> for DataFrame<'v, 'i, 'c, I, C>
    where I: Clone + Eq + Hash,
          C: 'c + Clone + Eq + Hash
{
    // ToDo: use 'n lifetime for values
    type Kept = Series<'c, 'c, Scalar, C>;
    type Counted = Series<'c, 'c, usize, C>;

    fn sum(&'c self) -> Self::Kept {
        let ndf = self.get_numeric_data();
        // ToDo: FIXME
        let new_values: Vec<Scalar> = ndf.values.iter().map(|x| x.sum()).collect();
        Series::from_cow(Cow::Owned(new_values), ndf.columns)
    }

    fn count(&'c self) -> Self::Counted {
        let ndf = self.get_numeric_data();
        let new_values: Vec<usize> = ndf.values.iter().map(|x| x.count()).collect();
        Series::from_cow(Cow::Owned(new_values), ndf.columns)
    }
}

impl<'v, 'i, 'c, I, C> NumericAggregation<'c> for DataFrame<'v, 'i, 'c, I, C>
    where I: Clone + Eq + Hash,
          C: 'c + Clone + Eq + Hash
{
    // ToDo: use 'n lifetime for values
    type Coerced = Series<'c, 'c, f64, C>;

    fn mean(&'c self) -> Self::Coerced {
        let ndf = self.get_numeric_data();
        // ToDo: FIXME
        let new_values_tmp: NullVec<f64> = ndf.values.iter().map(|x| x.mean()).collect();
        let new_values: Vec<f64> = new_values_tmp.into();
        Series::from_cow(Cow::Owned(new_values), ndf.columns)
    }

    fn var(&'c self) -> Self::Coerced {
        let ndf = self.get_numeric_data();
        // ToDo: FIXME
        let new_values_tmp: NullVec<f64> = ndf.values.iter().map(|x| x.var()).collect();
        let new_values: Vec<f64> = new_values_tmp.into();
        Series::from_cow(Cow::Owned(new_values), ndf.columns)
    }

    fn unbiased_var(&'c self) -> Self::Coerced {
        let ndf = self.get_numeric_data();
        // ToDo: FIXME
        let new_values_tmp: NullVec<f64> = ndf.values.iter().map(|x| x.unbiased_var()).collect();
        let new_values: Vec<f64> = new_values_tmp.into();
        Series::from_cow(Cow::Owned(new_values), ndf.columns)
    }

    fn std(&'c self) -> Self::Coerced {
        let ndf = self.get_numeric_data();
        // ToDo: FIXME
        let new_values_tmp: NullVec<f64> = ndf.values.iter().map(|x| x.std()).collect();
        let new_values: Vec<f64> = new_values_tmp.into();
        Series::from_cow(Cow::Owned(new_values), ndf.columns)
    }

    fn unbiased_std(&'c self) -> Self::Coerced {
        let ndf = self.get_numeric_data();
        // ToDo: FIXME
        let new_values_tmp: NullVec<f64> = ndf.values.iter().map(|x| x.unbiased_std()).collect();
        let new_values: Vec<f64> = new_values_tmp.into();
        Series::from_cow(Cow::Owned(new_values), ndf.columns)
    }
}

impl<'v, 'i, 'c, I, C> ComparisonAggregation<'c> for DataFrame<'v, 'i, 'c, I, C>
    where I: Clone + Eq + Hash,
          C: 'c + Clone + Eq + Hash
{
    // ToDo: use 'n lifetime for values
    type Kept = Series<'c, 'c, Scalar, C>;

    fn min(&'c self) -> Self::Kept {
        let ndf = self.get_numeric_data();
        // ToDo: FIXME
        let new_values: Vec<Scalar> = ndf.values.iter().map(|x| x.min()).collect();
        Series::from_cow(Cow::Owned(new_values), ndf.columns)
    }

    fn max(&'c self) -> Self::Kept {
        let ndf = self.get_numeric_data();
        // ToDo: FIXME
        let new_values: Vec<Scalar> = ndf.values.iter().map(|x| x.max()).collect();
        Series::from_cow(Cow::Owned(new_values), ndf.columns)
    }
}

impl<'v, 'i, 'c, I, C> Description<'c> for DataFrame<'v, 'i, 'c, I, C>
    where I: Clone + Eq + Hash,
          C: Clone + Eq + Hash
{
    type Described = DataFrame<'v, 'c, 'c, &'c str, C>;

    fn describe(&'c self) -> Self::Described {
        let ndf = self.get_numeric_data();

        let new_index: Vec<&str> = vec!["count", "mean", "std", "min", "max"];

        let describe = |x: &Array| {
            let values: Vec<Nullable<f64>> = vec![Nullable::new(x.count() as f64),
                                                  x.mean(),
                                                  x.std(),
                                                  x.min().as_f64(),
                                                  x.max().as_f64()];
            let nvalues: NullVec<f64> = values.into();
            Array::Float64Array(nvalues)
        };

        let new_values: Vec<Cow<Array>> = ndf.values
            .iter()
            .map(|ref x| Cow::Owned(describe(x)))
            .collect();
        DataFrame::from_cow(new_values, Cow::Owned(Indexer::new(new_index)), ndf.columns)
    }
}
