//! A data cell supporting extended operations (see [examples](./enum.DCell.html)).

use prelude::*;

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

/// A data cell that can contain all basic primitives.
///
/// The data cell supports extended operations (see Examples).
///
/// # Examples
/// ```
/// use raccoon::prelude::*;
///
/// let mut cell = DCell::from(6);
/// cell += 5;
/// assert_eq!(cell, DCell::Int(11_i64));
/// ```
///
/// The data cell supports extended operations such as boolean AND, OR, XOR:
/// ```
/// use raccoon::prelude::*;
///
/// let cell = DCell::from(true);
/// assert_eq!(cell + false, DCell::Bool(false));     // + is AND
///
/// let cell = DCell::from(true);
/// assert_eq!(cell - false, DCell::Bool(true));     // - is OR
///
/// let cell = DCell::from(true);
/// assert_eq!(cell * true, DCell::Bool(false));     // * is XOR
/// ```
///
/// It also supports string and character concatenation and multiplication:
/// ```
/// use raccoon::prelude::*;
///
/// let mut cell = DCell::from("hello!");
/// cell *= 2u32;
/// assert_eq!(cell, DCell::Text("hello!hello!".to_owned()));
///
/// cell += String::from("world!");
/// assert_eq!(cell, DCell::Text("hello!hello!world!".to_owned()));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum DCell {
    /// An integer
    Int(i64),
    /// An unsigned integer.
    UInt(u64),
    /// A floating point number,
    Float(f64),
    /// A character.
    Char(char),
    /// A boolean value.
    Bool(bool),
    /// A text entry.
    Text(String),
    /// A missing or invalid entry.
    NA
}

/// A data type.
#[derive(Debug, Clone, PartialEq)]
pub enum DType {
    /// Integer.
    Int,
    /// Unsigned integer.
    UInt,
    /// Float.
    Float,
    /// Character
    Char,
    /// Boolean.
    Bool,
    /// Text.
    Text,
    /// Missing or invalid.
    NA
}

impl ToString for DCell {
    fn to_string(&self) -> String {
        match self {
            DCell::Int(val)     => val.to_string(),
            DCell::UInt(val)    => val.to_string(),
            DCell::Float(val)   => val.to_string(),
            DCell::Char(val)    => val.to_string(),
            DCell::Bool(val)    => val.to_string(),
            DCell::Text(val)    => val.to_string(),
            DCell::NA           => String::from("NA"),
        }
    }
}

impl Typed for DCell {
    fn dtype(&self) -> DType {
        match self {
            DCell::Int(_)   => DType::Int,
            DCell::UInt(_)  => DType::UInt,
            DCell::Float(_) => DType::Float,
            DCell::Char(_)  => DType::Char,
            DCell::Bool(_)  => DType::Bool,
            DCell::Text(_)  => DType::Text,
            DCell::NA       => DType::NA,
        }
    }
}

impl<T> Add<T> for DCell where T: Into<DCell> + Typed {
    type Output = DCell;

    fn add(self, other: T) -> Self::Output {
        if self.dtype() != other.dtype() {
            DCell::NA
        } else {
            match (self, other.into()) {
                (DCell::Int(int1), DCell::Int(int2))    => DCell::Int(int1 + int2),
                (DCell::UInt(int1), DCell::UInt(int2))  => DCell::UInt(int1 + int2),
                (DCell::Float(f1), DCell::Float(f2))    => DCell::Float(f1 + f2),
                (DCell::Bool(b1), DCell::Bool(b2))      => DCell::Bool(b1 && b2),
                (DCell::Char(ch1), DCell::Char(ch2))    => DCell::Text(format!("{}{}", ch1, ch2)),
                (DCell::Text(txt1), DCell::Text(txt2))  => DCell::Text(txt1 + &txt2),
                _                                       => DCell::NA
            }
        }
    }
}

