//! A library aiming at facilitating handling of large amounts of data.

#![deny(missing_docs)]

#[macro_use]
extern crate quick_error;

pub mod error;
pub mod entry;
pub mod series;
pub mod dataframe;

pub use entry::{DataEntry, DataType};
pub use series::Series;
