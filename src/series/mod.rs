//! Series module

use prelude::*;
use utils;

use std::ops::{Index, IndexMut};

/// A growable, named series with a strict data type.
///
/// `Series` enforces its internal [`DType`](../cell/enum.DType.html), only accepting its own data type or `DType::NA`.
/// This is useful for rigorous data manipulations where the data is structured. If more flexibility is required,
/// use [`MixedSeries`](./struct.MixedSeries.html).
///
/// See [`SeriesLike`](./trait.SeriesLike.html) for most supported methods.
#[derive(Debug)]
pub struct Series {
    name: Option<String>,
    cells: Vec<DCell>,
    dtype: DType,
}

impl Series {
    /// Constructs a named series intialised with data.
    ///
    /// This can return an error if the data passed is not conform to the series type.
    ///
    /// # Example
    /// ```
    /// # use raccoon::prelude::*;
    /// let vector = vec![
    ///     DCell::Int(-213i64),
    ///     DCell::NA,
    ///     DCell::Bool(true),
    ///     DCell::Text("some text".to_owned())
    /// ];
    /// // may fail for mismatched types
    /// let result = Series::new_typed("my typed series", vector);
    /// assert!(result.is_err());
    ///
    /// // but can be useful to initiate `Series` with NA values
    /// let vector = vec![
    ///     DCell::Int(-213i64),
    ///     DCell::NA,
    ///     DCell::Int(-456i64),
    ///     DCell::Int(905_843i64)
    /// ];
    /// let result = Series::new_typed("my other typed series", vector.clone());
    /// assert!(result.is_ok());
    ///
    /// let series = result.unwrap();
    /// assert_eq!(series, vector);
    /// ```
    pub fn new_typed<T, U>(name: T, vector: Vec<U>) -> Result<Self, RaccoonError>
        where
            T: Into<String>,
            U: Into<DCell> + Typed
    {
        let name: String = name.into();
        let cells: Vec<DCell> = vector.into_iter().map(|x| x.into()).collect();
        let dtype = utils::vec_dtype(&cells);
        if dtype == DType::Mixed {
            return Err(RaccoonError::MixedTypeError);
        }
        Ok(Series {
            name: Some(name),
            cells: cells,
            dtype: dtype,
        })
    }
}

impl SeriesLike for Series {
    fn new<T, U>(name: T, vector: Vec<U>) -> Self where T: Into<String>, U: Into<DCell> + Primitive {
        let name: String = name.into();
        let cells: Vec<DCell> = vector.into_iter().map(|x| x.into()).collect();
        let dtype = utils::vec_dtype(&cells);
        if dtype == DType::Mixed {
            panic!("mixed type in type checked series");
        }
        Series {
            name: Some(name),
            cells: cells,
            dtype: dtype,
        }
    }

    fn len(&self) -> usize {
        self.cells.len()
    }

    fn push<T>(&mut self, val: T) -> RaccoonResult where T: Into<DCell> + Typed {
        self.push_cell(val.into())
    }

    fn push_cell(&mut self, cell: DCell) -> RaccoonResult {
        if !self.accepts(&cell) {
            Err(RaccoonError::InvalidType)
        } else {
            self.cells.push(cell);
            Ok(())
        }
    }

    fn accepts<T>(&self, val: &T) -> bool where T: Into<DCell> + Typed {
        if self.dtype() != val.dtype() && val.dtype() != DType::NA {
            false
        } else {
            true
        }
    }

    fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    fn set_name<T>(&mut self, name: T) where T: Into<String> {
        self.name = Some(name.into());
    }

    fn cells(&self) -> &Vec<DCell> {
        &self.cells
    }
}

