use std::{fmt, fs};
use std::io::prelude::*;
use rand::{Rng, thread_rng};
use yaml_rust::yaml::{Yaml, YamlLoader};

use super::Item;

lazy_static! {
  static ref NAMES: Vec<Yaml> = {
    let mut file = fs::File::open("src/data/names.yml").unwrap();
    let mut buffer = String::new();
    let _ = file.read_to_string(&mut buffer);

    YamlLoader::load_from_str(&buffer).unwrap()
  };
}

#[derive(Debug, PartialEq)]
pub enum Gender { Male, Female }

impl fmt::Display for Gender {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
  pub fn from_gender(gender: Gender) -> Name {
    let mut r = thread_rng();

    let first_name = match gender {
      Gender::Male =>
        r.choose(NAMES[0]["first_name"]["male"].as_vec().unwrap()).unwrap(),
      Gender::Female =>
        r.choose(NAMES[0]["first_name"]["female"].as_vec().unwrap()).unwrap(),
    };
    let last_name = r.choose(NAMES[0]["last_name"].as_vec().unwrap()).unwrap();

    Name {
      first:
        Item::new(
          first_name[0].as_str().unwrap_or(""),
          first_name[1].as_str().unwrap_or(""),
          first_name[2].as_str().unwrap_or("")),
      last:
        Item::new(
          last_name[0].as_str().unwrap_or(""),
          last_name[1].as_str().unwrap_or(""),
          last_name[2].as_str().unwrap_or("")),
      gender: gender,
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

impl fmt::Display for Name {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_kanji())
  }
}
