//! Error module.

use std::error::Error;
use std::fmt;

/// The result type used by the `raccoon` library.
pub type RaccoonResult = ::std::result::Result<(), RaccoonError>;

/// Raccoon error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RaccoonError {
    /// Invalid type.
    InvalidType,
    /// Conversion error.
    ConversionError,
    ///Mixed type error.
    MixedTypeError,
}

impl fmt::Display for RaccoonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RaccoonError::InvalidType       => write!(f, "invalid type provided"),
            RaccoonError::ConversionError   => write!(f, "error converting types"),
            RaccoonError::MixedTypeError    => write!(f, "mixed types are not allowed"),
        }
    }
}


impl Error for RaccoonError {
    fn description(&self) -> &str {
        match self {
            RaccoonError::InvalidType       => "invalid type provided",
            RaccoonError::ConversionError   => "error converting types",
            RaccoonError::MixedTypeError    => "mixed types are not allowed",
        }
    }
}




