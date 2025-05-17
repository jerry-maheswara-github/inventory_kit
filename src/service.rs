use crate::model::*;
use crate::error::InventoryError;

/// Trait untuk mengelola inventori
pub trait InventoryRepository<Item, Time>
where
    Item: InventoryItem,
    Time: Copy + Ord,
{
    fn get_availability(
        &self,
        item_id: &Item::Id,
        from: Time,
        to: Time,
    ) -> Result<Vec<AvailabilitySlot<Item::Id, Time>>, InventoryError>;

    fn reserve(
        &mut self,
        item_id: &Item::Id,
        from: Time,
        to: Time,
        quantity: u32,
    ) -> Result<(), InventoryError>;

    fn release(
        &mut self,
        item_id: &Item::Id,
        from: Time,
        to: Time,
        quantity: u32,
    ) -> Result<(), InventoryError>;

    fn adjust(
        &mut self,
        item_id: &Item::Id,
        from: Time,
        to: Time,
        new_quantity: u32,
    ) -> Result<(), InventoryError>;
}

pub trait AtomicInventoryOps<Item, Time>: InventoryRepository<Item, Time>
where
    Item: InventoryItem,
    Time: Copy + Ord,
{
    /// Reserve banyak slot sekaligus, secara atomik
    fn reserve_many(
        &mut self,
        item_id: &Item::Id,
        slots: &[(Time, Time, u32)],
    ) -> Result<(), InventoryError>;
}
