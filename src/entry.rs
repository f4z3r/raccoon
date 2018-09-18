//! `entry` module. Contains all code related to data entries.

use std::ops::{Add, Sub, Div, Mul};

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
    /// A missing entry
    NA
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
    /// - `char`,
    /// - `na`.
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
            DataEntry::NA           => "na"
        }
    }

    /// Retrieves the type of the entry.
    ///
    /// # Returns
    /// A `DataType` object.
    pub fn data_type(&self) -> DataType {
        match self {
            DataEntry::Text(_)      => DataType::Text,
            DataEntry::Integer(_)   => DataType::Integer,
            DataEntry::UInteger(_)  => DataType::UInteger,
            DataEntry::Long(_)      => DataType::Long,
            DataEntry::ULong(_)     => DataType::ULong,
            DataEntry::Float(_)     => DataType::Float,
            DataEntry::Double(_)    => DataType::Double,
            DataEntry::Boolean(_)   => DataType::Boolean,
            DataEntry::Character(_) => DataType::Character,
            DataEntry::NA           => DataType::NA
        }
    }

    /// Convert this entry into another data type. Note this does not modify the initial entry itself but returns a new
    /// entry with the desired type.
    ///
    /// # Args
    /// - `data_type`: the data type that is desired.
    ///
    /// # Returns
    /// A new `DataEntry`. Lossy conversions that reduce precision are allowed (e.g. `DataType::Float` to
    /// `DataType::Integer`). However, conversions from signed to unsigned numericals will return `DataEntry::NA` (e.g.
    /// `DataType::Long` to `DataType::UInteger`).
    ///
    /// Moreover, no type can be converted to `DataType::Character` except for entries having type `DataType::Text`.
    ///
    /// All types can be converted into `DataType::Boolean`, with exception of `DataType::Character`. The conversion
    /// is performed using a simple "equals 0" check for numeric values and a full parse for `DataType::Text`.
    pub fn convert_to(&self, data_type: &DataType) -> DataEntry {
        match *self {
            DataEntry::Integer(int)         => {
                match *data_type {
                    DataType::Integer   => DataEntry::from(int),
                    DataType::UInteger  => DataEntry::NA,
                    DataType::Long      => DataEntry::from(int as i64),
                    DataType::ULong     => DataEntry::NA,
                    DataType::Float     => DataEntry::from(int as f32),
                    DataType::Double    => DataEntry::from(int as f64),
                    DataType::Boolean   => DataEntry::from(int != 0),
                    DataType::Character => DataEntry::NA,
                    DataType::Text      => DataEntry::from(int.to_string()),
                    _                   => DataEntry::NA,

                }
            },
            DataEntry::UInteger(int)        => {
                match data_type {
                    DataType::Integer   => DataEntry::NA,
                    DataType::UInteger  => DataEntry::from(int),
                    DataType::Long      => DataEntry::from(int as i64),
                    DataType::ULong     => DataEntry::from(int as u64),
                    DataType::Float     => DataEntry::from(int as f32),
                    DataType::Double    => DataEntry::from(int as f64),
                    DataType::Boolean   => DataEntry::from(int != 0u32),
                    DataType::Character => DataEntry::NA,
                    DataType::Text      => DataEntry::from(int.to_string()),
                    _                   => DataEntry::NA,

                }
            },
            DataEntry::Long(int)            => {
                match data_type {
                    DataType::Integer   => DataEntry::NA,
                    DataType::UInteger  => DataEntry::NA,
                    DataType::Long      => DataEntry::from(int),
                    DataType::ULong     => DataEntry::NA,
                    DataType::Float     => DataEntry::from(int as f32),
                    DataType::Double    => DataEntry::from(int as f64),
                    DataType::Boolean   => DataEntry::from(int != 0i64),
                    DataType::Character => DataEntry::NA,
                    DataType::Text      => DataEntry::from(int.to_string()),
                    _                   => DataEntry::NA,

                }
            },
            DataEntry::ULong(int)           => {
                match data_type {
                    DataType::Integer   => DataEntry::NA,
                    DataType::UInteger  => DataEntry::NA,
                    DataType::Long      => DataEntry::NA,
                    DataType::ULong     => DataEntry::from(int),
                    DataType::Float     => DataEntry::from(int as f32),
                    DataType::Double    => DataEntry::from(int as f64),
                    DataType::Boolean   => DataEntry::from(int != 0u64),
                    DataType::Character => DataEntry::NA,
                    DataType::Text      => DataEntry::from(int.to_string()),
                    _                   => DataEntry::NA,

                }
            },
            DataEntry::Float(float)         => {
                match data_type {
                    DataType::Integer   => DataEntry::from(float as i32),
                    DataType::UInteger  => DataEntry::NA,
                    DataType::Long      => DataEntry::from(float as i64),
                    DataType::ULong     => DataEntry::NA,
                    DataType::Float     => DataEntry::from(float),
                    DataType::Double    => DataEntry::from(float as f64),
                    DataType::Boolean   => DataEntry::from(float != 0f32),
                    DataType::Character => DataEntry::NA,
                    DataType::Text      => DataEntry::from(float.to_string()),
                    _                   => DataEntry::NA,

                }
            },
            DataEntry::Double(float)        => {
                match data_type {
                    DataType::Integer   => DataEntry::from(float as i32),
                    DataType::UInteger  => DataEntry::NA,
                    DataType::Long      => DataEntry::from(float as i64),
                    DataType::ULong     => DataEntry::NA,
                    DataType::Float     => DataEntry::from(float as f32),
                    DataType::Double    => DataEntry::from(float),
                    DataType::Boolean   => DataEntry::from(float != 0f64),
                    DataType::Character => DataEntry::NA,
                    DataType::Text      => DataEntry::from(float.to_string()),
                    _                   => DataEntry::NA,

                }
            },
            DataEntry::Boolean(boolean)     => {
                match data_type {
                    DataType::Integer   => DataEntry::from(boolean as i32),
                    DataType::UInteger  => DataEntry::from(boolean as u32),
                    DataType::Long      => DataEntry::from(boolean as i64),
                    DataType::ULong     => DataEntry::from(boolean as u64),
                    DataType::Float     => DataEntry::from(boolean as u32 as f32),
                    DataType::Double    => DataEntry::from(boolean as u32 as f64),
                    DataType::Boolean   => DataEntry::from(boolean),
                    DataType::Character => DataEntry::NA,
                    DataType::Text      => DataEntry::from(boolean.to_string()),
                    _                   => DataEntry::NA,

                }
            },
            DataEntry::Character(ch)        => {
                match data_type {
                    DataType::Integer   => DataEntry::from(ch as i32),
                    DataType::UInteger  => DataEntry::from(ch as u32),
                    DataType::Long      => DataEntry::from(ch as i64),
                    DataType::ULong     => DataEntry::from(ch as u64),
                    DataType::Float     => DataEntry::from(ch as u32 as f32),
                    DataType::Double    => DataEntry::from(ch as u32 as f64),
                    DataType::Boolean   => DataEntry::NA,
                    DataType::Character => DataEntry::from(ch),
                    DataType::Text      => DataEntry::from(ch.to_string()),
                    _                   => DataEntry::NA,

                }
            },
            DataEntry::Text(ref txt)            => {
                match data_type {
                    DataType::Integer   => {
                        match txt.parse::<i32>() {
                            Ok(int) => DataEntry::from(int),
                            Err(_)  => DataEntry::NA
                        }
                    },
                    DataType::UInteger  => {
                        match txt.parse::<u32>() {
                            Ok(int) => DataEntry::from(int),
                            Err(_)  => DataEntry::NA
                        }
                    },
                    DataType::Long      => {
                        match txt.parse::<i64>() {
                            Ok(int) => DataEntry::from(int),
                            Err(_)  => DataEntry::NA
                        }
                    },
                    DataType::ULong     => {
                        match txt.parse::<u64>() {
                            Ok(int) => DataEntry::from(int),
                            Err(_)  => DataEntry::NA
                        }
                    },
                    DataType::Float     => {
                        match txt.parse::<f32>() {
                            Ok(f)   => DataEntry::from(f),
                            Err(_)  => DataEntry::NA
                        }
                    },
                    DataType::Double    => {
                        match txt.parse::<f64>() {
                            Ok(f)   => DataEntry::from(f),
                            Err(_)  => DataEntry::NA
                        }
                    },
                    DataType::Boolean   => {
                        match txt.parse::<bool>() {
                            Ok(b)   => DataEntry::from(b),
                            Err(_)  => DataEntry::NA
                        }
                    },
                    DataType::Character => {
                        match txt.parse::<char>() {
                            Ok(ch)  => DataEntry::from(ch),
                            Err(_)  => DataEntry::NA
                        }
                    },
                    DataType::Text      => DataEntry::from(txt),
                    _                   => DataEntry::NA,

                }
            },
            _                               => DataEntry::NA

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
                    _                           => DataEntry::NA
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
                    _                           => DataEntry::NA
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
                    _                           => DataEntry::NA
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
                    _                           => DataEntry::NA
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
                    _                           => DataEntry::NA
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
                    _                           => DataEntry::NA
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
                    _                           => DataEntry::NA
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
                    _                           => DataEntry::NA
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
                    _                           => DataEntry::NA
                }
            },
            _                           => DataEntry::NA
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
                    _                           => DataEntry::NA
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
                    _                           => DataEntry::NA
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
                    _                           => DataEntry::NA
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
                    _                           => DataEntry::NA
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
                    _                           => DataEntry::NA
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
                    _                           => DataEntry::NA
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
                    _                           => DataEntry::NA
                }
            },
            _                           => DataEntry::NA
        }
    }
}

