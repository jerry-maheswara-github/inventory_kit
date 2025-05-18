use std::collections::HashMap;
use std::hash::Hash;

use crate::repository::{AtomicInventoryOps, InventoryRepository};
use crate::error::InventoryError;
use crate::model::{AvailabilitySlot, InventoryItem};
use crate::utils::validate_time_range;

/// An in-memory implementation of an inventory repository.
///
/// This implementation stores inventory data in a `HashMap`, making it suitable for testing,
/// prototyping, or simple use cases without the need for persistent storage.
#[derive(Debug)]
pub struct InMemoryInventoryRepository<Item, Time>
where
    Item: InventoryItem,
    Time: Copy + Ord + Hash + Eq,
{
    storage: HashMap<Item::Id, Vec<AvailabilitySlot<Item::Id, Time>>>,
}

impl<Item, Time> InMemoryInventoryRepository<Item, Time>
where
    Item: InventoryItem,
    Time: Copy + Ord + Hash + Eq,
{
    /// Creates a new, empty in-memory inventory repository.
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    /// Inserts a new availability slot for a specific item.
    ///
    /// This does not perform validation. Callers must ensure the `from` and `to` range is valid.
    ///
    /// # Parameters
    /// - `item_id`: Identifier of the item.
    /// - `from`: Start of the availability period.
    /// - `to`: End of the availability period.
    /// - `quantity`: Number of available units.
    pub fn insert_availability(&mut self, item_id: Item::Id, from: Time, to: Time, quantity: u32) {
        let slots = self.storage.entry(item_id.clone()).or_default();
        slots.push(AvailabilitySlot {
            item_id,
            start: from,
            end: to,
            available: quantity,
        });
    }

    /// Finds a mutable reference to an availability slot.
    ///
    /// Returns `None` if the slot is not found.
    fn find_slot_mut(&mut self, item_id: &Item::Id, from: Time, to: Time) -> Option<&mut AvailabilitySlot<Item::Id, Time>> {
        self.storage.get_mut(item_id)?.iter_mut().find(|slot| {
            slot.start == from && slot.end == to
        })
    }
}

impl<Item, Time> InventoryRepository<Item, Time> for InMemoryInventoryRepository<Item, Time>
where
    Item: InventoryItem,
    Time: Copy + Ord + Hash + Eq,
    u32: From<Time>
{
    /// Retrieves available slots for a specific item between a given time range.
    fn get_availability(&self, item_id: &Item::Id, from: Time, to: Time) -> Result<Vec<AvailabilitySlot<Item::Id, Time>>, InventoryError> {
        validate_time_range::<Time>(from.into(), to.into())?;
        let slots = self.storage.get(item_id).ok_or(InventoryError::NotFound)?;
        let result = slots.iter()
            .filter(|s| s.start >= from && s.end <= to)
            .cloned()
            .collect();
        Ok(result)
    }

    /// Attempts to reserve a quantity of items within a given time slot.
    fn reserve(&mut self, item_id: &Item::Id, from: Time, to: Time, quantity: u32) -> Result<(), InventoryError> {
        validate_time_range(from, to)?;
        let slot = self.find_slot_mut(item_id, from, to).ok_or(InventoryError::NotFound)?;
        if slot.available < quantity {
            return Err(InventoryError::Insufficient);
        }
        slot.available -= quantity;
        Ok(())
    }

    /// Releases a quantity of items back to the slot.
    fn release(&mut self, item_id: &Item::Id, from: Time, to: Time, quantity: u32) -> Result<(), InventoryError> {
        let slot = self.find_slot_mut(item_id, from, to).ok_or(InventoryError::NotFound)?;
        slot.available += quantity;
        Ok(())
    }

    /// Adjusts the availability of a specific slot to a new quantity.
    fn adjust(&mut self, item_id: &Item::Id, from: Time, to: Time, new_quantity: u32) -> Result<(), InventoryError> {
        let slot = self.find_slot_mut(item_id, from, to).ok_or(InventoryError::NotFound)?;
        slot.available = new_quantity;
        Ok(())
    }
}

impl<Item, Time> AtomicInventoryOps<Item, Time> for InMemoryInventoryRepository<Item, Time>
where
    Item: InventoryItem,
    Time: Copy + Ord + Hash + Eq,
    u32: From<Time>
{
    /// Attempts to reserve multiple time slots atomically.
    ///
    /// If any of the reservations fail (e.g., not found or insufficient quantity),
    /// the operation is aborted without modifying any slot.
    fn reserve_many(&mut self, item_id: &Item::Id, slots: &[(Time, Time, u32)]) -> Result<(), InventoryError> {
        let storage = self
            .storage
            .get_mut(item_id)
            .ok_or(InventoryError::NotFound)?;

        let mut targets = Vec::with_capacity(slots.len());

        for (start, end, qty) in slots {
            let index = storage.iter().position(|s| s.start == *start && s.end == *end)
                .ok_or(InventoryError::NotFound)?;

            if storage[index].available < *qty {
                return Err(InventoryError::Insufficient);
            }

            targets.push((index, *qty));
        }

        for (index, qty) in targets {
            storage[index].available -= qty;
        }

        Ok(())
    }
}
