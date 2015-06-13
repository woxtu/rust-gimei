use std::{fmt, fs};
use std::io::prelude::*;
use rand::{Rng, thread_rng};
use yaml_rust::yaml::{Yaml, YamlLoader};

use super::Item;

lazy_static! {
  static ref ADDRESSES: Vec<Yaml> = {
    let mut file = fs::File::open("src/data/addresses.yml").unwrap();
    let mut buffer = String::new();
    let _ = file.read_to_string(&mut buffer);

    YamlLoader::load_from_str(&buffer).unwrap()
  };
}

#[derive(Debug)]
pub struct Address {
  pub prefecture: Item,
  pub city: Item,
  pub town: Item,
}

impl Address {
  pub fn new() -> Address {
    let mut r = thread_rng();

    let prefecture =
      r.choose(ADDRESSES[0]["addresses"]["prefecture"].as_vec().unwrap()).unwrap();
    let city = r.choose(ADDRESSES[0]["addresses"]["city"].as_vec().unwrap()).unwrap();
    let town = r.choose(ADDRESSES[0]["addresses"]["town"].as_vec().unwrap()).unwrap();

    Address {
      prefecture:
        Item::new(
          prefecture[0].as_str().unwrap_or(""),
          prefecture[1].as_str().unwrap_or(""),
          prefecture[2].as_str().unwrap_or("")),
      city:
        Item::new(
          city[0].as_str().unwrap_or(""),
          city[1].as_str().unwrap_or(""),
          city[2].as_str().unwrap_or("")),
      town:
        Item::new(
          town[0].as_str().unwrap_or(""),
          town[1].as_str().unwrap_or(""),
          town[2].as_str().unwrap_or("")),
    }
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