impl<T> AddAssign<T> for DCell where T: Into<DCell> + Typed {
    fn add_assign(&mut self, other: T) {
        *self = if self.dtype() != other.dtype() {
            DCell::NA
        } else {
            match (self.clone(), other.into()) {
                (DCell::Int(int1), DCell::Int(int2))    => DCell::Int(int1 + int2),
                (DCell::UInt(int1), DCell::UInt(int2))  => DCell::UInt(int1 + int2),
                (DCell::Float(f1), DCell::Float(f2))    => DCell::Float(f1 + f2),
                (DCell::Bool(b1), DCell::Bool(b2))      => DCell::Bool(b1 && b2),
                (DCell::Char(ch1), DCell::Char(ch2))    => DCell::Text(format!("{}{}", ch1, ch2)),
                (DCell::Text(txt1), DCell::Text(txt2))  => DCell::Text(txt1 + &txt2),
                _                                       => DCell::NA
            }
        }
    }
}

impl<T> Sub<T> for DCell where T: Into<DCell> + Typed {
    type Output = DCell;

    fn sub(self, other: T) -> Self::Output {
        if self.dtype() != other.dtype() {
            DCell::NA
        } else {
            match (self, other.into()) {
                (DCell::Int(int1), DCell::Int(int2))    => DCell::Int(int1 - int2),
                (DCell::UInt(int1), DCell::UInt(int2))  => DCell::UInt(int1 - int2),
                (DCell::Float(f1), DCell::Float(f2))    => DCell::Float(f1 - f2),
                (DCell::Bool(b1), DCell::Bool(b2))      => DCell::Bool(b1 || b2),
                _                                       => DCell::NA
            }
        }
    }
}

impl<T> SubAssign<T> for DCell where T: Into<DCell> + Typed {
    fn sub_assign(&mut self, other: T) {
        *self = if self.dtype() != other.dtype() {
            DCell::NA
        } else {
            match (self.clone(), other.into()) {
                (DCell::Int(int1), DCell::Int(int2))    => DCell::Int(int1 - int2),
                (DCell::UInt(int1), DCell::UInt(int2))  => DCell::UInt(int1 - int2),
                (DCell::Float(f1), DCell::Float(f2))    => DCell::Float(f1 - f2),
                (DCell::Bool(b1), DCell::Bool(b2))      => DCell::Bool(b1 || b2),
                _                                       => DCell::NA
            }
        }
    }
}

impl<T> Mul<T> for DCell where T: Into<DCell> + Typed {
    type Output = DCell;

    fn mul(self, other: T) -> Self::Output {
        match (self, other.into()) {
            (DCell::Int(int1), DCell::Int(int2))    => DCell::Int(int1 + int2),
            (DCell::UInt(int1), DCell::UInt(int2))  => DCell::UInt(int1 + int2),
            (DCell::Float(f1), DCell::Float(f2))    => DCell::Float(f1 + f2),
            (DCell::Bool(b1), DCell::Bool(b2))      => DCell::Bool((b1 || b2) && !(b1 && b2)),
            (DCell::Char(ch1), DCell::UInt(int2))   => DCell::Text(ch1.to_string().repeat(int2 as usize)),
            (DCell::Text(txt1), DCell::UInt(int2))  => DCell::Text(txt1.repeat(int2 as usize)),
            _                                       => DCell::NA
        }
    }
}

impl<T> MulAssign<T> for DCell where T: Into<DCell> + Typed {
    fn mul_assign(&mut self, other: T) {
        *self = match (self.clone(), other.into()) {
            (DCell::Int(int1), DCell::Int(int2))    => DCell::Int(int1 * int2),
            (DCell::UInt(int1), DCell::UInt(int2))  => DCell::UInt(int1 * int2),
            (DCell::Float(f1), DCell::Float(f2))    => DCell::Float(f1 * f2),
            (DCell::Bool(b1), DCell::Bool(b2))      => DCell::Bool((b1 || b2) && !(b1 && b2)),
            (DCell::Char(ch1), DCell::UInt(int2))   => DCell::Text(ch1.to_string().repeat(int2 as usize)),
            (DCell::Text(txt1), DCell::UInt(int2))  => DCell::Text(txt1.repeat(int2 as usize)),
            _                                       => DCell::NA
        }
    }
}

