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
        if !self.verify_type(data_entry.get_type()) {
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
        if entries.iter().any(|ref x| !self.verify_type(x.get_type())) {
            return Err(RaccoonError::InvalidType);
        }
        for item in entries {
            self.entries.push(item);
        }
        Ok(())
    }

    /// Parses the datatype of the series to become the datatype given in the arguments.
    ///
    /// # Args
    /// - `data_type`: the desired data type of the series.
    pub fn parse(&mut self, data_type: &DataType) {
        let mut parsed_entries: Vec<DataEntry> = Vec::new();
        for entry in &self.entries {
            parsed_entries.push(entry.parse(data_type));
        }
        self.entries = parsed_entries;
    }

    /// Getter for the series' data type.
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
        let name = String::from("Dog breads");
        let mut series = Series::new(name, DataType::Text);
        let result = series.push(DataEntry::Text("Labrador".to_owned()));
        assert!(result.is_ok());
        let result = series.push(DataEntry::NA);
        assert!(result.is_ok());
        let result = series.push(DataEntry::Text("Golden retriever".to_owned()));
        assert!(result.is_ok());
        let result = series.push(DataEntry::Integer(25));
        assert!(result.is_err());
    }

    #[test]
    fn parse_series() {
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
        series.parse(&DataType::UInteger);
        assert_eq!(DataEntry::UInteger(1u32), series[0usize]);
    }
}
