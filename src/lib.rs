#![crate_name = "gimei"]
#![crate_type = "lib"]

#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate yaml_rust;

use yaml_rust::yaml::Yaml;

#[derive(Debug)]
pub struct Item {
  pub kanji: String,
  pub hiragana: String,
  pub katakana: String,
}

impl Item {
  fn from_yaml(x: &Yaml) -> Item {
    Item {
      kanji: x[0].as_str().unwrap_or("").to_string(),
      hiragana: x[1].as_str().unwrap_or("").to_string(),
      katakana: x[2].as_str().unwrap_or("").to_string(),
    }
  }
}

pub use name::{Gender, Name};

pub mod name;