impl<T> Div<T> for DCell where T: Into<DCell> + Typed {
    type Output = DCell;

    fn div(self, other: T) -> Self::Output {
        if self.dtype() != other.dtype() {
            DCell::NA
        } else {
            match (self, other.into()) {
                (DCell::Int(int1), DCell::Int(int2))    => DCell::Int(int1 / int2),
                (DCell::UInt(int1), DCell::UInt(int2))  => DCell::UInt(int1 / int2),
                (DCell::Float(f1), DCell::Float(f2))    => DCell::Float(f1 / f2),
                _                                       => DCell::NA
            }
        }
    }
}

impl<T> DivAssign<T> for DCell where T: Into<DCell> + Typed {
    fn div_assign(&mut self, other: T) {
        *self = if self.dtype() != other.dtype() {
            DCell::NA
        } else {
            match (self.clone(), other.into()) {
                (DCell::Int(int1), DCell::Int(int2))    => DCell::Int(int1 / int2),
                (DCell::UInt(int1), DCell::UInt(int2))  => DCell::UInt(int1 / int2),
                (DCell::Float(f1), DCell::Float(f2))    => DCell::Float(f1 / f2),
                _                                       => DCell::NA
            }
        }
    }
}

from_trait_for_int!(i8);
from_trait_for_int!(i16);
from_trait_for_int!(i32);
from_trait_for_int!(i64);
from_trait_for_uint!(u8);
from_trait_for_uint!(u16);
from_trait_for_uint!(u32);
from_trait_for_uint!(u64);
from_trait_for_float!(f32);
from_trait_for_float!(f64);

impl From<char> for DCell {
    fn from(val: char) -> DCell {
        DCell::Char(val)
    }
}

impl From<bool> for DCell {
    fn from(val: bool) -> DCell {
        DCell::Bool(val)
    }
}

impl From<String> for DCell {
    fn from(val: String) -> DCell {
        DCell::Text(val)
    }
}

impl<'a> From<&'a str> for DCell {
    fn from(val: &'a str) -> DCell {
        DCell::Text(val.to_string())
    }
}

impl TryFrom<DCell> for i64 {
    type Error = RaccoonError;

    fn try_from(val: DCell) -> Result<Self, Self::Error> {
        if let DCell::Int(int) = val { Ok(int) } else { Err(RaccoonError::ConversionError) }
    }
}

impl TryFrom<DCell> for u64 {
    type Error = RaccoonError;

    fn try_from(val: DCell) -> Result<Self, Self::Error> {
        if let DCell::UInt(int) = val { Ok(int) } else { Err(RaccoonError::ConversionError) }
    }
}

impl TryFrom<DCell> for f64 {
    type Error = RaccoonError;

    fn try_from(val: DCell) -> Result<Self, Self::Error> {
        if let DCell::Float(f) = val { Ok(f) } else { Err(RaccoonError::ConversionError) }
    }
}

impl TryFrom<DCell> for char {
    type Error = RaccoonError;

    fn try_from(val: DCell) -> Result<Self, Self::Error> {
        if let DCell::Char(ch) = val { Ok(ch) } else { Err(RaccoonError::ConversionError) }
    }
}

impl TryFrom<DCell> for bool {
    type Error = RaccoonError;

    fn try_from(val: DCell) -> Result<Self, Self::Error> {
        if let DCell::Bool(b) = val { Ok(b) } else { Err(RaccoonError::ConversionError) }
    }
}

impl TryFrom<DCell> for String {
    type Error = RaccoonError;

    fn try_from(val: DCell) -> Result<Self, Self::Error> {
        if let DCell::Text(txt) = val { Ok(txt) } else { Err(RaccoonError::ConversionError) }
    }
}
