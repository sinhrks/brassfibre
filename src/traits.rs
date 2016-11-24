//! Common Traits

////////////////////////////////////////////////////////////////////////////////
// Indexing
////////////////////////////////////////////////////////////////////////////////

/// Indexing methods for Indexer
pub trait IndexerIndexer<U> {
    fn len(&self) -> usize;
    fn contains(&self, label: &U) -> bool;
    fn push(&mut self, label: U);
    fn get_loc(&self, label: &U) -> usize;
    fn get_locs(&self, labels: &Vec<U>) -> Vec<usize>;
    fn reindex(&self, locations: &Vec<usize>) -> Self;

    // temp
    fn init_state(&self);
}

/// Indexing methods for Index(Row)
pub trait RowIndexer<T>: Sized {

    fn reindex(&self, labels: &Vec<T>) -> Self;
    fn reindex_by_index(&self, locations: &Vec<usize>) -> Self;

    // selection

    /// Slice using given labels (slice by LOCationS)
    fn locs(&self, labels: &Vec<T>) -> Self {
        self.reindex(labels)
    }

    /// Slice using given indices (slice by Index LOCationS)
    fn ilocs(&self, locations: &Vec<usize>) -> Self {
        self.reindex_by_index(locations)
    }

    /// Slice using given Vec<bool> (slice by Bool LOCationS)
    fn blocs(&self, flags: &Vec<bool>) -> Self;
}

/// Indexing methods for Columns
pub trait ColIndexer<T, S>: Sized {

    /// Get column using label
    fn get(&self, label: &T) -> S;

    /// Get column using given index
    fn iget(&self, label: &usize) -> S;

    /// Slice columns using labels
    fn gets(&self, labels: &Vec<T>) -> Self;

    /// Slice columns given indices
    fn igets(&self, locations: &Vec<usize>) -> Self;
}

////////////////////////////////////////////////////////////////////////////////
// Append
////////////////////////////////////////////////////////////////////////////////

pub trait Appender: Sized {
    fn append(&self, other: &Self) -> Self;
}

////////////////////////////////////////////////////////////////////////////////
// Apply
////////////////////////////////////////////////////////////////////////////////

pub trait Applicable<T, R, C> {

    // T: Type for myself
    // R: Type function returns
    // C: Type of container which can hold W as values

    fn apply(&self, func: &Fn(&T) -> R) -> C;
}

////////////////////////////////////////////////////////////////////////////////
// Aggregation
////////////////////////////////////////////////////////////////////////////////

pub trait Aggregator {
    // result which can keep current dtype
    type Kept;
    // result for count (to usize or its container)
    type Counted;
    // result which is coerced (to f64 or its container)
    type Coerced;

    fn sum(&self) -> Self::Kept;
    fn count(&self) -> Self::Counted;
    fn mean(&self) -> Self::Coerced;
    fn var(&self) -> Self::Coerced;
    fn unbiased_var(&self) -> Self::Coerced;
    fn std(&self) -> Self::Coerced;
    fn unbiased_std(&self) -> Self::Coerced;
}