impl<T> PartialEq<T> for Series where T: SeriesLike {
    fn eq(&self, other: &T) -> bool {
        if self.name() == other.name() && self.len() == other.len() {
            if self.cells.iter().zip(other.cells().iter()).all(|(ref x1, ref x2)| x1 == x2 ) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl<T> PartialEq<Vec<T>> for Series where DCell: From<T>, T: Clone {
    fn eq(&self, other: &Vec<T>) -> bool {
        if self.cells().iter().zip(other.iter()).all(|(x1, x2)| *x1 == DCell::from(x2.clone())) {
            true
        } else {
            false
        }
    }
}

impl Index<usize> for Series {
    type Output = DCell;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.cells[idx]
    }
}

impl ToString for Series {
    fn to_string(&self) -> String {
        match self.name {
            Some(ref name)  => name.clone(),
            None            => String::from("")
        }
    }
}

impl Typed for Series {
    fn dtype(&self) -> DType {
        self.dtype.clone()
    }
}

impl AsType for Series {
    fn astype(&mut self, dtype: DType) {
        for cell in &mut self.cells { cell.astype(dtype.clone()) }
        self.dtype = dtype;
    }
}

impl<T> From<Vec<T>> for Series where T: Into<DCell> + Primitive {
    fn from(vector: Vec<T>) -> Self {
        let cells: Vec<DCell> = vector.into_iter().map(|x| x.into()).collect();
        let mut dtype = DType::NA;
        if !cells.is_empty() {
            dtype = cells[0].dtype();
        }
        Series {
            name: None,
            cells: cells,
            dtype: dtype
        }
    }
}

/// A growable, name series with no strict type checking.
///
/// For strict type checking, please see [`Series`](./struct.Series.html).
///
/// See [`SeriesLike`](./trait.SeriesLike.html) for most supported methods.
#[derive(Debug)]
pub struct MixedSeries {
    name: Option<String>,
    cells: Vec<DCell>,
}

impl MixedSeries {
    /// Constructs a named series intialised with data.
    ///
    /// # Example
    /// ```
    /// # use raccoon::prelude::*;
    /// let vector = vec![
    ///     DCell::Int(-213i64),
    ///     DCell::NA,
    ///     DCell::Bool(true),
    ///     DCell::Text("some text".to_owned())
    /// ];
    /// // may fail for mismatched types
    /// let mseries = MixedSeries::new_typed("my typed series", vector.clone());
    ///
    /// assert_eq!(mseries, vector);
    /// ```
    pub fn new_typed<T, U>(name: T, vector: Vec<U>) -> Self
        where
            T: Into<String>,
            U: Into<DCell> + Typed
    {
        let name: String = name.into();
        let cells: Vec<DCell> = vector.into_iter().map(|x| x.into()).collect();
        MixedSeries {
            name: Some(name),
            cells: cells,
        }
    }

    /// Same as [`push()`](./trait.SeriesLike.html#tymethod.push) from [`SeriesLike`](./trait.SeriesLike.html) but
    /// without returing a `RaccoonResult` as the function cannot fail.
    ///
    /// # Example
    /// ```
    /// # use raccoon::prelude::*;
    /// let mut mseries = MixedSeries::from(vec![true, false, true]);
    /// mseries.force_push(23.45);
    /// assert_eq!(mseries[3], DCell::Float(23.45));
    /// ```
    pub fn force_push<T>(&mut self, val: T) where T: Into<DCell> + Typed {
        let _ = self.push(val);
    }

    /// Same as [`push_cell()`](./trait.SeriesLike.html#tymethod.push_cell) from [`SeriesLike`](./trait.SeriesLike.html)
    /// but without returing a `RaccoonResult` as the function cannot fail.
    ///
    /// # Example
    /// ```
    /// # use raccoon::prelude::*;
    /// let mut mseries = MixedSeries::from(vec![true, false, true]);
    /// mseries.force_push_cell(DCell::Float(23.45));
    /// assert_eq!(mseries[3], DCell::Float(23.45));
    /// ```
    pub fn force_push_cell(&mut self, cell: DCell) {
        let _ = self.push_cell(cell);
    }
}

impl SeriesLike for MixedSeries {
    fn new<T, U>(name: T, vector: Vec<U>) -> Self where T: Into<String>, U: Into<DCell> + Primitive {
        let name: String = name.into();
        let cells: Vec<DCell> = vector.into_iter().map(|x| x.into()).collect();
        MixedSeries {
            name: Some(name),
            cells: cells,
        }
    }

    fn len(&self) -> usize {
        self.cells.len()
    }

    fn push<T>(&mut self, val: T) -> RaccoonResult where T: Into<DCell> + Typed {
        self.push_cell(val.into())
    }

    fn push_cell(&mut self, cell: DCell) -> RaccoonResult {
        self.cells.push(cell);
        Ok(())
    }

    fn accepts<T>(&self, _val: &T) -> bool where T: Into<DCell> + Typed {
        true
    }

    fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }


    fn set_name<T>(&mut self, name: T) where T: Into<String> {
        self.name = Some(name.into());
    }

    fn cells(&self) -> &Vec<DCell> {
        &self.cells
    }
}

impl Index<usize> for MixedSeries {
    type Output = DCell;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.cells[idx]
    }
}

impl IndexMut<usize> for MixedSeries {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.cells[idx]
    }
}

impl ToString for MixedSeries {
    fn to_string(&self) -> String {
        match self.name {
            Some(ref name)  => name.clone(),
            None            => String::from("")
        }
    }
}

impl Typed for MixedSeries {
    fn dtype(&self) -> DType {
        DType::Mixed
    }
}

impl AsType for MixedSeries {
    fn astype(&mut self, dtype: DType) {
        for cell in &mut self.cells { cell.astype(dtype.clone()) }
    }
}

impl<T> From<Vec<T>> for MixedSeries where T: Into<DCell> + Typed {
    fn from(vector: Vec<T>) -> Self {
        let cells: Vec<DCell> = vector.into_iter().map(|x| x.into()).collect();
        MixedSeries {
            name: None,
            cells: cells,
        }
    }
}

