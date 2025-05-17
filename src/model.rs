use core::fmt::Debug;
use core::hash::Hash;

/// Representasi item yang memiliki stok
pub trait InventoryItem: Clone + Debug + Eq + Hash {
    type Id: Clone + Debug + Eq + Hash;

    fn id(&self) -> Self::Id;
}

/// Slot ketersediaan dalam rentang waktu
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvailabilitySlot<ItemId, Time>
where
    ItemId: Clone + Eq + Hash,
    Time: Copy + Ord,
{
    pub item_id: ItemId,
    pub start: Time,
    pub end: Time,
    pub available: u32,
}
