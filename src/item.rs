use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Item {
  pub kanji: String,
  pub hiragana: String,
  pub katakana: String,
}
