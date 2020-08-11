use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Addresses {
  prefecture: Vec<Vec<String>>,
  city: Vec<Vec<String>>,
  town: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Data {
  addresses: Addresses,
}

lazy_static! {
  static ref DATA: Data = serde_yaml::from_str(&fs::read_to_string("src/data/addresses.yml").unwrap()).unwrap();
}

#[test]
fn prefecture() {
  let address = gimei::address();

  let items = &DATA.addresses.prefecture;
  assert!(items.iter().any(|x| x[0] == address.prefecture.kanji));
  assert!(items.iter().any(|x| x[1] == address.prefecture.hiragana));
  assert!(items.iter().any(|x| x[2] == address.prefecture.katakana));
}

#[test]
fn city() {
  let address = gimei::address();

  let items = &DATA.addresses.city;
  assert!(items.iter().any(|x| x[0] == address.city.kanji));
  assert!(items.iter().any(|x| x[1] == address.city.hiragana));
  assert!(items.iter().any(|x| x[2] == address.city.katakana));
}

#[test]
fn town() {
  let address = gimei::address();

  let items = &DATA.addresses.town;
  assert!(items.iter().any(|x| x[0] == address.town.kanji));
  assert!(items.iter().any(|x| x[1] == address.town.hiragana));
  assert!(items.iter().any(|x| x[2] == address.town.katakana));
}

#[test]
fn to_kanji() {
  let address = gimei::address();
  let re = Regex::new(r"^[\p{Han}\p{Hiragana}\p{Katakana}]+$").unwrap();
  assert!(re.is_match(&address.to_kanji()));
}

#[test]
fn to_hiragana() {
  let address = gimei::address();
  let re = Regex::new(r"^\p{Hiragana}+$").unwrap();
  assert!(re.is_match(&address.to_hiragana()));
}

#[test]
fn to_katakana() {
  let address = gimei::address();
  let re = Regex::new(r"^\p{Katakana}+$").unwrap();
  assert!(re.is_match(&address.to_katakana()));
}

#[test]
fn display() {
  let address = gimei::address();
  assert_eq!(format!("{}", address), address.to_kanji());
}
