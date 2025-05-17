#[cfg(test)]
mod tests {
    use inventory_kit::error::InventoryError;
    use inventory_kit::in_memory;
    use inventory_kit::model::InventoryItem;
    use inventory_kit::service::{AtomicInventoryOps, InventoryRepository};

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    struct HotelRoom {
        id: u32,
    }

    impl InventoryItem for HotelRoom {
        type Id = u32;

        fn id(&self) -> Self::Id {
            self.id
        }
    }

    #[test]
    fn test_reserve_and_release() {
        let mut repo = in_memory::InMemoryInventoryRepository::<HotelRoom, u64>::new();

        let room = HotelRoom { id: 1 };
        repo.insert_availability(room.id(), 1000, 2000, 10);

        assert_eq!(
            repo.get_availability(&room.id(), 1000, 2000).unwrap()[0].available,
            10
        );

        repo.reserve(&room.id(), 1000, 2000, 3).unwrap();
        assert_eq!(
            repo.get_availability(&room.id(), 1000, 2000).unwrap()[0].available,
            7
        );

        repo.release(&room.id(), 1000, 2000, 2).unwrap();
        assert_eq!(
            repo.get_availability(&room.id(), 1000, 2000).unwrap()[0].available,
            9
        );
    }

    #[test]
    fn test_atomic_reserve_success_and_fail() {
        let mut repo = in_memory::InMemoryInventoryRepository::<HotelRoom, u64>::new();
        let room_id = 1;
        repo.insert_availability(room_id, 1000, 1100, 5);
        repo.insert_availability(room_id, 1100, 1200, 3);

        // Should succeed
        let result = repo.reserve_many(&room_id, &[
            (1000, 1100, 2),
            (1100, 1200, 1),
        ]);
        assert!(result.is_ok());

        // Should fail (not enough in second slot)
        let result = repo.reserve_many(&room_id, &[
            (1000, 1100, 2),
            (1100, 1200, 5), // not enough
        ]);
        assert_eq!(result, Err(InventoryError::Insufficient));

        // Ensure nothing changed after failed reserve
        let slots = repo.get_availability(&room_id, 1000, 1200).unwrap();
        assert_eq!(slots[0].available, 3); // 5 - 2
        assert_eq!(slots[1].available, 2); // 3 - 1
    }

}


