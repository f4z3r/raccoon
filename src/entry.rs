//! `entry` module. Contains all code related to data entries.

use std::ops::{Add, Sub};

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
    Character(char),
    /// Invalid entry.
    Invalid(String)
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
    /// - invalid.
    ///
    /// # Returns
    /// One of the above as a string reference.
    pub fn internal_type(&self) -> &str {
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
            DataEntry::Invalid(_)   => "invalid",
        }
    }
}

impl Add for DataEntry {
    type Output = DataEntry;

    fn add(self, other: DataEntry) -> Self::Output {
        match self {
            DataEntry::Integer(int1)    => {
                match other {
                    DataEntry::Integer(int2)    => DataEntry::Integer(int1 + int2),
                    DataEntry::UInteger(int2)   => DataEntry::Long(int1 as i64 + int2 as i64),
                    DataEntry::Long(int2)       => DataEntry::Long(int1 as i64 + int2),
                    DataEntry::ULong(int2)      => DataEntry::Double(int1 as f64 + int2 as f64),
                    DataEntry::Float(f2)        => DataEntry::Float(int1 as f32 + f2),
                    DataEntry::Double(f2)       => DataEntry::Double(int1 as f64 + f2),
                    DataEntry::Boolean(b2)      => DataEntry::Integer(int1 + b2 as i32),
                    DataEntry::Character(ch1)   => DataEntry::Text(int1.to_string() + &ch1.to_string()),
                    DataEntry::Text(txt2)       => DataEntry::Text(int1.to_string() + &txt2),
                    DataEntry::Invalid(error)   => DataEntry::Invalid(error.clone())
                }
            },
            DataEntry::UInteger(int1)   => {
                match other {
                    DataEntry::Integer(int2)    => DataEntry::Long(int1 as i64 + int2 as i64),
                    DataEntry::UInteger(int2)   => DataEntry::UInteger(int1 + int2),
                    DataEntry::Long(int2)       => DataEntry::Long(int1 as i64 + int2),
                    DataEntry::ULong(int2)      => DataEntry::Double(int1 as f64 + int2 as f64),
                    DataEntry::Float(f2)        => DataEntry::Float(int1 as f32 + f2),
                    DataEntry::Double(f2)       => DataEntry::Double(int1 as f64 + f2),
                    DataEntry::Boolean(b2)      => DataEntry::UInteger(int1 + b2 as u32),
                    DataEntry::Character(ch1)   => DataEntry::Text(int1.to_string() + &ch1.to_string()),
                    DataEntry::Text(txt2)       => DataEntry::Text(int1.to_string() + &txt2),
                    DataEntry::Invalid(error)   => DataEntry::Invalid(error.clone())
                }
            },
            DataEntry::Long(int1)       => {
                match other {
                    DataEntry::Integer(int2)    => DataEntry::Long(int1 as i64 + int2 as i64),
                    DataEntry::UInteger(int2)   => DataEntry::Long(int1 as i64 + int2 as i64),
                    DataEntry::Long(int2)       => DataEntry::Long(int1 + int2),
                    DataEntry::ULong(int2)      => DataEntry::Double(int1 as f64 + int2 as f64),
                    DataEntry::Float(f2)        => DataEntry::Float(int1 as f32 + f2),
                    DataEntry::Double(f2)       => DataEntry::Double(int1 as f64 + f2),
                    DataEntry::Boolean(b2)      => DataEntry::Long(int1 + b2 as i64),
                    DataEntry::Character(ch1)   => DataEntry::Text(int1.to_string() + &ch1.to_string()),
                    DataEntry::Text(txt2)       => DataEntry::Text(int1.to_string() + &txt2),
                    DataEntry::Invalid(error)   => DataEntry::Invalid(error.clone())
                }
            },
            DataEntry::ULong(int1)      => {
                match other {
                    DataEntry::Integer(int2)    => DataEntry::Double(int1 as f64 + int2 as f64),
                    DataEntry::UInteger(int2)   => DataEntry::ULong(int1 + int2 as u64),
                    DataEntry::Long(int2)       => DataEntry::Double(int1 as f64 + int2 as f64),
                    DataEntry::ULong(int2)      => DataEntry::ULong(int1 + int2),
                    DataEntry::Float(f2)        => DataEntry::Double(int1 as f64 + f2 as f64),
                    DataEntry::Double(f2)       => DataEntry::Double(int1 as f64 + f2),
                    DataEntry::Boolean(b2)      => DataEntry::ULong(int1 + b2 as u64),
                    DataEntry::Character(ch1)   => DataEntry::Text(int1.to_string() + &ch1.to_string()),
                    DataEntry::Text(txt2)       => DataEntry::Text(int1.to_string() + &txt2),
                    DataEntry::Invalid(error)   => DataEntry::Invalid(error.clone())
                }
            },
            DataEntry::Float(f1)        => {
                match other {
                    DataEntry::Integer(int2)    => DataEntry::Float(f1 + int2 as f32),
                    DataEntry::UInteger(int2)   => DataEntry::Float(f1 + int2 as f32),
                    DataEntry::Long(int2)       => DataEntry::Double(f1 as f64 + int2 as f64),
                    DataEntry::ULong(int2)      => DataEntry::Double(f1 as f64 + int2 as f64),
                    DataEntry::Float(f2)        => DataEntry::Float(f1 + f2),
                    DataEntry::Double(f2)       => DataEntry::Double(f1 as f64 + f2),
                    DataEntry::Boolean(b2)      => DataEntry::Float(f1 + b2 as u8 as f32),
                    DataEntry::Character(ch1)   => DataEntry::Text(f1.to_string() + &ch1.to_string()),
                    DataEntry::Text(txt2)       => DataEntry::Text(f1.to_string() + &txt2),
                    DataEntry::Invalid(error)   => DataEntry::Invalid(error.clone())
                }
            },
            DataEntry::Double(f1)       => {
                match other {
                    DataEntry::Integer(int2)    => DataEntry::Double(f1 + int2 as f64),
                    DataEntry::UInteger(int2)   => DataEntry::Double(f1 + int2 as f64),
                    DataEntry::Long(int2)       => DataEntry::Double(f1 + int2 as f64),
                    DataEntry::ULong(int2)      => DataEntry::Double(f1 + int2 as f64),
                    DataEntry::Float(f2)        => DataEntry::Double(f1 + f2 as f64),
                    DataEntry::Double(f2)       => DataEntry::Double(f1 + f2),
                    DataEntry::Boolean(b2)      => DataEntry::Double(f1 + b2 as u8 as f64),
                    DataEntry::Character(ch1)   => DataEntry::Text(f1.to_string() + &ch1.to_string()),
                    DataEntry::Text(txt2)       => DataEntry::Text(f1.to_string() + &txt2),
                    DataEntry::Invalid(error)   => DataEntry::Invalid(error.clone())
                }
            },
            DataEntry::Boolean(b1)      => {
                match other {
                    DataEntry::Integer(int2)    => DataEntry::Integer(b1 as i32 + int2),
                    DataEntry::UInteger(int2)   => DataEntry::UInteger(b1 as u32 + int2),
                    DataEntry::Long(int2)       => DataEntry::Long(b1 as i64 + int2),
                    DataEntry::ULong(int2)      => DataEntry::ULong(b1 as u64 + int2),
                    DataEntry::Float(f2)        => DataEntry::Float(b1 as u8 as f32 + f2),
                    DataEntry::Double(f2)       => DataEntry::Double(b1 as u8 as f64 + f2),
                    DataEntry::Boolean(b2)      => DataEntry::Integer(b1 as i32 + b2 as i32),
                    DataEntry::Character(ch1)   => DataEntry::Text(b1.to_string() + &ch1.to_string()),
                    DataEntry::Text(txt2)       => DataEntry::Text(b1.to_string() + &txt2),
                    DataEntry::Invalid(error)   => DataEntry::Invalid(error.clone())
                }
            },
            DataEntry::Character(ch1)   => {
                match other {
                    DataEntry::Integer(int2)    => DataEntry::Text(ch1.to_string() + &int2.to_string()),
                    DataEntry::UInteger(int2)   => DataEntry::Text(ch1.to_string() + &int2.to_string()),
                    DataEntry::Long(int2)       => DataEntry::Text(ch1.to_string() + &int2.to_string()),
                    DataEntry::ULong(int2)      => DataEntry::Text(ch1.to_string() + &int2.to_string()),
                    DataEntry::Float(f2)        => DataEntry::Text(ch1.to_string() + &f2.to_string()),
                    DataEntry::Double(f2)       => DataEntry::Text(ch1.to_string() + &f2.to_string()),
                    DataEntry::Boolean(b2)      => DataEntry::Text(ch1.to_string() + &b2.to_string()),
                    DataEntry::Character(ch1)   => DataEntry::Text(ch1.to_string() + &ch1.to_string()),
                    DataEntry::Text(txt2)       => DataEntry::Text(ch1.to_string() + &txt2),
                    DataEntry::Invalid(error)   => DataEntry::Invalid(error.clone())
                }
            },
            DataEntry::Text(txt1)       => {
                match other {
                    DataEntry::Integer(int2)    => DataEntry::Text(txt1 + &int2.to_string()),
                    DataEntry::UInteger(int2)   => DataEntry::Text(txt1 + &int2.to_string()),
                    DataEntry::Long(int2)       => DataEntry::Text(txt1 + &int2.to_string()),
                    DataEntry::ULong(int2)      => DataEntry::Text(txt1 + &int2.to_string()),
                    DataEntry::Float(f2)        => DataEntry::Text(txt1 + &f2.to_string()),
                    DataEntry::Double(f2)       => DataEntry::Text(txt1 + &f2.to_string()),
                    DataEntry::Boolean(b2)      => DataEntry::Text(txt1 + &b2.to_string()),
                    DataEntry::Character(ch1)   => DataEntry::Text(txt1 + &ch1.to_string()),
                    DataEntry::Text(txt2)       => DataEntry::Text(txt1 + &txt2),
                    DataEntry::Invalid(error)   => DataEntry::Invalid(error.clone())
                }
            },
            DataEntry::Invalid(error)   => DataEntry::Invalid(error.clone())
        }
    }
}

