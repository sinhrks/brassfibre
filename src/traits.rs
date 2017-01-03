//! Common Traits

////////////////////////////////////////////////////////////////////////////////
// Indexing
////////////////////////////////////////////////////////////////////////////////

/// Indexing methods for non-labeled Array / Indexer
pub trait Slicer: Sized {
    fn len(&self) -> usize;
    fn ilocs(&self, locations: &[usize]) -> Self;
    unsafe fn ilocs_unchecked(&self, locations: &[usize]) -> Self;
    fn blocs(&self, flags: &[bool]) -> Self;

    fn reindex(&self, locations: &[usize]) -> Self {
        self.ilocs(locations)
    }
}

/// Indexing methods for Indexer
pub trait IndexerIndex: Slicer {

    type Key;

    fn contains(&self, label: &Self::Key) -> bool;
    fn push(&mut self, label: Self::Key);
    fn get_loc(&self, label: &Self::Key) -> usize;
    fn get_locs(&self, labels: &[Self::Key]) -> Vec<usize>;

    // temp
    fn init_state(&self);
}

/// Indexing methods for Index(Row)
pub trait RowIndex<'s>: Sized {

    // 's: lifetime of myself

    type Key;
    type Row;

    fn len(&'s self) -> usize;

    fn head(&'s self, n: usize) -> Self {
        let indexer: Vec<usize> = (0..n).collect();
        self.ilocs(&indexer)
    }

    fn tail(&'s self, n: usize) -> Self {
        let len = self.len();
        let indexer: Vec<usize> = ((len - n)..len).collect();
        self.ilocs(&indexer)
    }

    fn reindex<'l>(&'s self, labels: &'l [Self::Key]) -> Self;
    fn reindex_by_index<'l>(&'s self, locations: &'l [usize]) -> Self;

    // selection

    /// Get a single value corresponding to given label (slice by LOCation)
    fn loc<'l>(&'s self, label: &'l Self::Key) -> Self::Row;

    /// Get a single value corresponding to given index (slice by Index LOCation)
    fn iloc<'l>(&'s self, location: &'l usize) -> Self::Row;

    /// Slice using given labels (slice by LOCationS)
    fn locs<'l>(&'s self, labels: &'l [Self::Key]) -> Self {
        self.reindex(labels)
    }

    /// Slice using given indices (slice by Index LOCationS)
    fn ilocs<'l>(&'s self, locations: &'l [usize]) -> Self {
        self.reindex_by_index(locations)
    }

    /// Slice using given Vec<bool> (slice by Bool LOCationS)
    fn blocs<'l>(&'s self, flags: &'l [bool]) -> Self;
}

/// Indexing methods for Columns
pub trait ColIndex<'s>: Sized {

    // 's: lifetime of myself

    type Key;
    type Column;

    /// Get column using label
    fn get<'l>(&'s self, label: &'l Self::Key) -> Self::Column;

    /// Get column using given index
    fn iget<'l>(&'s self, label: &'l usize) -> Self::Column;

    /// Slice columns using labels
    fn gets<'l>(&'s self, labels: &'l [Self::Key]) -> Self;

    /// Slice columns given indices
    fn igets<'l>(&'s self, locations: &'l [usize]) -> Self;

    // ToDo: Add .insert
    // ToDo bgets
}

////////////////////////////////////////////////////////////////////////////////
// Reshaping
////////////////////////////////////////////////////////////////////////////////

/// Concatenate along row
pub trait Append<'s>: Sized {
    fn append<'o>(&'s self, other: &'o Self) -> Self;
}

/// Concatenate along columns
pub trait Concatenation<'s>: Sized {
    fn concat<'o>(&'s self, other: &'o Self) -> Self;
}

/// Join by index
pub trait Join: Sized {
    fn join_inner(&self, other: &Self) -> Self;
}

////////////////////////////////////////////////////////////////////////////////
// Apply
////////////////////////////////////////////////////////////////////////////////

pub trait Apply<'s, R> {
    // R: Type function returns, dummy to avoid unconstrained lifetime parameter

    type In;
    type FOut;
    type Out;

    fn apply<'f>(&'s self, func: &'f Fn(&Self::In) -> Self::FOut) -> Self::Out;
}

////////////////////////////////////////////////////////////////////////////////
// Aggregation
////////////////////////////////////////////////////////////////////////////////

pub trait BasicAggregation<'s> {

    // result which can keep current dtype
    type Kept;
    // result for count (to usize or its container)
    type Counted;

    fn sum(&'s self) -> Self::Kept;
    fn count(&'s self) -> Self::Counted;
}

pub trait NumericAggregation<'s> {

    // result which is coerced (to f64 or its container)
    type Coerced;

    fn mean(&'s self) -> Self::Coerced;
    fn var(&'s self) -> Self::Coerced;
    fn unbiased_var(&'s self) -> Self::Coerced;
    fn std(&'s self) -> Self::Coerced;
    fn unbiased_std(&'s self) -> Self::Coerced;
}

pub trait ComparisonAggregation<'s> {

    type Kept;

    fn min(&'s self) -> Self::Kept;
    fn max(&'s self) -> Self::Kept;
}

pub trait Description<'s>: BasicAggregation<'s> +
                           NumericAggregation<'s> +
                           ComparisonAggregation<'s> {

    type Described;

    fn describe(&'s self) -> Self::Described;
}
