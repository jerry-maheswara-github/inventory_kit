//! # inventory_kit
//! 
//! **A powerful Rust toolkit for dynamic inventory management and integration.**
//!
//! `inventory_kit` is a flexible, composable inventory management framework for Rust. It supports availability tracking, time-slot-based reservations, and structured extensibility through traits and generic types.
//!
//! ## ✅ Features
//!
//! - 📦 Generic inventory system via `InventoryItem` trait
//! - ⏱️ Time-slot-based availability via `AvailabilitySlot`
//! - 🧠 Abstractions via `InventoryRepository` and `AtomicInventoryOps` traits
//! - 🗂️ In-memory implementation ready to use
//! - ⚠️ Typed error handling with `InventoryError`
//! 
//! ---
//!
//! ## 📦 Use Case
//!
//! Ideal for applications needing availability tracking or reservations, such as:
//!
//! - Booking systems (rooms, rentals, events)
//! - Inventory-based games
//! - Product stock availability with time windows
//! - Scheduling of shared resources
//!
//! ---
//! 
//! ## ✨ Quick Start
//!
//! ```rust
//! use inventory_kit::error::InventoryError;
//! use inventory_kit::in_memory::InMemoryInventoryRepository;
//! use inventory_kit::model::InventoryItem;
//! use inventory_kit::repository::InventoryRepository;
//! 
//! #[derive(Debug, Clone, PartialEq, Eq, Hash)]
//! struct Product(u32);
//! 
//! impl InventoryItem for Product {
//!     type Id = u32;
//!     fn id(&self) -> Self::Id {
//!         self.0
//!     }
//! }
//! 
//! fn main() -> Result<(), InventoryError> {
//!     let mut repo = InMemoryInventoryRepository::<Product, u32>::new();
//! 
//!     let start_time = 10_00;
//!     let end_time = 20_00;
//! 
//!     repo.insert_availability(1, start_time, end_time, 5);
//!     print!("repo.insert_availability 5 (available:5) \n{:#?}\n", repo);
//!     
//!     match repo.reserve(&1, start_time, end_time, 3) {
//!         Ok(_) => println!("Reservation successful!"),
//!         Err(e) => return Err(e),
//!     }
//!     print!("repo.reserve 3 (available: 5-3=2) \n{:#?}\n", repo);
//! 
//!     match repo.release(&1, start_time, end_time, 1) {
//!         Ok(_) => println!("Release successful!"),
//!         Err(e) => return Err(e),
//!     }
//!     print!("repo.release 1 (available: 2+1=3) \n{:#?}\n", repo);
//! 
//!     match repo.adjust(&1, start_time, end_time, 10) {
//!         Ok(_) => println!("Adjust successful!"),
//!         Err(e) => return Err(e),
//!     }
//!     print!("repo.adjust 10 (available: 10) \n{:#?}\n", repo);
//! 
//!     let slots = repo.get_availability(&1, 0, 20_00)?;
//!     print!("slots item_id 1 \n{:#?}\n", slots);
//!     for slot in slots {
//!         println!(
//!             "Item {} is available for {} units between hour {} and hour {}.",
//!             slot.item_id, slot.available, slot.start, slot.end
//!         );
//!     }
//! 
//!     Ok(())
//! }
//! ```
//!
//! ---
//! 
//! ## 📄 License
//!
//! Licensed under the [Apache-2.0 license](http://www.apache.org/licenses/LICENSE-2.0.txt)
//!
//! ---
//!
//! ## 👨 Author
//!
//! Jerry Maheswara <jerrymaheswara@gmail.com>
//!
//! ---
//!
//! ## ❤️ Built with Love in Rust
//!
//! This project is built with ❤️ using **Rust** — a systems programming language that is safe, fast, and concurrent.  
//! Rust is the perfect choice for building reliable and efficient applications.
//!
//! ---
//!
//! ## 🤝 Contributing
//!
//! Pull requests, issues, and feedback are welcome!  
//! If you find this crate useful, give it a ⭐ and share it with others in the Rustacean community.
//!
//! ---

/// Core types like `InventoryItem` and `AvailabilitySlot`
pub mod model;

/// Traits for defining storage/repository behavior
pub mod repository;

/// Strongly-typed errors (`InventoryError`)
pub mod error;

/// In-memory implementation of the inventory system
pub mod in_memory;

/// Utility module for helper functions and internal logic.
pub mod utils;