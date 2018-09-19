//! Library providing fast data processing.
//!
//! This aims at providing functionality similar to python's `pandas` module.

#![deny(missing_docs)]

#[macro_use] mod macros;

pub mod dataframe;
pub mod cell;
pub mod series;
pub mod traits;
pub mod error;
pub mod prelude;

