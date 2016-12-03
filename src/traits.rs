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
pub trait RowIndexer<'s>: Sized {

    // 's: lifetime of myself

    type Key;
    type Row;

    fn len(&'s self) -> usize;
    fn reindex<'l>(&'s self, labels: &'l Vec<Self::Key>) -> Self;
    fn reindex_by_index<'l>(&'s self, locations: &'l Vec<usize>) -> Self;

    // selection

    /// Get a single value corresponding to given label (slice by LOCation)
    fn loc<'l>(&'s self, label: &'l Self::Key) -> Self::Row;

    /// Get a single value corresponding to given index (slice by Index LOCation)
    fn iloc<'l>(&'s self, location: &'l usize) -> Self::Row;

    /// Slice using given labels (slice by LOCationS)
    fn locs<'l>(&'s self, labels: &'l Vec<Self::Key>) -> Self {
        self.reindex(labels)
    }

    /// Slice using given indices (slice by Index LOCationS)
    fn ilocs<'l>(&'s self, locations: &'l Vec<usize>) -> Self {
        self.reindex_by_index(locations)
    }

    /// Slice using given Vec<bool> (slice by Bool LOCationS)
    fn blocs<'l>(&'s self, flags: &'l Vec<bool>) -> Self;
}

/// Indexing methods for Columns
pub trait ColIndexer<'s>: Sized {

    // 's: lifetime of myself

    type Key;
    type Column;

    /// Get column using label
    fn get<'l>(&'s self, label: &'l Self::Key) -> Self::Column;

    /// Get column using given index
    fn iget<'l>(&'s self, label: &'l usize) -> Self::Column;

    /// Slice columns using labels
    fn gets<'l>(&'s self, labels: &'l Vec<Self::Key>) -> Self;

    /// Slice columns given indices
    fn igets<'l>(&'s self, locations: &'l Vec<usize>) -> Self;
}

////////////////////////////////////////////////////////////////////////////////
// Reshaping
////////////////////////////////////////////////////////////////////////////////

/// Concatenate along row
pub trait Appender<'s>: Sized {
    fn append<'o>(&'s self, other: &'o Self) -> Self;
}

/// Concatenate along columns
pub trait Concatenator<'s>: Sized {
    fn concat<'o>(&'s self, other: &'o Self) -> Self;
}

/// Join by index
pub trait Joiner: Sized {
    fn join_inner(&self, other: &Self) -> Self;
}

////////////////////////////////////////////////////////////////////////////////
// Apply
////////////////////////////////////////////////////////////////////////////////

pub trait Applicable<'s, T, U, V> {

    // T: Type for myself
    // R: Type function returns
    // C: Type of container which can hold W as values

    fn apply<'f>(&'s self, func: &'f Fn(&T) -> U) -> V;
}

////////////////////////////////////////////////////////////////////////////////
// Aggregation
////////////////////////////////////////////////////////////////////////////////

pub trait Aggregator<'s, 'r> {

    // 'r: lifetime of result

    // result which can keep current dtype
    type Kept;
    // result for count (to usize or its container)
    type Counted;
    // result which is coerced (to f64 or its container)
    type Coerced;

    fn sum(&'s self) -> Self::Kept;
    fn count(&'s self) -> Self::Counted;
    fn mean(&'s self) -> Self::Coerced;
    fn var(&'s self) -> Self::Coerced;
    fn unbiased_var(&'s self) -> Self::Coerced;
    fn std(&'s self) -> Self::Coerced;
    fn unbiased_std(&'s self) -> Self::Coerced;
}