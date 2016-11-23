extern crate itertools;
extern crate num;

mod algos;
mod block;
mod computations;
mod eval;
mod formatting;
mod frame;
mod indexer;
mod internals;
mod series;
mod traits;

pub use block::Block;
pub use indexer::Indexer;
pub use series::Series;
pub use traits::{RowIndexer, ColIndexer};
