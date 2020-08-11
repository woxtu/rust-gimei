pub mod address;
pub mod item;
pub mod name;

pub use address::Address;
pub use item::Item;
pub use name::{Gender, Name};

pub fn name() -> Name {
  if rand::random() {
    rand::random::<name::MaleName>().into()
  } else {
    rand::random::<name::FemaleName>().into()
  }
}

pub fn male() -> Name {
  rand::random::<name::MaleName>().into()
}

pub fn female() -> Name {
  rand::random::<name::FemaleName>().into()
}

pub fn address() -> Address {
  rand::random()
}
