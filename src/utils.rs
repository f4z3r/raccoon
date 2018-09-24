//! Utility module.

use prelude::*;

/// Returns the type of a vector of `DCell`.
///
/// # Example
/// ```ignore
/// # use raccoon::prelude::*;
/// let vector = vec![
///     DCell::NA,
///     DCell::Int(56i64),
///     DCell::NA,
///     DCell::Int(-123i64),
///     DCell::Int(902_348i64),
///     DCell::NA,
/// ];
/// assert_eq!(vec_dtype(&vector), DType::Int);
/// ```
pub fn vec_dtype(vector: &Vec<DCell>) -> DType {
    let mut result = DType::NA;
    for cell in vector {
        let cell_type = cell.dtype();
        if cell_type != result && !result.is_nan() && !cell_type.is_nan() {
            return DType::Mixed;
        } else if cell_type != result && result.is_nan() {
            result = cell_type;
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_type_check() {
        let vector = vec![
            DCell::NA,
            DCell::Int(56i64),
            DCell::NA,
            DCell::Int(-123i64),
            DCell::Int(902_348i64),
            DCell::NA,
        ];
        assert_eq!(vec_dtype(&vector), DType::Int);

        // mixed type
        let vector = vec![
            DCell::NA,
            DCell::Int(56i64),
            DCell::NA,
            DCell::Bool(true),
            DCell::Int(902_348i64),
            DCell::NA,
        ];
        assert_eq!(vec_dtype(&vector), DType::Mixed);

        // NA type
        let vector = vec![
            DCell::NA,
            DCell::NA,
            DCell::NA,
            DCell::NA,
            DCell::NA,
            DCell::NA,
        ];
        assert_eq!(vec_dtype(&vector), DType::NA);
    }
}
