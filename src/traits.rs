//! Trait module.

use prelude::*;

use std::fmt::Debug;

/// Attempt to construct `Self` via conversion.
pub trait TryFrom<T>: Sized {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Perform the conversion.
    fn try_from(val: T) -> Result<Self, Self::Error>;
}

/// Extract data type for an entity.
pub trait Typed: Debug + ToString {
    /// Extracts the data type.
    fn dtype(&self) -> DType;
}

impl Typed for i8 {
    fn dtype(&self) -> DType { DType::Int }
}

impl Typed for i16 {
    fn dtype(&self) -> DType { DType::Int }
}

impl Typed for i32 {
    fn dtype(&self) -> DType { DType::Int }
}

impl Typed for i64 {
    fn dtype(&self) -> DType { DType::Int }
}

impl Typed for u8 {
    fn dtype(&self) -> DType { DType::UInt }
}

impl Typed for u16 {
    fn dtype(&self) -> DType { DType::UInt }
}

impl Typed for u32 {
    fn dtype(&self) -> DType { DType::UInt }
}

impl Typed for u64 {
    fn dtype(&self) -> DType { DType::UInt }
}

impl Typed for f32 {
    fn dtype(&self) -> DType { DType::Float }
}

impl Typed for f64 {
    fn dtype(&self) -> DType { DType::Float }
}

impl Typed for bool {
    fn dtype(&self) -> DType { DType::Bool }
}

impl Typed for char {
    fn dtype(&self) -> DType { DType::Char }
}

impl Typed for String {
    fn dtype(&self) -> DType { DType::Text }
}
