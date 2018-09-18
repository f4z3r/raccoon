//! `series` module.

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
    /// # Args
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
    /// series' data type.
    ///
    /// # Args
    /// - `data_entry`: the data entry to append to the series.
    ///
    /// # Returns
    /// A `RaccoonResult`.
    pub fn push(&mut self, data_entry: DataEntry) -> RaccoonResult {
        if !self.verify_type(data_entry.data_type()) {
            return Err(RaccoonError::InvalidType);
        }
        self.entries.push(data_entry);
        Ok(())
    }

    /// Same as `push` but takes a vector as an argument.
    ///
    /// # Args
    /// - `entries`: the vector of `DataEntry` you want to append to the series.
    ///
    /// # Returns
    /// A `RaccoonResult`.
    pub fn push_vec(&mut self, entries: Vec<DataEntry>) -> RaccoonResult {
        if entries.iter().any(|ref x| !self.verify_type(x.data_type())) {
            return Err(RaccoonError::InvalidType);
        }
        for item in entries {
            self.entries.push(item);
        }
        Ok(())
    }

    /// Converts the series into another data type.
    ///
    /// # Args
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
    /// # Args
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
    /// # Args
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
        let result = series.push(DataEntry::Text("Labrador".to_owned()));
        assert!(result.is_ok());
        let result = series.push(DataEntry::NA);
        assert!(result.is_ok());
        let result = series.push(DataEntry::Text("Golden retriever".to_owned()));
        assert!(result.is_ok());
        let result = series.push(DataEntry::Integer(25));
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
        let result = series.push_vec(items);
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
        let series = Series::from(vec);
        assert_eq!("Series1", series.name());
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
