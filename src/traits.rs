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

/// Convert into another type.
pub trait AsType: Typed {
    /// Perform the conversion.
    ///
    /// # Examples
    /// ```
    /// # use raccoon::prelude::*;
    /// let mut cell = DCell::from(true);
    /// assert_eq!(cell, DCell::Bool(true));
    ///
    /// cell.astype(DType::Int);
    /// assert_eq!(cell, DCell::Int(1i64))
    /// ```
    ///
    /// Converting to booleans checks for equality with 0.
    /// ```
    /// # use raccoon::prelude::*;
    /// let mut mseries = MixedSeries::from(vec![1, 0, 0, 5]);
    /// mseries.push(true);
    /// mseries.push(6.43);
    ///
    /// // perform the conversion
    /// mseries.astype(DType::Bool);
    /// let expected = vec![
    ///     DCell::Bool(true),
    ///     DCell::Bool(false),
    ///     DCell::Bool(false),
    ///     DCell::Bool(true),
    ///     DCell::Bool(true),
    ///     DCell::Bool(true)
    /// ];
    /// assert_eq!(mseries, expected);
    /// ```
    fn astype(&mut self, dtype: DType);
}

/// Trait implemented by all primitive data values.
pub trait Primitive: Typed {}

typed_trait_for_int!(i8);
typed_trait_for_int!(i16);
typed_trait_for_int!(i32);
typed_trait_for_int!(i64);
typed_trait_for_uint!(u8);
typed_trait_for_uint!(u16);
typed_trait_for_uint!(u32);
typed_trait_for_uint!(u64);
typed_trait_for_float!(f32);
typed_trait_for_float!(f64);

impl Typed for bool {
    fn dtype(&self) -> DType { DType::Bool }
}

impl Primitive for bool {}

impl Typed for char {
    fn dtype(&self) -> DType { DType::Char }
}

impl Primitive for char {}

impl Typed for String {
    fn dtype(&self) -> DType { DType::Text }
}

impl Primitive for String {}
