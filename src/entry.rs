//! `entry` module. Contains all code related to data entries.

/// A data entry.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DataEntry {
    /// A text entry.
    Text(String),
    /// An integer entry (default for whole decimals).
    Integer(i32),
    /// An unsigned integer entry (default for whole unsigned decimals).
    UInteger(u32),
    /// A long entry.
    Long(i64),
    /// An unsigned long entry.
    ULong(u64),
    /// A floating point entry (default for decimal entries).
    Float(f32),
    /// A double precision floating point entry.
    Double(f64),
    /// A boolean entry.
    Boolean(bool),
    /// A character entry.
    Character(char)
}

impl DataEntry {
    /// Retrieves the internal type of the entry. This can be one of the following:
    ///
    /// - `String`,
    /// - `i32`,
    /// - `u32`,
    /// - `i64`,
    /// - `u64`,
    /// - `f32`,
    /// - `f64`,
    /// - `bool`,
    /// - `char`.
    ///
    /// # Returns
    /// One of the above as a string reference.
    pub fn get_internal_type(&self) -> &str {
        match self {
            DataEntry::Text(_)      => "String",
            DataEntry::Integer(_)   => "i32",
            DataEntry::UInteger(_)  => "u32",
            DataEntry::Long(_)      => "i64",
            DataEntry::ULong(_)     => "u64",
            DataEntry::Float(_)     => "f32",
            DataEntry::Double(_)    => "f64",
            DataEntry::Boolean(_)   => "bool",
            DataEntry::Character(_) => "char",
        }
    }
}


/// The data type any entry can take.
#[derive(Debug)]
pub enum DataType {
    /// Text
    Text,
    /// Integer
    Integer,
    /// Unsigned integer
    UInteger,
    /// Long
    Long,
    /// Unsigned long
    ULong,
    /// Floating point number
    Float,
    /// Double precision floating point number
    Double,
    /// Boolean
    Boolean,
    /// Character
    Character
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getting_types() {
        let entry = DataEntry::Text("Some text".to_owned());
        assert_eq!("String", entry.get_internal_type());

        let entry = DataEntry::Integer(-353);
        assert_eq!("i32", entry.get_internal_type());

        let entry = DataEntry::UInteger(454);
        assert_eq!("u32", entry.get_internal_type());

        let entry = DataEntry::Long(-123_456_789);
        assert_eq!("i64", entry.get_internal_type());

        let entry = DataEntry::ULong(987_654_321);
        assert_eq!("u64", entry.get_internal_type());

        let entry = DataEntry::Float(-90.345);
        assert_eq!("f32", entry.get_internal_type());

        let entry = DataEntry::Double(-34532.34);
        assert_eq!("f64", entry.get_internal_type());

        let entry = DataEntry::Boolean(true);
        assert_eq!("bool", entry.get_internal_type());

        let entry = DataEntry::Character('a');
        assert_eq!("char", entry.get_internal_type());
    }

    #[test]
    fn ordering() {
        let a = DataEntry::Long(-234_567);
        let b = DataEntry::Long(-234_567);
        let c = DataEntry::Long(-234_568);
        assert!(a == b);
        assert!(a != c);
        assert!(b != c);
        assert!(a > c);
        assert!(!(b <= c));

        let a = DataEntry::Float(23.45);
        let b = DataEntry::Float(23.45);
        let c = DataEntry::Float(23.455);
        assert!(a == b);
        assert!(a != c);
        assert!(b != c);
        assert!(a < c);
        assert!(!(b >= c));

        let a = DataEntry::Text("hello".to_owned());
        let b = DataEntry::Text("hello".to_owned());
        let c = DataEntry::Text("world".to_owned());
        assert!(a == b);
        assert!(a != c);
        assert!(b != c);
        assert!(a < c);
        assert!(!(b >= c));

        let a = DataEntry::Character('a');
        let b = DataEntry::Character('a');
        let c = DataEntry::Character('c');
        assert!(a == b);
        assert!(a != c);
        assert!(b != c);
        assert!(a < c);
        assert!(!(b >= c));
    }
}
