use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::{convert, fmt};

use crate::Item;

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

#[derive(Debug)]
struct MaleName {
  first: Item,
  last: Item,
}

impl rand::distributions::Distribution<MaleName> for rand::distributions::Standard {
  fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> MaleName {
    MaleName {
      first: DATA.first_name.male.choose(rng).cloned().unwrap(),
      last: DATA.last_name.choose(rng).cloned().unwrap(),
    }
  }
}

#[derive(Debug)]
struct FemaleName {
  first: Item,
  last: Item,
}

impl rand::distributions::Distribution<FemaleName> for rand::distributions::Standard {
  fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> FemaleName {
    FemaleName {
      first: DATA.first_name.female.choose(rng).cloned().unwrap(),
      last: DATA.last_name.choose(rng).cloned().unwrap(),
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

impl convert::From<MaleName> for Name {
  fn from(name: MaleName) -> Self {
    Self {
      first: name.first,
      last: name.last,
      gender: Gender::Male,
    }
  }
}

impl convert::From<FemaleName> for Name {
  fn from(name: FemaleName) -> Self {
    Self {
      first: name.first,
      last: name.last,
      gender: Gender::Female,
    }
  }
}

impl fmt::Display for Name {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_kanji())
  }
}

pub fn male() -> Name {
  rand::random::<MaleName>().into()
}

pub fn female() -> Name {
  rand::random::<FemaleName>().into()
}

pub fn name() -> Name {
  if rand::random() {
    male()
  } else {
    female()
  }
}
