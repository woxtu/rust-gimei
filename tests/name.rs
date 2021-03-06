use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct FirstName {
  male: Vec<Vec<String>>,
  female: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Data {
  first_name: FirstName,
  last_name: Vec<Vec<String>>,
}

lazy_static! {
  static ref DATA: Data = serde_yaml::from_str(&fs::read_to_string("src/data/names.yml").unwrap()).unwrap();
}

#[test]
fn male_first_name() {
  let name = gimei::male();

  let items = &DATA.first_name.male;
  assert!(items.iter().any(|x| x[0] == name.first.kanji));
  assert!(items.iter().any(|x| x[1] == name.first.hiragana));
  assert!(items.iter().any(|x| x[2] == name.first.katakana));
}

#[test]
fn female_first_name() {
  let name = gimei::female();

  let items = &DATA.first_name.female;
  assert!(items.iter().any(|x| x[0] == name.first.kanji));
  assert!(items.iter().any(|x| x[1] == name.first.hiragana));
  assert!(items.iter().any(|x| x[2] == name.first.katakana));
}

#[test]
fn last_name() {
  let name = gimei::name();

  let items = &DATA.last_name;
  assert!(items.iter().any(|x| x[0] == name.last.kanji));
  assert!(items.iter().any(|x| x[1] == name.last.hiragana));
  assert!(items.iter().any(|x| x[2] == name.last.katakana));
}

#[test]
fn to_kanji() {
  let name = gimei::name();
  let re = Regex::new(r"^\p{Han}+\s\p{Han}+$").unwrap();
  assert!(re.is_match(&name.to_kanji()));
}

#[test]
fn to_hiragana() {
  let name = gimei::name();
  let re = Regex::new(r"^\p{Hiragana}+\s\p{Hiragana}+$").unwrap();
  assert!(re.is_match(&name.to_hiragana()));
}

#[test]
fn to_katakana() {
  let name = gimei::name();
  let re = Regex::new(r"^\p{Katakana}+\s\p{Katakana}+$").unwrap();
  assert!(re.is_match(&name.to_katakana()));
}

#[test]
fn is_male() {
  let male_name = gimei::male();
  assert!(male_name.is_male());

  let female_name = gimei::female();
  assert!(!female_name.is_male());
}

#[test]
fn is_female() {
  let male_name = gimei::male();
  assert!(!male_name.is_female());

  let female_name = gimei::female();
  assert!(female_name.is_female());
}

#[test]
fn display() {
  let name = gimei::name();
  assert_eq!(format!("{}", name), name.to_kanji());
}
