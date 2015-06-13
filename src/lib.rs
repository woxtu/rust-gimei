#![crate_name = "gimei"]
#![crate_type = "lib"]

#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate yaml_rust;

pub mod address;
pub mod item;
pub mod name;

pub use address::Address;
pub use item::Item;
pub use name::{Gender, Name};
