//! Dataframe module.

use prelude::*;
use std::collections::HashMap;

/// A strictly type checked dataframe.
///
///
#[derive(Debug)]
pub struct DataFrame {
    index: Series,
    series: HashMap<String, Series>,
}

impl DataFrameLike for DataFrame {
    fn new() -> Self {
        DataFrame {
            index: Series::new_empty("index", DType::UInt),
            series: HashMap::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.index.is_empty()
    }
}


/// Common functionality for dataframe-like objects.
pub trait DataFrameLike {
    /// Builds an empty dataframe.
    ///
    /// # Example
    /// ```
    /// # use raccoon::prelude::*;
    /// let df = DataFrame::new();
    /// assert!(df.is_empty());
    /// ```
    fn new() -> Self;

    /// Checks if the dataframe is empty.
    ///
    /// A dataframe is considered empty if it contains no records. Note this is not the same as not containing any
    /// series. In fact, a dataframe is empty if all series it contains are empty.
    ///
    /// # Example
    /// ```
    /// # use raccoon::prelude::*;
    /// let df = DataFrame::new();
    /// assert!(df.is_empty());
    /// ```
    fn is_empty(&self) -> bool;
}
