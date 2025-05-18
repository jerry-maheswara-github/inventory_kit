use inventory_kit::error::InventoryError;
use inventory_kit::in_memory::InMemoryInventoryRepository;
use inventory_kit::model::InventoryItem;
use inventory_kit::repository::InventoryRepository;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Product(u32);

impl InventoryItem for Product {
    type Id = u32;
    fn id(&self) -> Self::Id {
        self.0
    }
}

fn main() -> Result<(), InventoryError> {
    let mut repo = InMemoryInventoryRepository::<Product, u32>::new();

    let start_time = 10_00;
    let end_time = 20_00;

    let _ = repo.insert_availability(1, start_time, end_time, 5);
    print!("repo.insert_availability 5 (available:5) \n{:#?}\n", repo);
    
    match repo.reserve(&1, start_time, end_time, 3) {
        Ok(_) => println!("Reservation successful!"),
        Err(e) => return Err(e),
    }
    print!("repo.reserve 3 (available: 5-3=2) \n{:#?}\n", repo);

    match repo.release(&1, start_time, end_time, 1) {
        Ok(_) => println!("Release successful!"),
        Err(e) => return Err(e),
    }
    print!("repo.release 1 (available: 2+1=3) \n{:#?}\n", repo);

    match repo.adjust(&1, start_time, end_time, 10) {
        Ok(_) => println!("Adjust successful!"),
        Err(e) => return Err(e),
    }
    print!("repo.adjust 10 (available: 10) \n{:#?}\n", repo);

    let slots = repo.get_availability(&1, 0, 20_00)?;
    print!("slots item_id 1 \n{:#?}\n", slots);
    for slot in slots {
        println!(
            "Item {} is available for {} units between hour {} and hour {}.",
            slot.item_id, slot.available, slot.start, slot.end
        );
    }

    Ok(())
}
