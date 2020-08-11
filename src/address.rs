use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::fmt;

use crate::Item;

#[derive(Debug, Deserialize)]
struct Addresses {
  prefecture: Vec<Item>,
  city: Vec<Item>,
  town: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct Data {
  addresses: Addresses,
}

lazy_static! {
  static ref DATA: Data = serde_yaml::from_str(include_str!("data/addresses.yml")).unwrap();
}

#[derive(Clone, Debug)]
pub struct Address {
  pub prefecture: Item,
  pub city: Item,
  pub town: Item,
}

impl Address {
  pub fn to_kanji(&self) -> String {
    format!("{}{}{}", self.prefecture.kanji, self.city.kanji, self.town.kanji)
  }

  pub fn to_hiragana(&self) -> String {
    format!("{}{}{}", self.prefecture.hiragana, self.city.hiragana, self.town.hiragana)
  }

  pub fn to_katakana(&self) -> String {
    format!("{}{}{}", self.prefecture.katakana, self.city.katakana, self.town.katakana)
  }
}

impl rand::distributions::Distribution<Address> for rand::distributions::Standard {
  fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Address {
    Address {
      prefecture: DATA.addresses.prefecture.choose(rng).cloned().unwrap(),
      city: DATA.addresses.city.choose(rng).cloned().unwrap(),
      town: DATA.addresses.town.choose(rng).cloned().unwrap(),
    }
  }
}

impl fmt::Display for Address {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_kanji())
  }
}

pub fn address() -> Address {
  rand::random()
}
