extern crate itertools;
extern crate num;

mod algos;
mod block;
mod computations;
mod formatting;
mod frame;
mod indexer;
mod internals;
#[macro_use]
mod macros;
mod series;
mod traits;

pub use block::Block;
pub use indexer::Indexer;
pub use internals::Array;
pub use series::Series;
pub use traits::{IndexerIndexer, RowIndexer, ColIndexer, Appender,
                 Applicable, Aggregator};
