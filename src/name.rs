use std::fmt::{Display, Formatter, Result};
use std::fs::File;
use std::io::prelude::*;
use rand::{Rng, thread_rng};
use yaml_rust::yaml::{Yaml, YamlLoader};

use super::Item;

lazy_static! {
  static ref NAMES: Vec<Yaml> = {
    let mut file = File::open("src/data/names.yml").unwrap();
    let mut buffer = String::new();
    let _ = file.read_to_string(&mut buffer);

    YamlLoader::load_from_str(&buffer).unwrap()
  };
}

#[derive(Debug, PartialEq)]
pub enum Gender { Male, Female }

impl Display for Gender {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match *self {
      Gender::Male => write!(f, "男"),
      Gender::Female => write!(f, "女"),
    }
  }
}

#[derive(Debug)]
pub struct Name {
  pub first: Item,
  pub last: Item,
  pub gender: Gender,
}

impl Name {
  pub fn new() -> Name {
    let mut r = thread_rng();

    if r.gen::<bool>() { Name::new_male() } else { Name::new_female() }
  }

  pub fn new_male() -> Name {
    let mut r = thread_rng();

    Name {
      first:
        Item::from_yaml(
          r.choose(NAMES[0]["first_name"]["male"].as_vec().unwrap()).unwrap()),
      last:
        Item::from_yaml(
          r.choose(NAMES[0]["last_name"].as_vec().unwrap()).unwrap()),
      gender:
        Gender::Male,
    }
  }

  pub fn new_female() -> Name {
    let mut r = thread_rng();

    Name {
      first:
        Item::from_yaml(
          r.choose(NAMES[0]["first_name"]["female"].as_vec().unwrap()).unwrap()),
      last:
        Item::from_yaml(
          r.choose(NAMES[0]["last_name"].as_vec().unwrap()).unwrap()),
      gender:
        Gender::Female,
    }
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

impl Display for Name {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.to_kanji())
  }
}
