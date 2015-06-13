#![crate_name = "gimei"]
#![crate_type = "lib"]

#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate yaml_rust;

use rand::Rng;

pub mod address;
pub mod item;
pub mod name;

pub use address::Address;
pub use item::Item;
pub use name::{Gender, Name};

pub fn name() -> Name {
  if rand::thread_rng().gen() {
    Name::from_gender(Gender::Male)
  } else {
    Name::from_gender(Gender::Female)
  }
}

pub fn male() -> Name {
  Name::from_gender(Gender::Male)
}

pub fn female() -> Name {
  Name::from_gender(Gender::Female)
}

pub fn address() -> Address {
  Address::new()
}