impl Mul for DataEntry {
    type Output = DataEntry;

    fn mul(self, rhs: DataEntry) -> Self::Output {
        match self {
            DataEntry::Integer(int1)    => {
                match rhs {
                    DataEntry::Integer(int2)    => DataEntry::Integer(int1 * int2),
                    DataEntry::UInteger(int2)   => DataEntry::Long(int1 as i64 * int2 as i64),
                    DataEntry::Long(int2)       => DataEntry::Long(int1 as i64 * int2),
                    DataEntry::ULong(int2)      => DataEntry::Double(int1 as f64 * int2 as f64),
                    DataEntry::Float(f2)        => DataEntry::Float(int1 as f32 * f2),
                    DataEntry::Double(f2)       => DataEntry::Double(int1 as f64 * f2),
                    DataEntry::Boolean(b2)      => DataEntry::Integer(int1 * b2 as i32),
                    DataEntry::Character(ch1)   => {
                        if int1 >= 0i32 {
                            DataEntry::Text(ch1.to_string().repeat(int1 as usize))
                        } else {
                            DataEntry::Text("".to_owned())
                        }
                    },
                    DataEntry::Text(txt2)       => {
                        if int1 >= 0i32 {
                            DataEntry::Text(txt2.repeat(int1 as usize))
                        } else {
                            DataEntry::Text("".to_owned())
                        }
                    },
                    _                           => DataEntry::NA
                }
            },
            DataEntry::UInteger(int1)   => {
                match rhs {
                    DataEntry::Integer(int2)    => DataEntry::Long(int1 as i64 * int2 as i64),
                    DataEntry::UInteger(int2)   => DataEntry::UInteger(int1 * int2),
                    DataEntry::Long(int2)       => DataEntry::Long(int1 as i64 * int2),
                    DataEntry::ULong(int2)      => DataEntry::ULong(int1 as u64 * int2),
                    DataEntry::Float(f2)        => DataEntry::Float(int1 as f32 * f2),
                    DataEntry::Double(f2)       => DataEntry::Double(int1 as f64 * f2),
                    DataEntry::Boolean(b2)      => DataEntry::Integer(int1 as i32 * b2 as i32),
                    DataEntry::Character(ch1)   => DataEntry::Text(ch1.to_string().repeat(int1 as usize)),
                    DataEntry::Text(txt2)       => DataEntry::Text(txt2.repeat(int1 as usize)),
                    _                           => DataEntry::NA
                }
            },
            DataEntry::Long(int1)       => {
                match rhs {
                    DataEntry::Integer(int2)    => DataEntry::Long(int1 as i64 * int2 as i64),
                    DataEntry::UInteger(int2)   => DataEntry::Long(int1 as i64 * int2 as i64),
                    DataEntry::Long(int2)       => DataEntry::Long(int1 * int2),
                    DataEntry::ULong(int2)      => DataEntry::Double(int1 as f64 * int2 as f64),
                    DataEntry::Float(f2)        => DataEntry::Float(int1 as f32 * f2),
                    DataEntry::Double(f2)       => DataEntry::Double(int1 as f64 * f2),
                    DataEntry::Boolean(b2)      => DataEntry::Long(int1 * b2 as i64),
                    DataEntry::Character(ch1)   => {
                        if int1 >= 0i64 {
                            DataEntry::Text(ch1.to_string().repeat(int1 as usize))
                        } else {
                            DataEntry::Text("".to_owned())
                        }
                    },
                    DataEntry::Text(txt2)       => {
                        if int1 >= 0i64 {
                            DataEntry::Text(txt2.repeat(int1 as usize))
                        } else {
                            DataEntry::Text("".to_owned())
                        }
                    },
                    _                           => DataEntry::NA
                }
            },
            DataEntry::ULong(int1)      => {
                match rhs {
                    DataEntry::Integer(int2)    => DataEntry::Double(int1 as f64 * int2 as f64),
                    DataEntry::UInteger(int2)   => DataEntry::ULong(int1 * int2 as u64),
                    DataEntry::Long(int2)       => DataEntry::Double(int1 as f64 * int2 as f64),
                    DataEntry::ULong(int2)      => DataEntry::ULong(int1 * int2),
                    DataEntry::Float(f2)        => DataEntry::Double(int1 as f64 * f2 as f64),
                    DataEntry::Double(f2)       => DataEntry::Double(int1 as f64 * f2),
                    DataEntry::Boolean(b2)      => DataEntry::Double(int1 as f64 * b2 as u8 as f64),
                    DataEntry::Character(ch1)   => DataEntry::Text(ch1.to_string().repeat(int1 as usize)),
                    DataEntry::Text(txt2)       => DataEntry::Text(txt2.repeat(int1 as usize)),
                    _                           => DataEntry::NA
                }
            },
            DataEntry::Float(f1)        => {
                match rhs {
                    DataEntry::Integer(int2)    => DataEntry::Float(f1 * int2 as f32),
                    DataEntry::UInteger(int2)   => DataEntry::Float(f1 * int2 as f32),
                    DataEntry::Long(int2)       => DataEntry::Double(f1 as f64 * int2 as f64),
                    DataEntry::ULong(int2)      => DataEntry::Double(f1 as f64 * int2 as f64),
                    DataEntry::Float(f2)        => DataEntry::Float(f1 * f2),
                    DataEntry::Double(f2)       => DataEntry::Double(f1 as f64 * f2),
                    DataEntry::Boolean(b2)      => DataEntry::Float(f1 * b2 as u8 as f32),
                    DataEntry::Character(ch1)   => {
                        if f1 >= 0f32 {
                            DataEntry::Text(ch1.to_string().repeat(f1 as usize))
                        } else {
                            DataEntry::Text("".to_owned())
                        }
                    },
                    DataEntry::Text(txt2)       => {
                        if f1 >= 0f32 {
                            DataEntry::Text(txt2.repeat(f1 as usize))
                        } else {
                            DataEntry::Text("".to_owned())
                        }
                    },
                    _                           => DataEntry::NA
                }
            },
            DataEntry::Double(f1)       => {
                match rhs {
                    DataEntry::Integer(int2)    => DataEntry::Double(f1 * int2 as f64),
                    DataEntry::UInteger(int2)   => DataEntry::Double(f1 * int2 as f64),
                    DataEntry::Long(int2)       => DataEntry::Double(f1 * int2 as f64),
                    DataEntry::ULong(int2)      => DataEntry::Double(f1 * int2 as f64),
                    DataEntry::Float(f2)        => DataEntry::Double(f1 * f2 as f64),
                    DataEntry::Double(f2)       => DataEntry::Double(f1 * f2),
                    DataEntry::Boolean(b2)      => DataEntry::Double(f1 * b2 as u8 as f64),
                    DataEntry::Character(ch1)   => {
                        if f1 >= 0f64 {
                            DataEntry::Text(ch1.to_string().repeat(f1 as usize))
                        } else {
                            DataEntry::Text("".to_owned())
                        }
                    },
                    DataEntry::Text(txt2)       => {
                        if f1 >= 0f64 {
                            DataEntry::Text(txt2.repeat(f1 as usize))
                        } else {
                            DataEntry::Text("".to_owned())
                        }
                    },
                    _                           => DataEntry::NA
                }
            },
            DataEntry::Boolean(b1)      => {
                match rhs {
                    DataEntry::Integer(int2)    => DataEntry::Integer(b1 as i32 * int2),
                    DataEntry::UInteger(int2)   => DataEntry::UInteger(b1 as u32 * int2),
                    DataEntry::Long(int2)       => DataEntry::Long(b1 as i64 * int2),
                    DataEntry::ULong(int2)      => DataEntry::ULong(b1 as u64 * int2),
                    DataEntry::Float(f2)        => DataEntry::Float(b1 as u8 as f32 * f2),
                    DataEntry::Double(f2)       => DataEntry::Double(b1 as u8 as f64 * f2),
                    DataEntry::Boolean(b2)      => DataEntry::Integer(b1 as i32 * b2 as i32),
                    DataEntry::Character(ch1)   => DataEntry::Text(ch1.to_string().repeat(b1 as usize)),
                    DataEntry::Text(txt2)       => DataEntry::Text(txt2.repeat(b1 as usize)),
                    _                           => DataEntry::NA
                }
            },
            DataEntry::Character(ch1)   => {
                match rhs {
                    DataEntry::Integer(int2)   => {
                        if int2 >= 0i32 {
                            DataEntry::Text(ch1.to_string().repeat(int2 as usize))
                        } else {
                            DataEntry::Text("".to_owned())
                        }
                    },
                    DataEntry::UInteger(int2)   => DataEntry::Text(ch1.to_string().repeat(int2 as usize)),
                    DataEntry::Long(int2)       => {
                        if int2 >= 0i64 {
                            DataEntry::Text(ch1.to_string().repeat(int2 as usize))
                        } else {
                            DataEntry::Text("".to_owned())
                        }
                    },
                    DataEntry::ULong(int2)      => DataEntry::Text(ch1.to_string().repeat(int2 as usize)),
                    DataEntry::Float(f2)        => {
                        if f2 >= 0f32 {
                            DataEntry::Text(ch1.to_string().repeat(f2 as usize))
                        } else {
                            DataEntry::Text("".to_owned())
                        }
                    },
                    DataEntry::Double(f2)       => {
                        if f2 >= 0f64 {
                            DataEntry::Text(ch1.to_string().repeat(f2 as usize))
                        } else {
                            DataEntry::Text("".to_owned())
                        }
                    },
                    DataEntry::Boolean(b2)      => DataEntry::Text(ch1.to_string().repeat(b2 as usize)),
                    _                           => DataEntry::NA
                }
            },
            DataEntry::Text(txt1)       => {
                match rhs {
                    DataEntry::Integer(int2)    => {
                        if int2 >= 0i32 {
                            DataEntry::Text(txt1.repeat(int2 as usize))
                        } else {
                            DataEntry::Text("".to_owned())
                        }
                    },
                    DataEntry::UInteger(int2)   => DataEntry::Text(txt1.repeat(int2 as usize)),
                    DataEntry::Long(int2)       => {
                        if int2 >= 0i64 {
                            DataEntry::Text(txt1.repeat(int2 as usize))
                        } else {
                            DataEntry::Text("".to_owned())
                        }
                    },
                    DataEntry::ULong(int2)      => DataEntry::Text(txt1.repeat(int2 as usize)),
                    DataEntry::Float(f2)        => {
                        if f2 >= 0f32 {
                            DataEntry::Text(txt1.repeat(f2 as usize))
                        } else {
                            DataEntry::Text("".to_owned())
                        }
                    },
                    DataEntry::Double(f2)       => {
                        if f2 >= 0f64 {
                            DataEntry::Text(txt1.repeat(f2 as usize))
                        } else {
                            DataEntry::Text("".to_owned())
                        }
                    },
                    DataEntry::Boolean(b2)      => DataEntry::Text(txt1.repeat(b2 as usize)),
                    _                           => DataEntry::NA
                }
            },
            _                           => DataEntry::NA
        }
    }
}

