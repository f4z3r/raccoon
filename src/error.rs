//! Error module.

/// The result type used by the `raccoon` library.
pub type RaccoonResult = ::std::result::Result<(), RaccoonError>;

quick_error! {
    /// Contains all errors used by the `raccoon` library.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum RaccoonError {
        /// Invalid type. Thrown when an argument has an invalid type.
        InvalidType {
            description("Invalid data type")
            display("Invalid data type")
        }
    }
}

