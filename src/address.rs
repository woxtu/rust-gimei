use lazy_static::lazy_static;
use rand::{seq::SliceRandom, thread_rng};
use serde::Deserialize;
use std::fmt;

use super::Item;

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
  pub fn new() -> Address {
    let mut r = thread_rng();

    let prefecture = DATA.addresses.prefecture.choose(&mut r).cloned().unwrap();
    let city = DATA.addresses.city.choose(&mut r).cloned().unwrap();
    let town = DATA.addresses.town.choose(&mut r).cloned().unwrap();

    Address { prefecture, city, town }
  }

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

impl fmt::Display for Address {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_kanji())
  }
}
