use core::fmt::Debug;
use core::hash::Hash;

/// Represents a domain object that can be tracked and managed in the inventory system.
///
/// Implementors must define an associated `Id` type and provide a method to retrieve the identifier.
/// The item itself must be clonable, equatable, debuggable, and hashable to support generic inventory operations.
pub trait InventoryItem: Clone + Debug + Eq + Hash {
    /// The identifier type used to uniquely represent each inventory item.
    type Id: Clone + Debug + Eq + Hash;

    /// Returns the identifier of the item.
    fn id(&self) -> Self::Id;
}

/// Represents a time-bound availability slot for a specific inventory item.
///
/// Each slot defines a range (start to end) during which a specific quantity of the item is available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvailabilitySlot<ItemId, Time>
where
    ItemId: Clone + Eq + Hash,
    Time: Copy + Ord,
{
    /// The ID of the item this slot belongs to.
    pub item_id: ItemId,

    /// The start time of the availability period.
    pub start: Time,

    /// The end time of the availability period.
    pub end: Time,

    /// The number of units available during this time slot.
    pub available: u32,
}
