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

impl DCell {
    /// Determine if the `DCell` is of type `DType::NA`
    ///
    /// # Example
    /// ```
    /// # use raccoon::prelude::*;
    /// let mut cell = DCell::Int(89);
    /// cell += true;
    /// assert!(cell.is_nan());
    /// ```
    pub fn is_nan(&self) -> bool {
        match self {
            DCell::NA   => true,
            _           => false
        }
    }

    /// Parse a `DCell` from a `&str`.
    ///
    /// # Example
    /// ```
    /// # use raccoon::prelude::*;
    /// let cell = DCell::from_str("54");
    /// assert_eq!(cell, DCell::UInt(54u64));
    ///
    /// let cell = DCell::from_str("-123");
    /// assert_eq!(cell, DCell::Int(-123i64));
    ///
    /// let cell = DCell::from_str("90.87");
    /// assert_eq!(cell, DCell::Float(90.87f64));
    ///
    /// let cell = DCell::from_str("true");
    /// assert_eq!(cell, DCell::Bool(true));
    ///
    /// let cell = DCell::from_str("a");
    /// assert_eq!(cell, DCell::Char('a'));
    ///
    /// let cell = DCell::from_str("some random text");
    /// assert_eq!(cell, DCell::Text("some random text".to_owned()));
    /// ```
    pub fn from_str<T>(val: T) -> DCell where T: Into<String> {
        let val: String = val.into();
        if let Some(int) = val.parse::<u64>().ok() {
            return DCell::UInt(int);
        }

        if let Some(int) = val.parse::<i64>().ok() {
            return DCell::Int(int);
        }

        if let Some(float) = val.parse::<f64>().ok() {
            return DCell::Float(float);
        }

        if let Some(b) = val.parse::<bool>().ok() {
            return DCell::Bool(b);
        }

        if let Some(ch) = val.parse::<char>().ok() {
            return DCell::Char(ch);
        }

        DCell::Text(val)
    }
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

impl AsType for DCell {
    fn astype(&mut self, dtype: DType) {
        *self = match (self.clone(), dtype) {
            (DCell::Int(int), DType::Int)       => DCell::Int(int),
            (DCell::Float(f), DType::Int)       => DCell::Int(f as i64),
            (DCell::Bool(b), DType::Int)        => DCell::Int(b as i64),
            (DCell::Text(txt), DType::Int)      => if let Some(int) = txt.parse::<i64>().ok() { DCell::Int(int) } else { DCell::NA },
            (DCell::UInt(int), DType::UInt)     => DCell::UInt(int),
            (DCell::Bool(b), DType::UInt)       => DCell::UInt(b as u64),
            (DCell::Text(txt), DType::UInt)     => if let Some(int) = txt.parse::<u64>().ok() { DCell::UInt(int) } else { DCell::NA },
            (DCell::Int(int), DType::Float)     => DCell::Float(int as f64),
            (DCell::UInt(int), DType::Float)    => DCell::Float(int as f64),
            (DCell::Float(f), DType::Float)     => DCell::Float(f),
            (DCell::Bool(b), DType::Float)      => DCell::Float(b as u8 as f64),
            (DCell::Text(txt), DType::Float)    => if let Some(f) = txt.parse::<f64>().ok() { DCell::Float(f) } else { DCell::NA },
            (DCell::Int(int), DType::Bool)      => DCell::Bool(int != 0i64),
            (DCell::UInt(int), DType::Bool)     => DCell::Bool(int != 0u64),
            (DCell::Float(f), DType::Bool)      => DCell::Bool(f != 0f64),
            (DCell::Bool(b), DType::Bool)       => DCell::Bool(b),
            (DCell::Text(txt), DType::Bool)     => if let Some(b) = txt.parse::<bool>().ok() { DCell::Bool(b) } else { DCell::NA },
            (DCell::Char(ch), DType::Char)      => DCell::Char(ch),
            (DCell::Text(txt), DType::Char)     => if let Some(ch) = txt.parse::<char>().ok() { DCell::Char(ch) } else { DCell::NA },
            (DCell::Text(txt), DType::Text)     => DCell::Text(txt),
            (DCell::Int(int), DType::Text)      => DCell::Text(int.to_string()),
            (DCell::UInt(int), DType::Text)     => DCell::Text(int.to_string()),
            (DCell::Float(f), DType::Text)      => DCell::Text(f.to_string()),
            (DCell::Bool(b), DType::Text)       => DCell::Text(b.to_string()),
            (DCell::Char(ch), DType::Text)      => DCell::Text(ch.to_string()),
            _                                   => DCell::NA
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
    NA,
    /// Mixed
    Mixed
}

impl DType {
    /// Checks equality between types.
    ///
    /// Note that this is different to the `PartialEq` trait since, for instance, all types have the same type as
    /// `DType::NA`. This comes from the fact, that `DCell::NA` represents a missing entry, no matter what type the
    /// entry is.
    ///
    /// # Example
    /// ```
    /// # use raccoon::prelude::*;
    /// assert!(DType::Float.equals(&DType::Float));
    /// assert!(!DType::Float.equals(&DType::Int));
    /// assert!(DType::Float.equals(&DType::NA));
    ///
    /// // mixed does not behave like NA
    /// assert!(!DType::Float.equals(&DType::Mixed));
    /// ```
    pub fn equals(&self, other: &DType) -> bool {
        match (self, other) {
            (DType::Int,   DType::Int)      => true,
            (DType::UInt,  DType::UInt)     => true,
            (DType::Float, DType::Float)    => true,
            (DType::Char,  DType::Char)     => true,
            (DType::Bool,  DType::Bool)     => true,
            (DType::Text,  DType::Text)     => true,
            (DType::Mixed, DType::Mixed)    => true,
            (_,            DType::NA)       => true,
            (DType::NA,    _)               => true,
            _                               => false,
        }
    }
}

impl ToString for DType {
    fn to_string(&self) -> String {
        match self {
            DType::Int      => String::from("Integer"),
            DType::UInt     => String::from("Unsigned Integer"),
            DType::Float    => String::from("Floating Point Number"),
            DType::Bool     => String::from("Boolean"),
            DType::Char     => String::from("Character"),
            DType::Text     => String::from("Text"),
            DType::NA       => String::from("Invalid/Missing"),
            DType::Mixed    => String::from("Mixed"),
        }
    }
}

impl Typed for DType {
    fn dtype(&self) -> DType {
        self.clone()
    }
}

impl AsType for DType {
    fn astype(&mut self, dtype: DType) {
        *self = match (self.clone(), dtype) {
            (DType::Int, DType::Int)        => DType::Int,
            (DType::Float, DType::Int)      => DType::Int,
            (DType::Bool, DType::Int)       => DType::Int,
            (DType::Text, DType::Int)       => DType::Mixed,
            (DType::UInt, DType::UInt)      => DType::UInt,
            (DType::Bool, DType::UInt)      => DType::UInt,
            (DType::Text, DType::UInt)      => DType::Mixed,
            (DType::Int, DType::Float)      => DType::Float,
            (DType::UInt, DType::Float)     => DType::Float,
            (DType::Float, DType::Float)    => DType::Float,
            (DType::Bool, DType::Float)     => DType::Float,
            (DType::Text, DType::Float)     => DType::Mixed,
            (DType::Int, DType::Bool)       => DType::Bool,
            (DType::UInt, DType::Bool)      => DType::Bool,
            (DType::Float, DType::Bool)     => DType::Bool,
            (DType::Bool, DType::Bool)      => DType::Bool,
            (DType::Text, DType::Bool)      => DType::Mixed,
            (DType::Char, DType::Char)      => DType::Char,
            (DType::Text, DType::Char)      => DType::Mixed,
            (DType::Text, DType::Text)      => DType::Text,
            (DType::Int, DType::Text)       => DType::Text,
            (DType::UInt, DType::Text)      => DType::Text,
            (DType::Float, DType::Text)     => DType::Text,
            (DType::Bool, DType::Text)      => DType::Text,
            (DType::Char, DType::Text)      => DType::Text,
            _                               => DType::NA
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dtype_equality() {
        assert!(DType::Int.equals(&DType::Int));
        assert!(DType::Int.equals(&DType::NA));
        assert!(DType::NA.equals(&DType::Int));
        assert!(!DType::Text.equals(&DType::Int));
        assert!(!DType::Float.equals(&DType::Bool));
        assert!(DType::Char.equals(&DType::Char));
        assert!(DType::NA.equals(&DType::Float));
        assert!(DType::NA.equals(&DType::NA));
        assert!(!DType::Mixed.equals(&DType::Bool));
        assert!(DType::Mixed.equals(&DType::NA));
        assert!(DType::Mixed.equals(&DType::Mixed));
    }
}
