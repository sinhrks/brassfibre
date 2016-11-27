//! Common Traits

////////////////////////////////////////////////////////////////////////////////
// Indexing
////////////////////////////////////////////////////////////////////////////////

/// Indexing methods for Indexer
pub trait IndexerIndexer {

    type Key;

    fn len(&self) -> usize;
    fn contains(&self, label: &Self::Key) -> bool;
    fn push(&mut self, label: Self::Key);
    fn get_loc(&self, label: &Self::Key) -> usize;
    fn get_locs(&self, labels: &Vec<Self::Key>) -> Vec<usize>;
    fn reindex(&self, locations: &Vec<usize>) -> Self;

    // temp
    fn init_state(&self);
}

/// Indexing methods for Index(Row)
pub trait RowIndexer: Sized {

    type Key;
    type Row;

    fn len(&self) -> usize;
    fn reindex(&self, labels: &Vec<Self::Key>) -> Self;
    fn reindex_by_index(&self, locations: &Vec<usize>) -> Self;

    // selection

    /// Get a single value corresponding to given label (slice by LOCation)
    fn loc(&self, label: &Self::Key) -> Self::Row;

    /// Get a single value corresponding to given index (slice by Index LOCation)
    fn iloc(&self, location: &usize) -> Self::Row;

    /// Slice using given labels (slice by LOCationS)
    fn locs(&self, labels: &Vec<Self::Key>) -> Self {
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
pub trait ColIndexer: Sized {

    type Key;
    type Column;

    /// Get column using label
    fn get(&self, label: &Self::Key) -> Self::Column;

    /// Get column using given index
    fn iget(&self, label: &usize) -> Self::Column;

    /// Slice columns using labels
    fn gets(&self, labels: &Vec<Self::Key>) -> Self;

    /// Slice columns given indices
    fn igets(&self, locations: &Vec<usize>) -> Self;
}

////////////////////////////////////////////////////////////////////////////////
// Reshaping
////////////////////////////////////////////////////////////////////////////////

/// Concatenate along row
pub trait Appender: Sized {
    fn append(&self, other: &Self) -> Self;
}

/// Concatenate along columns
pub trait Concatenator: Sized {
    fn concat(&self, other: &Self) -> Self;
}

/// Join by index
pub trait Joiner: Sized {
    fn join_inner(&self, other: &Self) -> Self;
}

////////////////////////////////////////////////////////////////////////////////
// Apply
////////////////////////////////////////////////////////////////////////////////

pub trait Applicable<T, U, V> {

    // T: Type for myself
    // R: Type function returns
    // C: Type of container which can hold W as values

    fn apply(&self, func: &Fn(&T) -> U) -> V;
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