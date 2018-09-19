//! Series module

use prelude::*;

use std::fmt::Debug;
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


impl SeriesLike for Series {
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

impl<T> From<Vec<T>> for Series where T: Into<DCell> + Typed {
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

impl<T> From<Vec<T>> for MixedSeries where T: Into<DCell> + Typed {
    fn from(vector: Vec<T>) -> Self {
        let cells: Vec<DCell> = vector.into_iter().map(|x| x.into()).collect();
        MixedSeries {
            name: None,
            cells: cells,
        }
    }
}

/// Provide common series functionality.
pub trait SeriesLike: Debug + ToString + Typed + Index<usize> {
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
}
