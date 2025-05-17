use std::collections::HashMap;
use std::hash::Hash;

use crate::model::*;
use crate::service::{AtomicInventoryOps, InventoryRepository};
use crate::error::InventoryError;

/// In-memory inventory repository
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
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn insert_availability(
        &mut self,
        item_id: Item::Id,
        from: Time,
        to: Time,
        quantity: u32,
    ) {
        let slots = self.storage.entry(item_id.clone()).or_default();
        slots.push(AvailabilitySlot {
            item_id,
            start: from,
            end: to,
            available: quantity,
        });
    }

    fn find_slot_mut(
        &mut self,
        item_id: &Item::Id,
        from: Time,
        to: Time,
    ) -> Option<&mut AvailabilitySlot<Item::Id, Time>> {
        self.storage.get_mut(item_id)?.iter_mut().find(|slot| {
            slot.start == from && slot.end == to
        })
    }
}

impl<Item, Time> InventoryRepository<Item, Time> for InMemoryInventoryRepository<Item, Time>
where
    Item: InventoryItem,
    Time: Copy + Ord + Hash + Eq,
{
    fn get_availability(
        &self,
        item_id: &Item::Id,
        from: Time,
        to: Time,
    ) -> Result<Vec<AvailabilitySlot<Item::Id, Time>>, InventoryError> {
        let slots = self.storage.get(item_id).ok_or(InventoryError::NotFound)?;
        let result = slots.iter()
            .filter(|s| s.start >= from && s.end <= to)
            .cloned()
            .collect();
        Ok(result)
    }

    fn reserve(
        &mut self,
        item_id: &Item::Id,
        from: Time,
        to: Time,
        quantity: u32,
    ) -> Result<(), InventoryError> {
        let slot = self.find_slot_mut(item_id, from, to).ok_or(InventoryError::NotFound)?;
        if slot.available < quantity {
            return Err(InventoryError::Insufficient);
        }
        slot.available -= quantity;
        Ok(())
    }

    fn release(
        &mut self,
        item_id: &Item::Id,
        from: Time,
        to: Time,
        quantity: u32,
    ) -> Result<(), InventoryError> {
        let slot = self.find_slot_mut(item_id, from, to).ok_or(InventoryError::NotFound)?;
        slot.available += quantity;
        Ok(())
    }

    fn adjust(
        &mut self,
        item_id: &Item::Id,
        from: Time,
        to: Time,
        new_quantity: u32,
    ) -> Result<(), InventoryError> {
        let slot = self.find_slot_mut(item_id, from, to).ok_or(InventoryError::NotFound)?;
        slot.available = new_quantity;
        Ok(())
    }
}

impl<Item, Time> AtomicInventoryOps<Item, Time> for InMemoryInventoryRepository<Item, Time>
where
    Item: InventoryItem,
    Time: Copy + Ord + Hash + Eq,
{
    fn reserve_many(
        &mut self,
        item_id: &Item::Id,
        slots: &[(Time, Time, u32)],
    ) -> Result<(), InventoryError> {
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