impl Sub for DataEntry {
    type Output = DataEntry;

    fn sub(self, other: DataEntry) -> Self::Output {
        match self {
            DataEntry::Integer(int1)    => {
                match other {
                    DataEntry::Integer(int2)    => DataEntry::Integer(int1 - int2),
                    DataEntry::UInteger(int2)   => DataEntry::Long(int1 as i64 - int2 as i64),
                    DataEntry::Long(int2)       => DataEntry::Long(int1 as i64 - int2),
                    DataEntry::ULong(int2)      => DataEntry::Double(int1 as f64 - int2 as f64),
                    DataEntry::Float(f2)        => DataEntry::Float(int1 as f32 - f2),
                    DataEntry::Double(f2)       => DataEntry::Double(int1 as f64 - f2),
                    DataEntry::Boolean(b2)      => DataEntry::Integer(int1 - b2 as i32),
                    DataEntry::Character(ch1)   => DataEntry::Invalid("subtracting characters is invalid".to_owned()),
                    DataEntry::Text(txt2)       => DataEntry::Invalid("subtracting strings is invalid".to_owned()),
                    DataEntry::Invalid(error)   => DataEntry::Invalid(error.clone())
                }
            },
            DataEntry::UInteger(int1)   => {
                match other {
                    DataEntry::Integer(int2)    => DataEntry::Long(int1 as i64 - int2 as i64),
                    DataEntry::UInteger(int2)   => DataEntry::Long(int1 as i64 - int2 as i64),
                    DataEntry::Long(int2)       => DataEntry::Long(int1 as i64 + int2),
                    DataEntry::ULong(int2)      => DataEntry::Double(int1 as f64 - int2 as f64),
                    DataEntry::Float(f2)        => DataEntry::Float(int1 as f32 - f2),
                    DataEntry::Double(f2)       => DataEntry::Double(int1 as f64 - f2),
                    DataEntry::Boolean(b2)      => DataEntry::Integer(int1 as i32 - b2 as i32),
                    DataEntry::Character(ch1)   => DataEntry::Invalid("subtracting characters is invalid".to_owned()),
                    DataEntry::Text(txt2)       => DataEntry::Invalid("subtracting strings is invalid".to_owned()),
                    DataEntry::Invalid(error)   => DataEntry::Invalid(error.clone())
                }
            },
            DataEntry::Long(int1)       => {
                match other {
                    DataEntry::Integer(int2)    => DataEntry::Long(int1 as i64 - int2 as i64),
                    DataEntry::UInteger(int2)   => DataEntry::Long(int1 as i64 - int2 as i64),
                    DataEntry::Long(int2)       => DataEntry::Long(int1 - int2),
                    DataEntry::ULong(int2)      => DataEntry::Double(int1 as f64 - int2 as f64),
                    DataEntry::Float(f2)        => DataEntry::Float(int1 as f32 - f2),
                    DataEntry::Double(f2)       => DataEntry::Double(int1 as f64 - f2),
                    DataEntry::Boolean(b2)      => DataEntry::Long(int1 - b2 as i64),
                    DataEntry::Character(ch1)   => DataEntry::Invalid("subtracting characters is invalid".to_owned()),
                    DataEntry::Text(txt2)       => DataEntry::Invalid("subtracting strings is invalid".to_owned()),
                    DataEntry::Invalid(error)   => DataEntry::Invalid(error.clone())
                }
            },
            DataEntry::ULong(int1)      => {
                match other {
                    DataEntry::Integer(int2)    => DataEntry::Double(int1 as f64 - int2 as f64),
                    DataEntry::UInteger(int2)   => DataEntry::Double(int1 as f64 - int2 as f64),
                    DataEntry::Long(int2)       => DataEntry::Double(int1 as f64 - int2 as f64),
                    DataEntry::ULong(int2)      => DataEntry::Double(int1 as f64 - int2 as f64),
                    DataEntry::Float(f2)        => DataEntry::Double(int1 as f64 - f2 as f64),
                    DataEntry::Double(f2)       => DataEntry::Double(int1 as f64 - f2),
                    DataEntry::Boolean(b2)      => DataEntry::Double(int1 as f64 - b2 as u8 as f64),
                    DataEntry::Character(ch1)   => DataEntry::Invalid("subtracting characters is invalid".to_owned()),
                    DataEntry::Text(txt2)       => DataEntry::Invalid("subtracting strings is invalid".to_owned()),
                    DataEntry::Invalid(error)   => DataEntry::Invalid(error.clone())
                }
            },
            DataEntry::Float(f1)        => {
                match other {
                    DataEntry::Integer(int2)    => DataEntry::Float(f1 - int2 as f32),
                    DataEntry::UInteger(int2)   => DataEntry::Float(f1 - int2 as f32),
                    DataEntry::Long(int2)       => DataEntry::Double(f1 as f64 - int2 as f64),
                    DataEntry::ULong(int2)      => DataEntry::Double(f1 as f64 - int2 as f64),
                    DataEntry::Float(f2)        => DataEntry::Float(f1 - f2),
                    DataEntry::Double(f2)       => DataEntry::Double(f1 as f64 - f2),
                    DataEntry::Boolean(b2)      => DataEntry::Float(f1 - b2 as u8 as f32),
                    DataEntry::Character(ch1)   => DataEntry::Invalid("subtracting characters is invalid".to_owned()),
                    DataEntry::Text(txt2)       => DataEntry::Invalid("subtracting strings is invalid".to_owned()),
                    DataEntry::Invalid(error)   => DataEntry::Invalid(error.clone())
                }
            },
            DataEntry::Double(f1)       => {
                match other {
                    DataEntry::Integer(int2)    => DataEntry::Double(f1 - int2 as f64),
                    DataEntry::UInteger(int2)   => DataEntry::Double(f1 - int2 as f64),
                    DataEntry::Long(int2)       => DataEntry::Double(f1 - int2 as f64),
                    DataEntry::ULong(int2)      => DataEntry::Double(f1 - int2 as f64),
                    DataEntry::Float(f2)        => DataEntry::Double(f1 - f2 as f64),
                    DataEntry::Double(f2)       => DataEntry::Double(f1 - f2),
                    DataEntry::Boolean(b2)      => DataEntry::Double(f1 - b2 as u8 as f64),
                    DataEntry::Character(ch1)   => DataEntry::Invalid("subtracting characters is invalid".to_owned()),
                    DataEntry::Text(txt2)       => DataEntry::Invalid("subtracting strings is invalid".to_owned()),
                    DataEntry::Invalid(error)   => DataEntry::Invalid(error.clone())
                }
            },
            DataEntry::Boolean(b1)      => {
                match other {
                    DataEntry::Integer(int2)    => DataEntry::Integer(b1 as i32 - int2),
                    DataEntry::UInteger(int2)   => DataEntry::UInteger(b1 as u32 - int2),
                    DataEntry::Long(int2)       => DataEntry::Long(b1 as i64 - int2),
                    DataEntry::ULong(int2)      => DataEntry::ULong(b1 as u64 - int2),
                    DataEntry::Float(f2)        => DataEntry::Float(b1 as u8 as f32 - f2),
                    DataEntry::Double(f2)       => DataEntry::Double(b1 as u8 as f64 - f2),
                    DataEntry::Boolean(b2)      => DataEntry::Integer(b1 as i32 - b2 as i32),
                    DataEntry::Character(ch1)   => DataEntry::Invalid("subtracting characters is invalid".to_owned()),
                    DataEntry::Text(txt2)       => DataEntry::Invalid("subtracting strings is invalid".to_owned()),
                    DataEntry::Invalid(error)   => DataEntry::Invalid(error.clone())
                }
            },
            DataEntry::Character(ch1)   => {
                match other {
                    _                           => DataEntry::Invalid("subtracting from characters is invalid".to_owned())
                }
            },
            DataEntry::Text(txt1)       => {
                match other {
                    _                           => DataEntry::Invalid("subtracting from strings is invalid".to_owned())
                }
            },
            DataEntry::Invalid(error)   => DataEntry::Invalid(error.clone())
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
    Character,
    /// Invalid
    Invalid
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getting_types() {
        let entry = DataEntry::Text("Some text".to_owned());
        assert_eq!("String", entry.internal_type());

        let entry = DataEntry::Integer(-353);
        assert_eq!("i32", entry.internal_type());

        let entry = DataEntry::UInteger(454);
        assert_eq!("u32", entry.internal_type());

        let entry = DataEntry::Long(-123_456_789);
        assert_eq!("i64", entry.internal_type());

        let entry = DataEntry::ULong(987_654_321);
        assert_eq!("u64", entry.internal_type());

        let entry = DataEntry::Float(-90.345);
        assert_eq!("f32", entry.internal_type());

        let entry = DataEntry::Double(-34532.34);
        assert_eq!("f64", entry.internal_type());

        let entry = DataEntry::Boolean(true);
        assert_eq!("bool", entry.internal_type());

        let entry = DataEntry::Character('a');
        assert_eq!("char", entry.internal_type());

        let entry = DataEntry::Invalid("Some error explanation".to_owned());
        assert_eq!("invalid", entry.internal_type());
    }

    #[test]
    fn addition() {
        let a = DataEntry::Integer(-34);
        let b = DataEntry::Integer(12);
        assert_eq!(DataEntry::Integer(-22), a + b);

        let a = DataEntry::Integer(-34);
        let b = DataEntry::Text("hello world".to_owned());
        assert_eq!(DataEntry::Text("-34hello world".to_owned()), a + b);

        let a = DataEntry::Double(-100_000.000_1);
        let b = DataEntry::Long(-12);
        assert_eq!(DataEntry::Double(-100_012.000_1), a + b);

        let a = DataEntry::Character('x');
        let b = DataEntry::Boolean(false);
        assert_eq!(DataEntry::Text("xfalse".to_owned()), a + b);

        let a = DataEntry::ULong(123_456_789);
        let b = DataEntry::Boolean(true);
        assert_eq!(DataEntry::ULong(123_456_790), a + b);

        let a = DataEntry::Boolean(false);
        let b = DataEntry::Double(234_567.120_345);
        assert_eq!(DataEntry::Double(234_567.120_345), a + b);

        let a = DataEntry::Integer(-23);
        let b = DataEntry::ULong(234_567);
        assert_eq!(DataEntry::Double(234_544 as f64), a + b);

        let a = DataEntry::Boolean(false);
        let b = DataEntry::Boolean(true);
        assert_eq!(DataEntry::Integer(1), a + b);

        let a = DataEntry::Invalid("Some error explanation".to_owned());
        let b = DataEntry::Integer(123);
        assert_eq!(DataEntry::Invalid("Some error explanation".to_owned()), a + b);
    }

    #[test]
    fn substraction() {
        let a = DataEntry::Integer(-34);
        let b = DataEntry::Integer(12);
        assert_eq!(DataEntry::Integer(-46), a - b);

        let a = DataEntry::Integer(-34);
        let b = DataEntry::Text("hello world".to_owned());
        assert_eq!(DataEntry::Invalid("subtracting strings is invalid".to_owned()), a - b);

        let a = DataEntry::Double(-100_000.000_1);
        let b = DataEntry::Long(-12);
        assert_eq!(DataEntry::Double(-99_988.000_1), a - b);

        let a = DataEntry::Character('x');
        let b = DataEntry::Boolean(false);
        assert_eq!(DataEntry::Invalid("subtracting from characters is invalid".to_owned()), a - b);

        let a = DataEntry::ULong(123_456_789);
        let b = DataEntry::Boolean(true);
        assert_eq!(DataEntry::Double(123_456_788 as f64), a - b);

        let a = DataEntry::Boolean(false);
        let b = DataEntry::Double(234_567.120_345);
        assert_eq!(DataEntry::Double(-234_567.120_345), a - b);

        let a = DataEntry::Integer(-23);
        let b = DataEntry::ULong(234_567);
        assert_eq!(DataEntry::Double(-234_590 as f64), a - b);

        let a = DataEntry::Boolean(false);
        let b = DataEntry::Boolean(true);
        assert_eq!(DataEntry::Integer(-1), a - b);

        let a = DataEntry::Invalid("Some error explanation".to_owned());
        let b = DataEntry::Integer(123);
        assert_eq!(DataEntry::Invalid("Some error explanation".to_owned()), a - b);
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
