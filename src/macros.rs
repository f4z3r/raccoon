//! Macro module

macro_rules! from_trait_for_int {
    ($type:ty) => {
        impl From<$type> for DCell {
            fn from(val: $type) -> Self {
                DCell::Int(val as i64)
            }
        }
    }
}

macro_rules! from_trait_for_uint {
    ($type:ty) => {
        impl From<$type> for DCell {
            fn from(val: $type) -> Self {
                DCell::UInt(val as u64)
            }
        }
    }
}

macro_rules! from_trait_for_float {
    ($type:ty) => {
        impl From<$type> for DCell {
            fn from(val: $type) -> Self {
                DCell::Float(val as f64)
            }
        }
    }
}
