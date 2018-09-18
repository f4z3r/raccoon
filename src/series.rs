//! A vector-like type that allows for aggregate operations similar to python's `pandas.Series`.
//!
// TODO: add info on performance.
//!
//! # Examples
//! You can create a new `Series` using `new`:
//! ```
//! use raccoon::{Series, DataType};
//!
//! // create a series with name "Name" containing integers (defaults to `i32`)
//! let series = Series::new("Name".to_owned(), DataType::Integer);
//! ```
//!
//! Alternatively, you can create a `Series` with data using a vector:
//! ```
//! use raccoon::{Series, DataEntry};
//!
//! let v = vec![true, false, true];
//! let series = Series::from_vector("bools".to_owned(), v);
//!
//! // data type is infered from data passed to it
//! assert_eq!(series[1usize], DataEntry::Boolean(false));
//! ```
//!
//! You can also push new values onto the end of a `Series`:
//! ```
//! use raccoon::{Series, DataEntry, DataType};
//!
//! let mut series = Series::from(vec![0u32, 1u32, 2u32]);
//! assert_eq!(series.data_type(), &DataType::UInteger);
//!
//! let result = series.push(3u32);
//! assert!(result.is_ok());
//! assert_eq!(series[3usize], DataEntry::UInteger(3u32));
//!
//! let result = series.push(false);
//! assert!(result.is_err());
//!
//! // you can also push vectors
//! let _ = series.push_vec(vec![4u32, 5u32, 6u32]);
//! assert_eq!(series[5usize], DataEntry::UInteger(5u32));
//!
//! // or push `DataEntry`s
//! let _ = series.push_entry(DataEntry::UInteger(7u32));
//! let _ = series.push_entry_vec(vec![DataEntry::UInteger(8u32), DataEntry::UInteger(9u32)]);
//! assert_eq!(series[9usize], DataEntry::UInteger(9u32));
//! ```


use entry::{DataEntry, DataType};
use error::{RaccoonResult, RaccoonError};

use std::ops::Index;

/// A series object.
#[derive(Debug, Clone)]
pub struct Series {
    name: String,
    entries: Vec<DataEntry>,
    data_type: DataType,
}

impl Series {
    /// Constructor. This creates an empty series of the given data type.
    ///
    /// # Arguments
    /// - `name`: the name of the series.
    /// - `data_type`: the data type of the series.
    ///
    /// # Returns
    /// A empty `Series` object.
    pub fn new(name: String, data_type: DataType) -> Series {
        Series {
            name: name,
            entries: Vec::new(),
            data_type: data_type
        }
    }

    /// Append a data entry to the series. This returns an error if the data type of the entry does not match the
    /// series' data type. This uses type inference and might fail if the wrong internal type is used. For instance
    /// passing `1` to a series using `DataType::UInteger` will fail as the argument has type `i32` and not `u32`.
    ///
    /// # Arguments
    /// - `data`: the data to append to the series.
    ///
    /// # Returns
    /// A `RaccoonResult`.
    pub fn push<T>(&mut self, data: T) -> RaccoonResult where T: Into<DataEntry> {
        let data_entry: DataEntry = data.into();
        self.push_entry(data_entry)
    }

    /// Append a data vector to the series. This returns an error if the data type of the entry does not match the
    /// series' data type. This uses type inference and might fail if the wrong internal type is used. For instance
    /// passing `vec![1]` to a series using `DataType::UInteger` will fail as the argument has type `i32` and not `u32`.
    ///
    /// # Arguments
    /// - `vector`: data vector to append to the series.
    ///
    /// # Returns
    /// A `RaccoonResult`.
    pub fn push_vec<T>(&mut self, vector: Vec<T>) -> RaccoonResult where T: Into<DataEntry> {
        let entries: Vec<DataEntry> = vector.into_iter().map(|x| x.into()).collect();
        self.push_entry_vec(entries)
    }

    /// Append a data entry to the series. This returns an error if the data type of the entry does not match the
    /// series' data type.
    ///
    /// # Arguments
    /// - `data_entry`: the data entry to append to the series.
    ///
    /// # Returns
    /// A `RaccoonResult`.
    pub fn push_entry(&mut self, data_entry: DataEntry) -> RaccoonResult {
        if !self.verify_type(data_entry.data_type()) {
            return Err(RaccoonError::InvalidType);
        }
        self.entries.push(data_entry);
        Ok(())
    }

    /// Same as `push_entry` but takes a vector as an argument.
    ///
    /// # Arguments
    /// - `vector`: the vector of `DataEntry` you want to append to the series.
    ///
    /// # Returns
    /// A `RaccoonResult`.
    pub fn push_entry_vec(&mut self, vector: Vec<DataEntry>) -> RaccoonResult {
        if vector.iter().any(|ref x| !self.verify_type(x.data_type())) {
            return Err(RaccoonError::InvalidType);
        }
        for item in vector {
            self.entries.push(item);
        }
        Ok(())
    }

    /// Converts the series into another data type.
    ///
    /// # Arguments
    /// - `data_type`: the desired data type of the series.
    pub fn convert_to(&mut self, data_type: &DataType) {
        let mut converted_entries: Vec<DataEntry> = Vec::new();
        for entry in &self.entries {
            converted_entries.push(entry.convert_to(data_type));
        }
        self.entries = converted_entries;
        self.data_type = data_type.clone();
    }

