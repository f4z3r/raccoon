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
                    DataEntry::Text(txt2)       => DataEntry::Text(int1.to_string() + &txt2)
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
                    DataEntry::Text(txt2)       => DataEntry::Text(int1.to_string() + &txt2)
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
                    DataEntry::Text(txt2)       => DataEntry::Text(int1.to_string() + &txt2)
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
                    DataEntry::Text(txt2)       => DataEntry::Text(int1.to_string() + &txt2)
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
                    DataEntry::Text(txt2)       => DataEntry::Text(f1.to_string() + &txt2)
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
                    DataEntry::Text(txt2)       => DataEntry::Text(f1.to_string() + &txt2)
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
                    DataEntry::Text(txt2)       => DataEntry::Text(b1.to_string() + &txt2)
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
                    DataEntry::Text(txt2)       => DataEntry::Text(ch1.to_string() + &txt2)
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
                    DataEntry::Text(txt2)       => DataEntry::Text(txt1 + &txt2)
                }
            }
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
                    DataEntry::Character(_)     => panic!("subtracting characters is invalid"),
                    DataEntry::Text(_)          => panic!("subtracting strings is invalid")
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
                    DataEntry::Character(_)     => panic!("subtracting characters is invalid"),
                    DataEntry::Text(_)          => panic!("subtracting strings is invalid")
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
                    DataEntry::Character(_)     => panic!("subtracting characters is invalid"),
                    DataEntry::Text(_)          => panic!("subtracting strings is invalid")
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
                    DataEntry::Character(_)     => panic!("subtracting characters is invalid"),
                    DataEntry::Text(_)          => panic!("subtracting strings is invalid")
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
                    DataEntry::Character(_)     => panic!("subtracting characters is invalid"),
                    DataEntry::Text(_)          => panic!("subtracting strings is invalid")
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
                    DataEntry::Character(_)     => panic!("subtracting characters is invalid"),
                    DataEntry::Text(_)          => panic!("subtracting strings is invalid")
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
                    DataEntry::Character(_)     => panic!("subtracting characters is invalid"),
                    DataEntry::Text(_)          => panic!("subtracting strings is invalid")
                }
            },
            DataEntry::Character(_)     => panic!("subtracting from characters is invalid"),
            DataEntry::Text(_)          => panic!("subtracting from strings is invalid")
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
                    }
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
                    DataEntry::Text(txt2)       => DataEntry::Text(txt2.repeat(int1 as usize))
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
                    }
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
                    DataEntry::Text(txt2)       => DataEntry::Text(txt2.repeat(int1 as usize))
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
                    }
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
                    }
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
                    DataEntry::Text(txt2)       => DataEntry::Text(txt2.repeat(b1 as usize))
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
                    DataEntry::Character(_)     => panic!("mutliplying a character with another is invalid"),
                    DataEntry::Text(_)          => panic!("multiplying a character with a string is invalid")
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
                    DataEntry::Character(_)     => panic!("mutliplying a string with a character is invalid"),
                    DataEntry::Text(_)          => panic!("multiplying a string with another is invalid")
                }
            }
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
                    DataEntry::Boolean(_)       => panic!("dividing by boolean is invalid"),
                    DataEntry::Character(_)     => panic!("dividing by character is invalid"),
                    DataEntry::Text(_)          => panic!("dividing by string is invalid")
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
                    DataEntry::Boolean(_)       => panic!("dividing by boolean is invalid"),
                    DataEntry::Character(_)     => panic!("dividing by character is invalid"),
                    DataEntry::Text(_)          => panic!("dividing by string is invalid")
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
                    DataEntry::Boolean(_)       => panic!("dividing by boolean is invalid"),
                    DataEntry::Character(_)     => panic!("dividing by character is invalid"),
                    DataEntry::Text(_)          => panic!("dividing by string is invalid")
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
                    DataEntry::Boolean(_)       => panic!("dividing by boolean is invalid"),
                    DataEntry::Character(_)     => panic!("dividing by character is invalid"),
                    DataEntry::Text(_)          => panic!("dividing by string is invalid")
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
                    DataEntry::Boolean(_)       => panic!("dividing by boolean is invalid"),
                    DataEntry::Character(_)     => panic!("dividing by character is invalid"),
                    DataEntry::Text(_)          => panic!("dividing by string is invalid")
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
                    DataEntry::Boolean(_)       => panic!("dividing by boolean is invalid"),
                    DataEntry::Character(_)     => panic!("dividing by character is invalid"),
                    DataEntry::Text(_)          => panic!("dividing by string is invalid")
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
                    DataEntry::Boolean(_)       => panic!("dividing by boolean is invalid"),
                    DataEntry::Character(_)     => panic!("dividing by character is invalid"),
                    DataEntry::Text(_)          => panic!("dividing by string is invalid")
                }
            },
            DataEntry::Character(_)     => panic!("dividing characters is invalid"),
            DataEntry::Text(_)          => panic!("dividing strings is invalid")
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
    }

    #[test]
    #[should_panic(expected="subtracting from characters is invalid")]
    fn invalid_sub_01() {
        let a = DataEntry::Character('x');
        let b = DataEntry::Boolean(false);
        let _c = a - b;      // panic!
    }

    #[test]
    #[should_panic(expected="subtracting strings is invalid")]
    fn invalid_sub_02() {
        let a = DataEntry::Integer(-34);
        let b = DataEntry::Text("hello world".to_owned());
        let _c = a - b;      // panic!
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
    }

    #[test]
    #[should_panic(expected="mutliplying a string with a character is invalid")]
    fn invalid_mul_01() {
        let a = DataEntry::Text("hello".to_owned());
        let b = DataEntry::Character('a');
        let _c = a * b;      // panic!
    }

    #[test]
    #[should_panic(expected="mutliplying a character with another is invalid")]
    fn invalid_mul_02() {
        let a = DataEntry::Character('a');
        let b = DataEntry::Character('b');
        let _c = a * b;      // panic!
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
    }

    #[test]
    #[should_panic(expected="dividing by string is invalid")]
    fn invalid_div_01() {
        let a = DataEntry::Integer(-34);
        let b = DataEntry::Text("hello world".to_owned());
        let _c = a / b;     //panic!
    }

    #[test]
    #[should_panic(expected="dividing characters is invalid")]
    fn invalid_div_02() {
        let a = DataEntry::Character('x');
        let b = DataEntry::Boolean(false);
        let _c = a / b;     // panic!
    }

    #[test]
    #[should_panic(expected="dividing by boolean is invalid")]
    fn invalid_div_03() {
        let a = DataEntry::ULong(123_456_789);
        let b = DataEntry::Boolean(true);
        let _c = a / b;     //panic!
    }

    #[test]
    #[should_panic(expected="dividing by boolean is invalid")]
    fn invalid_div_04() {
        let a = DataEntry::Boolean(false);
        let b = DataEntry::Boolean(true);
        let _c = a / b;     // panic!
    }

    #[test]
    #[should_panic(expected="dividing strings is invalid")]
    fn invalid_div_05() {
        let a = DataEntry::Text("hello ".to_owned());
        let b = DataEntry::Float(3.4);
        let _c = a / b;     // panic!
    }

    #[test]
    #[should_panic(expected="dividing by character is invalid")]
    fn invalid_div_06() {
        let a = DataEntry::Long(5i64);
        let b = DataEntry::Character('a');
        let _c = a / b;     // panic!
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