impl<T> PartialEq<T> for MixedSeries where T: SeriesLike {
    fn eq(&self, other: &T) -> bool {
        if self.name() == other.name() && self.len() == other.len() {
            if self.cells.iter().zip(other.cells().iter()).all(|(ref x1, ref x2)| x1 == x2 ) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl<T> PartialEq<Vec<T>> for MixedSeries where DCell: From<T>, T: Clone {
    fn eq(&self, other: &Vec<T>) -> bool {
        if self.cells().iter().zip(other.iter()).all(|(x1, x2)| *x1 == DCell::from(x2.clone())) {
            true
        } else {
            false
        }
    }
}

/// Provide common series functionality.
pub trait SeriesLike: Index<usize> + AsType {
    /// Constructs a named series initialised with data.
    ///
    /// # Example
    /// ```
    /// # use raccoon::prelude::*;
    /// let series = MixedSeries::new("random chars", vec!['a', 'b', 'c']);
    /// assert_eq!(series.name(), Some(&"random chars".to_owned()));
    /// assert_eq!(series[1], DCell::Char('b'));
    /// ```
    fn new<T, U>(name: T, vector: Vec<U>) -> Self where T: Into<String>, U: Into<DCell> + Primitive;

    /// Returns the length of the series.
    ///
    /// # Example
    /// ```
    /// # use raccoon::prelude::*;
    /// let series = Series::from(vec![1, 2, 3, 4, 5]);
    /// assert_eq!(series.len(), 5);
    /// ```
    fn len(&self) -> usize;

    /// Pushes a value to the end of the series.
    ///
    /// # Example
    /// ```
    /// # use raccoon::prelude::*;
    /// let mut series = Series::from(vec![true, false, true]);
    /// assert_eq!(series.len(), 3);
    ///
    /// let result = series.push(false);
    /// assert!(result.is_ok());
    /// assert_eq!(series.len(), 4);
    ///
    /// let result = series.push(456);
    /// assert!(result.is_err());
    /// assert_eq!(series.len(), 4);
    /// ```
    fn push<T>(&mut self, val: T) -> RaccoonResult where T: Into<DCell> + Typed;

    /// Pushes a `DCell` to the end of the series.
    ///
    /// # Example
    /// ```
    /// # use raccoon::prelude::*;
    /// let mut series = Series::from(vec!['a', 'b', 'c']);
    /// assert_eq!(series.len(), 3);
    ///
    /// let result = series.push('d');
    /// assert!(result.is_ok());
    /// assert_eq!(series.len(), 4);
    ///
    /// let result = series.push(456);
    /// assert!(result.is_err());
    /// assert_eq!(series.len(), 4);
    /// ```
    fn push_cell(&mut self, cell: DCell) -> RaccoonResult;

    /// Checks if the series accepts that data.
    ///
    /// # Example
    /// ```
    /// # use raccoon::prelude::*;
    /// let series = Series::from(vec![4u32, 7u32, 9u32]);
    /// assert!(series.accepts(&7u32));
    /// assert!(series.accepts(&DCell::UInt(67u64)));
    /// assert!(series.accepts(&125u8));
    /// assert!(!series.accepts(&true));
    /// ```
    fn accepts<T>(&self, val: &T) -> bool where T: Into<DCell> + Typed;

    /// Gets the name of the series.
    ///
    /// # Example
    /// ```
    /// # use raccoon::prelude::*;
    /// let series = Series::new("My series", vec![1, 2, 3, 4]);
    /// assert_eq!(series.name(), Some(&"My series".to_owned()));
    /// ```
    fn name(&self) -> Option<&String>;


    /// Sets the name of the series.
    ///
    /// # Example
    /// ```
    /// # use raccoon::prelude::*;
    /// let mut series = MixedSeries::from(vec![true, false, true]);
    /// assert_eq!(series.name(), None);
    ///
    /// series.set_name("some name");
    /// assert_eq!(series.name(), Some(&"some name".to_owned()));
    /// ```
    fn set_name<T>(&mut self, name: T) where T: Into<String>;

    /// Gets a reference to the internal vector of cells of the series.
    ///
    /// # Example
    /// ```
    /// # use raccoon::prelude::*;
    /// let series = Series::from(vec!["some".to_owned(), "words".to_owned()]);
    ///
    /// let expected = vec![DCell::Text("some".to_owned()), DCell::Text("words".to_owned())];
    /// assert_eq!(series.cells(), &expected);
    /// ```
    fn cells(&self) -> &Vec<DCell>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        let series = Series::from(vec![0, 1, 2, 3]);
        let mut mseries = MixedSeries::from(vec![0, 1, 2, 3]);
        assert!(series == mseries);
        assert!(series == vec![0, 1, 2, 3]);
        assert!(mseries == vec![0, 1, 2, 3]);

        mseries.set_name("random");
        assert!(series != mseries);
        assert!(mseries == vec![0, 1, 2, 3]);
    }
}