impl Div for DataEntry {
    type Output = DataEntry;

    fn div(self, rhs: DataEntry) -> Self::Output {
        match self {
            DataEntry::Integer(int1)    => {
                match rhs {
                    DataEntry::Integer(int2)    => DataEntry::Integer(int1 / int2),
                    DataEntry::UInteger(int2)   => DataEntry::Long(int1 as i64 / int2 as i64),
                    DataEntry::Long(int2)       => DataEntry::Long(int1 as i64 / int2),
                    DataEntry::ULong(int2)      => DataEntry::Double(int1 as f64 / int2 as f64),
                    DataEntry::Float(f2)        => DataEntry::Float(int1 as f32 / f2),
                    DataEntry::Double(f2)       => DataEntry::Double(int1 as f64 / f2),
                    _                           => DataEntry::NA
                }
            },
            DataEntry::UInteger(int1)   => {
                match rhs {
                    DataEntry::Integer(int2)    => DataEntry::Long(int1 as i64 / int2 as i64),
                    DataEntry::UInteger(int2)   => DataEntry::UInteger(int1 / int2),
                    DataEntry::Long(int2)       => DataEntry::Long(int1 as i64 / int2),
                    DataEntry::ULong(int2)      => DataEntry::ULong(int1 as u64 / int2),
                    DataEntry::Float(f2)        => DataEntry::Float(int1 as f32 / f2),
                    DataEntry::Double(f2)       => DataEntry::Double(int1 as f64 / f2),
                    _                           => DataEntry::NA
                }
            },
            DataEntry::Long(int1)       => {
                match rhs {
                    DataEntry::Integer(int2)    => DataEntry::Long(int1 as i64 / int2 as i64),
                    DataEntry::UInteger(int2)   => DataEntry::Long(int1 as i64 / int2 as i64),
                    DataEntry::Long(int2)       => DataEntry::Long(int1 / int2),
                    DataEntry::ULong(int2)      => DataEntry::Double(int1 as f64 / int2 as f64),
                    DataEntry::Float(f2)        => DataEntry::Float(int1 as f32 / f2),
                    DataEntry::Double(f2)       => DataEntry::Double(int1 as f64 / f2),
                    _                           => DataEntry::NA
                }
            },
            DataEntry::ULong(int1)      => {
                match rhs {
                    DataEntry::Integer(int2)    => DataEntry::Double(int1 as f64 / int2 as f64),
                    DataEntry::UInteger(int2)   => DataEntry::ULong(int1 / int2 as u64),
                    DataEntry::Long(int2)       => DataEntry::Double(int1 as f64 / int2 as f64),
                    DataEntry::ULong(int2)      => DataEntry::ULong(int1 / int2),
                    DataEntry::Float(f2)        => DataEntry::Double(int1 as f64 / f2 as f64),
                    DataEntry::Double(f2)       => DataEntry::Double(int1 as f64 / f2),
                    _                           => DataEntry::NA
                }
            },
            DataEntry::Float(f1)        => {
                match rhs {
                    DataEntry::Integer(int2)    => DataEntry::Float(f1 / int2 as f32),
                    DataEntry::UInteger(int2)   => DataEntry::Float(f1 / int2 as f32),
                    DataEntry::Long(int2)       => DataEntry::Double(f1 as f64 / int2 as f64),
                    DataEntry::ULong(int2)      => DataEntry::Double(f1 as f64 / int2 as f64),
                    DataEntry::Float(f2)        => DataEntry::Float(f1 / f2),
                    DataEntry::Double(f2)       => DataEntry::Double(f1 as f64 / f2),
                    _                           => DataEntry::NA
                }
            },
            DataEntry::Double(f1)       => {
                match rhs {
                    DataEntry::Integer(int2)    => DataEntry::Double(f1 / int2 as f64),
                    DataEntry::UInteger(int2)   => DataEntry::Double(f1 / int2 as f64),
                    DataEntry::Long(int2)       => DataEntry::Double(f1 / int2 as f64),
                    DataEntry::ULong(int2)      => DataEntry::Double(f1 / int2 as f64),
                    DataEntry::Float(f2)        => DataEntry::Double(f1 / f2 as f64),
                    DataEntry::Double(f2)       => DataEntry::Double(f1 / f2),
                    _                           => DataEntry::NA
                }
            },
            DataEntry::Boolean(b1)      => {
                match rhs {
                    DataEntry::Integer(int2)    => DataEntry::Integer(b1 as i32 / int2),
                    DataEntry::UInteger(int2)   => DataEntry::UInteger(b1 as u32 / int2),
                    DataEntry::Long(int2)       => DataEntry::Long(b1 as i64 / int2),
                    DataEntry::ULong(int2)      => DataEntry::ULong(b1 as u64 / int2),
                    DataEntry::Float(f2)        => DataEntry::Float(b1 as u8 as f32 / f2),
                    DataEntry::Double(f2)       => DataEntry::Double(b1 as u8 as f64 / f2),
                    _                           => DataEntry::NA
                }
            },
            _                           => DataEntry::NA
        }
    }
}

