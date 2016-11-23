//! Common Traits

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