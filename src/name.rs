use lazy_static::lazy_static;
use rand::{seq::SliceRandom, thread_rng};
use serde::Deserialize;
use std::fmt;

use super::Item;

#[derive(Debug, Deserialize)]
struct FirstName {
  male: Vec<Item>,
  female: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct Data {
  first_name: FirstName,
  last_name: Vec<Item>,
}

lazy_static! {
  static ref DATA: Data = serde_yaml::from_str(include_str!("data/names.yml")).unwrap();
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Gender {
  Male,
  Female,
}

impl fmt::Display for Gender {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Gender::Male => write!(f, "男"),
      Gender::Female => write!(f, "女"),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Name {
  pub first: Item,
  pub last: Item,
  pub gender: Gender,
}

impl Name {
  pub fn from_gender(gender: Gender) -> Name {
    let mut rng = thread_rng();

    let first = match gender {
      Gender::Male => DATA.first_name.male.choose(&mut rng).cloned().unwrap(),
      Gender::Female => DATA.first_name.female.choose(&mut rng).cloned().unwrap(),
    };
    let last = DATA.last_name.choose(&mut rng).cloned().unwrap();

    Name { first, last, gender }
  }

  pub fn to_kanji(&self) -> String {
    format!("{} {}", self.last.kanji, self.first.kanji)
  }

  pub fn to_hiragana(&self) -> String {
    format!("{} {}", self.last.hiragana, self.first.hiragana)
  }

  pub fn to_katakana(&self) -> String {
    format!("{} {}", self.last.katakana, self.first.katakana)
  }

  pub fn is_male(&self) -> bool {
    self.gender == Gender::Male
  }

  pub fn is_female(&self) -> bool {
    self.gender == Gender::Female
  }
}

impl fmt::Display for Name {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_kanji())
  }
}