impl From<i8> for DataEntry {
    fn from(integer: i8) -> Self {
        DataEntry::Integer(integer as i32)
    }
}

impl From<u8> for DataEntry {
    fn from(integer: u8) -> Self {
        DataEntry::UInteger(integer as u32)
    }
}

impl From<i16> for DataEntry {
    fn from(integer: i16) -> Self {
        DataEntry::Integer(integer as i32)
    }
}

impl From<u16> for DataEntry {
    fn from(integer: u16) -> Self {
        DataEntry::UInteger(integer as u32)
    }
}

impl From<i32> for DataEntry {
    fn from(integer: i32) -> Self {
        DataEntry::Integer(integer)
    }
}

impl From<u32> for DataEntry {
    fn from(integer: u32) -> Self {
        DataEntry::UInteger(integer)
    }
}

impl From<i64> for DataEntry {
    fn from(integer: i64) -> Self {
        DataEntry::Long(integer)
    }
}

impl From<u64> for DataEntry {
    fn from(integer: u64) -> Self {
        DataEntry::ULong(integer)
    }
}

impl From<f32> for DataEntry {
    fn from(float: f32) -> Self {
        DataEntry::Float(float)
    }
}

impl From<f64> for DataEntry {
    fn from(float: f64) -> Self {
        DataEntry::Double(float)
    }
}

