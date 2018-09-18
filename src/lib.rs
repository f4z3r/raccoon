//! A library aiming at facilitating handling of large amounts of data.
//!
//! # ATTENTION
//! This is a work in progress is **not at all stable**. I am a student developing this as a hobby and to get a better
//! grasp of Rust.
//!
//! Use in production code at your own risk.

#![deny(missing_docs)]

#[macro_use]
extern crate quick_error;

pub mod error;
pub mod entry;
pub mod series;
pub mod dataframe;

pub use entry::{DataEntry, DataType};
pub use series::Series;
pub use error::{RaccoonError, RaccoonResult};
