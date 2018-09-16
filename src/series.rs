//! `series` module.

use entry;

/// A series object.
pub struct Series {
    name: String,
    entries: Vec<entry::DataEntry>,
    data_type: entry::DataType,
}
