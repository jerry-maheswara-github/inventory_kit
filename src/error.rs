/// Represents possible errors that can occur during inventory operations.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum InventoryError {
    /// Returned when the requested item does not exist in the repository.
    #[error("Item not found")]
    NotFound,

    /// Returned when the requested quantity exceeds the available units.
    #[error("Insufficient availability")]
    Insufficient,

    /// A generic internal error with an attached message.
    #[error("Internal error: {0}")]
    Internal(String),

    /// Returned when the provided time range is invalid,
    /// such as when the start time is greater than or equal to the end time.
    #[error("Invalid time range: start time must be before end time")]
    InvalidTimeRange,
}
