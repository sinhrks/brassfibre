use std::borrow::Cow;
use std::hash::Hash;

use super::DataFrame;
use super::super::series::Series;
use super::super::traits::Aggregator;

impl<'i, 'c, I, C> Aggregator<'c> for DataFrame<'i, 'c, I, C>
    where I: Clone + Eq + Hash,
          C: 'c + Clone + Eq + Hash {

    // ToDo: use 'n lifetime for values
    type Kept = Series<'c, 'c, f64, C>;
    type Counted = Series<'c, 'c, usize, C>;
    type Coerced = Series<'c, 'c, f64, C>;

    fn sum(&'c self) -> Self::Kept {
        let ndf = self.get_numeric_data();
        let new_values: Vec<f64> = ndf.values.iter().map(|x| x.sum()).collect();
        Series::from_cow(Cow::Owned(new_values), ndf.columns)
    }

    fn count(&'c self) -> Self::Counted {
        let ndf = self.get_numeric_data();
        let new_values: Vec<usize> = ndf.values.iter().map(|x| x.count()).collect();
        Series::from_cow(Cow::Owned(new_values), ndf.columns)
    }

    fn mean(&'c self) -> Self::Coerced {
        let ndf = self.get_numeric_data();
        let new_values: Vec<f64> = ndf.values.iter().map(|x| x.mean()).collect();
        Series::from_cow(Cow::Owned(new_values), ndf.columns)
    }

    fn var(&'c self) -> Self::Coerced {
        let ndf = self.get_numeric_data();
        let new_values: Vec<f64> = ndf.values.iter().map(|x| x.var()).collect();
        Series::from_cow(Cow::Owned(new_values), ndf.columns)
    }

    fn unbiased_var(&'c self) -> Self::Coerced {
        let ndf = self.get_numeric_data();
        let new_values: Vec<f64> = ndf.values.iter().map(|x| x.unbiased_var()).collect();
        Series::from_cow(Cow::Owned(new_values), ndf.columns)
    }

    fn std(&'c self) -> Self::Coerced {
        let ndf = self.get_numeric_data();
        let new_values: Vec<f64> = ndf.values.iter().map(|x| x.std()).collect();
        Series::from_cow(Cow::Owned(new_values), ndf.columns)
    }

    fn unbiased_std(&'c self) -> Self::Coerced {
        let ndf = self.get_numeric_data();
        let new_values: Vec<f64> = ndf.values.iter().map(|x| x.unbiased_std()).collect();
        Series::from_cow(Cow::Owned(new_values), ndf.columns)
    }
}
