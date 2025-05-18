use crate::error::InventoryError;

/// Validates that a given time range is logically correct and within bounds.
///
/// This utility is meant to be reused by implementations that use time as `u32`-like values,
/// where times are expected to be in a 24-hour format (e.g., `1300` for 1:00 PM).
///
/// # Parameters
/// - `start_time`: The starting time of the range.
/// - `end_time`: The ending time of the range.
///
/// # Returns
/// - `Ok(())` if the time range is valid.
/// - `Err(InventoryError::InvalidTimeRange)` if `end_time` is not greater than `start_time`.
/// - `Err(InventoryError::Internal)` if conversion to `u32` fails or time exceeds 2400.
///
/// # Type Constraints
/// - `Time` must implement `Copy` and `TryInto<u32>`, which allows flexible generic usage.
///
/// # Examples
/// ```
/// use inventory_kit::utils::validate_time_range;
/// assert!(validate_time_range(900u32, 1200u32).is_ok());
/// assert!(validate_time_range(1500u32, 1400u32).is_err());
/// ```
pub fn validate_time_range<Time: Copy + TryInto<u32>>(
    start_time: Time,
    end_time: Time,
) -> Result<(), InventoryError> {
    let start: u32 = start_time.try_into().map_err(|_| {
        InventoryError::Internal("Failed to convert start_time to u32".to_string())
    })?;
    let end: u32 = end_time.try_into().map_err(|_| {
        InventoryError::Internal("Failed to convert end_time to u32".to_string())
    })?;

    if end <= start {
        return Err(InventoryError::InvalidTimeRange);
    }

    if end >= 2400 {
        return Err(InventoryError::Internal("Time must be less than 2400".to_string()));
    }

    Ok(())
}
