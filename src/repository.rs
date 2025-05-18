use crate::model::*;
use crate::error::InventoryError;

/// Defines the core operations for managing inventory availability and transactions.
///
/// This trait is generic over both the inventory `Item` and the time unit `Time`.
/// It supports querying availability and managing quantities through reservation, release, and adjustment.
pub trait InventoryRepository<Item, Time>
where
    Item: InventoryItem,
    Time: Copy + Ord,
{
    /// Retrieves all availability slots for a given item within a specified time range.
    ///
    /// Returns a list of matching `AvailabilitySlot`s or an `InventoryError` if the item is not found
    /// or the range is invalid.
    fn get_availability(
        &self,
        item_id: &Item::Id,
        from: Time,
        to: Time,
    ) -> Result<Vec<AvailabilitySlot<Item::Id, Time>>, InventoryError>;

    /// Attempts to reserve a specified quantity of an item for a given time range.
    ///
    /// Returns an error if the item is not found, the slot is not defined, or there is insufficient availability.
    fn reserve(
        &mut self,
        item_id: &Item::Id,
        from: Time,
        to: Time,
        quantity: u32,
    ) -> Result<(), InventoryError>;

    /// Releases a previously reserved quantity back into availability for the given time range.
    ///
    /// This operation increases the `available` value of the targeted slot.
    fn release(
        &mut self,
        item_id: &Item::Id,
        from: Time,
        to: Time,
        quantity: u32,
    ) -> Result<(), InventoryError>;

    /// Adjusts the total availability for a given slot to a new quantity.
    ///
    /// This operation sets the `available` value of the slot to the specified amount,
    /// regardless of its current value.
    fn adjust(
        &mut self,
        item_id: &Item::Id,
        from: Time,
        to: Time,
        new_quantity: u32,
    ) -> Result<(), InventoryError>;
}

/// An extension of `InventoryRepository` that supports atomic operations across multiple slots.
///
/// Useful for use cases like batch reservations, where all operations must succeed together or fail entirely.
pub trait AtomicInventoryOps<Item, Time>: InventoryRepository<Item, Time>
where
    Item: InventoryItem,
    Time: Copy + Ord,
{
    /// Attempts to reserve multiple availability slots in a single atomic operation.
    ///
    /// If any reservation fails (due to not found or insufficient availability),
    /// no changes will be applied to any of the slots.
    ///
    /// # Parameters
    /// - `item_id`: The ID of the item to reserve.
    /// - `slots`: A slice of `(start_time, end_time, quantity)` tuples to reserve.
    fn reserve_many(
        &mut self,
        item_id: &Item::Id,
        slots: &[(Time, Time, u32)],
    ) -> Result<(), InventoryError>;
}