    /// Getter for the series' data type.
    ///
    /// # Returns
    /// A reference to the `DataType` of the series.
    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }

    /// Verifies the validity of the datatype. This checks if a given data type is conform to this series.
    ///
    /// # Arguments
    /// - `data_type`: the data type we want to check for validity.
    ///
    /// # Returns
    /// A `bool` indicating the validity (i.e. if `true`, entries of that type can be safely added to the series; if
    /// `false`, it is not safe).
    fn verify_type(&self, data_type: DataType) -> bool {
        if data_type != self.data_type && data_type != DataType::NA {
            false
        } else {
            true
        }
    }

    /// Builds a `Series` from a vector of items.
    ///
    /// # Arguments
    /// - `name`: the name of the series.
    /// - `vector`: vector containing the data.
    ///
    /// # Returns
    /// A `Series` object.
    pub fn from_vector<T>(name: String, vector: Vec<T>) -> Series where T: Into<DataEntry> {
        let entries: Vec<DataEntry> = vector.into_iter().map(|x| x.into()).collect();
        let mut data_type = DataType::NA;
        if !entries.is_empty() {
            data_type = entries[0].data_type().clone();
        }
        Series {
            name: name,
            entries: entries,
            data_type: data_type,
        }
    }

    /// Getter for the series' name.
    ///
    /// # Returns
    /// A reference to a string.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Setter for the series' name.
    ///
    /// # Arguments
    /// - `name`: the new name for the series.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl<T> From<Vec<T>> for Series where T: Into<DataEntry> {
    fn from(vector: Vec<T>) -> Self {
        Series::from_vector("Series1".to_owned(), vector)
    }
}

impl Index<usize> for Series {
    type Output = DataEntry;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.entries[idx]
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_series() {
        let name = String::from("Dog breeds");
        let mut series = Series::new(name, DataType::Text);
        let result = series.push_entry(DataEntry::Text("Labrador".to_owned()));
        assert!(result.is_ok());
        let result = series.push_entry(DataEntry::NA);
        assert!(result.is_ok());
        let result = series.push_entry(DataEntry::Text("Golden retriever".to_owned()));
        assert!(result.is_ok());
        let result = series.push_entry(DataEntry::Integer(25));
        assert!(result.is_err());
        assert_eq!("Dog breeds", series.name());
    }

    #[test]
    fn convert_series() {
        let name = String::from("Numbers");
        let mut series = Series::new(name, DataType::Text);
        let items = vec![
            DataEntry::Text("1".to_owned()),
            DataEntry::Text("2".to_owned()),
            DataEntry::Text("3".to_owned()),
            DataEntry::Text("".to_owned()),
            DataEntry::Text("4".to_owned())
        ];
        let result = series.push_entry_vec(items);
        assert!(result.is_ok());
        series.convert_to(&DataType::UInteger);
        assert_eq!("Numbers", series.name());
        assert_eq!(&DataType::UInteger, series.data_type());
        assert_eq!(DataEntry::UInteger(1u32), series[0usize]);
        assert_eq!(DataEntry::UInteger(2u32), series[1usize]);
        assert_eq!(DataEntry::UInteger(3u32), series[2usize]);
        assert_eq!(DataEntry::NA, series[3usize]);
        assert_eq!(DataEntry::UInteger(4u32), series[4usize]);
    }

    #[test]
    fn push_raw_entries() {
        let mut series = Series::new("name".to_owned(), DataType::Integer);
        let result = series.push_vec(vec![0, 1, 2]);
        assert!(result.is_ok());
        assert_eq!(DataEntry::Integer(0i32), series[0usize]);
        assert_eq!(DataEntry::Integer(1i32), series[1usize]);
        assert_eq!(DataEntry::Integer(2i32), series[2usize]);
        let result = series.push(3);
        assert!(result.is_ok());
        assert_eq!(DataEntry::Integer(3i32), series[3usize]);
        let result = series.push(true);
        assert!(result.is_err());
    }

    #[test]
    fn construction_from_vector() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7];
        let series = Series::from_vector("Some series".to_owned(), vec);
        assert_eq!("Some series", series.name());
        assert_eq!(&DataType::Integer, series.data_type());
        assert_eq!(DataEntry::Integer(1), series[0usize]);
        assert_eq!(DataEntry::Integer(2), series[1usize]);
        assert_eq!(DataEntry::Integer(3), series[2usize]);
        assert_eq!(DataEntry::Integer(4), series[3usize]);
        assert_eq!(DataEntry::Integer(5), series[4usize]);
        assert_eq!(DataEntry::Integer(6), series[5usize]);
        assert_eq!(DataEntry::Integer(7), series[6usize]);

        let vec: Vec<u64> = Vec::new();
        let series = Series::from_vector("empty".to_owned(), vec);
        assert_eq!("empty", series.name());
        assert_eq!(&DataType::NA, series.data_type());
    }

    #[test]
    #[should_panic(expected="index out of bounds: the len is 0 but the index is 0")]
    fn empty_indexing() {
        let vec: Vec<u64> = Vec::new();
        let series = Series::from(vec);
        let _ = &series[0_usize];
    }

    #[test]
    fn construction_from_trait() {
        let vec = vec![true, false, false, true, false, true, true];
        let mut series = Series::from(vec);
        assert_eq!("Series1", series.name());
        series.set_name("My new name".to_owned());
        assert_eq!("My new name", series.name());
        assert_eq!(&DataType::Boolean, series.data_type());
        assert_eq!(DataEntry::Boolean(true), series[0usize]);
        assert_eq!(DataEntry::Boolean(false), series[1usize]);
        assert_eq!(DataEntry::Boolean(false), series[2usize]);
        assert_eq!(DataEntry::Boolean(true), series[3usize]);
        assert_eq!(DataEntry::Boolean(false), series[4usize]);
        assert_eq!(DataEntry::Boolean(true), series[5usize]);
        assert_eq!(DataEntry::Boolean(true), series[6usize]);
    }
}
