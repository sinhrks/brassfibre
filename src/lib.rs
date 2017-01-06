extern crate csv;
extern crate num;
// extern crate rayon;
extern crate rustc_serialize;

mod algos;
mod block;
mod formatting;
mod frame;
mod groupby;
mod indexer;
mod internals;
mod io;
#[macro_use]
mod macros;
mod series;
mod traits;

pub mod prelude;