impl From<bool> for DataEntry {
    fn from(boolean: bool) -> Self {
        DataEntry::Boolean(boolean)
    }
}

impl From<char> for DataEntry {
    fn from(character: char) -> Self {
        DataEntry::Character(character)
    }
}

impl From<String> for DataEntry {
    fn from(text: String) -> Self {
        DataEntry::Text(text)
    }
}

impl<'a> From<&'a String> for DataEntry {
    fn from(text: &'a String) -> Self {
        DataEntry::Text(text.to_owned())
    }
}

impl<'a> From<&'a str> for DataEntry {
    fn from(text: &'a str) -> Self {
        DataEntry::Text(text.to_owned())
    }
}


/// The data type any entry can take.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Missing
    NA
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getting_types() {
        let entry = DataEntry::Text("Some text".to_owned());
        assert_eq!("String", entry.internal_type());
        assert_eq!(DataType::Text, entry.data_type());

        let entry = DataEntry::Integer(-353);
        assert_eq!("i32", entry.internal_type());
        assert_eq!(DataType::Integer, entry.data_type());

        let entry = DataEntry::UInteger(454);
        assert_eq!("u32", entry.internal_type());
        assert_eq!(DataType::UInteger, entry.data_type());

        let entry = DataEntry::Long(-123_456_789);
        assert_eq!("i64", entry.internal_type());
        assert_eq!(DataType::Long, entry.data_type());

        let entry = DataEntry::ULong(987_654_321);
        assert_eq!("u64", entry.internal_type());
        assert_eq!(DataType::ULong, entry.data_type());

        let entry = DataEntry::Float(-90.345);
        assert_eq!("f32", entry.internal_type());
        assert_eq!(DataType::Float, entry.data_type());

        let entry = DataEntry::Double(-34532.34);
        assert_eq!("f64", entry.internal_type());
        assert_eq!(DataType::Double, entry.data_type());

        let entry = DataEntry::Boolean(true);
        assert_eq!("bool", entry.internal_type());
        assert_eq!(DataType::Boolean, entry.data_type());

        let entry = DataEntry::Character('a');
        assert_eq!("char", entry.internal_type());
        assert_eq!(DataType::Character, entry.data_type());

        let entry = DataEntry::NA;
        assert_eq!("na", entry.internal_type());
        assert_eq!(DataType::NA, entry.data_type());
    }

    #[test]
    fn conversion() {
        let entry = DataEntry::Integer(-42);
        assert_eq!(DataEntry::Float(-42f32), entry.convert_to(&DataType::Float));

        let entry = DataEntry::Float(12.4);
        assert_eq!(DataEntry::Boolean(true), entry.convert_to(&DataType::Boolean));

        let entry = DataEntry::Integer(-42);
        assert_eq!(DataEntry::NA, entry.convert_to(&DataType::UInteger));

        let entry = DataEntry::Text("123.345".to_owned());
        assert_eq!(DataEntry::Double(123.345f64), entry.convert_to(&DataType::Double));

        let entry = DataEntry::Text("z".to_owned());
        assert_eq!(DataEntry::Character('z'), entry.convert_to(&DataType::Character));

        let entry = DataEntry::Double(-42.4353f64);
        assert_eq!(DataEntry::Integer(-42), entry.convert_to(&DataType::Integer));
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
        assert_eq!(DataEntry::Double(234_544f64), a + b);

        let a = DataEntry::Boolean(false);
        let b = DataEntry::Boolean(true);
        assert_eq!(DataEntry::Integer(1), a + b);
    }

    #[test]
    fn substraction() {
        let a = DataEntry::Integer(-34);
        let b = DataEntry::Integer(12);
        assert_eq!(DataEntry::Integer(-46), a - b);

        let a = DataEntry::Double(-100_000.000_1);
        let b = DataEntry::Long(-12);
        assert_eq!(DataEntry::Double(-99_988.000_1), a - b);

        let a = DataEntry::ULong(123_456_789);
        let b = DataEntry::Boolean(true);
        assert_eq!(DataEntry::Double(123_456_788f64), a - b);

        let a = DataEntry::Boolean(false);
        let b = DataEntry::Double(234_567.120_345);
        assert_eq!(DataEntry::Double(-234_567.120_345), a - b);

        let a = DataEntry::Integer(-23);
        let b = DataEntry::ULong(234_567);
        assert_eq!(DataEntry::Double(-234_590f64), a - b);

        let a = DataEntry::Boolean(false);
        let b = DataEntry::Boolean(true);
        assert_eq!(DataEntry::Integer(-1), a - b);

        let a = DataEntry::Character('x');
        let b = DataEntry::Boolean(false);
        let c = a - b;      // NA
        assert_eq!(DataEntry::NA, c);

        let a = DataEntry::Integer(-34);
        let b = DataEntry::Text("hello world".to_owned());
        let c = a - b;      // NA
        assert_eq!(DataEntry::NA, c);
    }

    #[test]
    fn multiplication() {
        let a = DataEntry::Integer(-34);
        let b = DataEntry::Integer(12);
        assert_eq!(DataEntry::Integer(-408), a * b);

        let a = DataEntry::Integer(-34);
        let b = DataEntry::Text("hello world".to_owned());
        assert_eq!(DataEntry::Text("".to_owned()), a * b);

        let a = DataEntry::Double(-100_000.000_1);
        let b = DataEntry::Long(-12);
        assert_eq!(DataEntry::Double(1_200_000.001_2), a * b);

        let a = DataEntry::Character('x');
        let b = DataEntry::Boolean(false);
        assert_eq!(DataEntry::Text("".to_owned()), a * b);

        let a = DataEntry::ULong(123_456_789);
        let b = DataEntry::Boolean(true);
        assert_eq!(DataEntry::Double(123_456_789f64), a * b);

        let a = DataEntry::Boolean(false);
        let b = DataEntry::Double(234_567.120_345);
        assert_eq!(DataEntry::Double(0f64), a * b);

        let a = DataEntry::Integer(-23);
        let b = DataEntry::ULong(234_567);
        assert_eq!(DataEntry::Double(-5_395_041f64), a * b);

        let a = DataEntry::Boolean(false);
        let b = DataEntry::Boolean(true);
        assert_eq!(DataEntry::Integer(0), a * b);

        let a = DataEntry::Text("hello ".to_owned());
        let b = DataEntry::Float(3.4);
        assert_eq!(DataEntry::Text("hello hello hello ".to_owned()), a * b);

        let a = DataEntry::Long(5i64);
        let b = DataEntry::Character('a');
        assert_eq!(DataEntry::Text("aaaaa".to_owned()), a * b);

        let a = DataEntry::Boolean(true);
        let b = DataEntry::Text("Please display me".to_owned());
        assert_eq!(DataEntry::Text("Please display me".to_owned()), a * b);

        let a = DataEntry::Text("hello".to_owned());
        let b = DataEntry::Character('a');
        let c = a * b;      // NA
        assert_eq!(DataEntry::NA, c);

        let a = DataEntry::Character('a');
        let b = DataEntry::Character('b');
        let c = a * b;      // NA
        assert_eq!(DataEntry::NA, c);

    }

    #[test]
    fn division() {
        let a = DataEntry::Integer(-34);
        let b = DataEntry::Integer(12);
        assert_eq!(DataEntry::Integer(-2), a / b);

        let a = DataEntry::Double(-100_000.000_1);
        let b = DataEntry::Long(-12);
        let c = a / b;
        assert!(DataEntry::Double(8_333.33334166) < c);
        assert!(DataEntry::Double(8_333.33334167) > c);

        let a = DataEntry::Boolean(false);
        let b = DataEntry::Double(234_567.120_345);
        assert_eq!(DataEntry::Double(0f64), a / b);

        let a = DataEntry::Integer(-23);
        let b = DataEntry::ULong(234_567);
        let c = a / b;
        assert!(DataEntry::Double(-9.8053008e-5f64) > c);
        assert!(DataEntry::Double(-9.8053009e-5f64) < c);

        let a = DataEntry::Integer(-34);
        let b = DataEntry::Text("hello world".to_owned());
        let c = a / b;     // NA
        assert_eq!(DataEntry::NA, c);

        let a = DataEntry::Character('x');
        let b = DataEntry::Boolean(false);
        let c = a / b;     // NA
        assert_eq!(DataEntry::NA, c);

        let a = DataEntry::ULong(123_456_789);
        let b = DataEntry::Boolean(true);
        let c = a / b;     // NA
        assert_eq!(DataEntry::NA, c);

        let a = DataEntry::Boolean(false);
        let b = DataEntry::Boolean(true);
        let c = a / b;     // NA
        assert_eq!(DataEntry::NA, c);

        let a = DataEntry::Text("hello ".to_owned());
        let b = DataEntry::Float(3.4);
        let c = a / b;     // NA
        assert_eq!(DataEntry::NA, c);

        let a = DataEntry::Long(5i64);
        let b = DataEntry::Character('a');
        let c = a / b;     // NA
        assert_eq!(DataEntry::NA, c);
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
