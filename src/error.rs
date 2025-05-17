#[derive(Debug, thiserror::Error, PartialEq)]
pub enum InventoryError {
    #[error("Item not found")]
    NotFound,

    #[error("Insufficient availability")]
    Insufficient,

    #[error("Internal error: {0}")]
    Internal(String),
}
