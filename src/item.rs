#[derive(Debug)]
pub struct Item {
  pub kanji: String,
  pub hiragana: String,
  pub katakana: String,
}

impl Item {
  pub fn new(kanji: &str, hiragana: &str, katakana: &str) -> Item {
    Item {
      kanji: kanji.to_string(),
      hiragana: hiragana.to_string(),
      katakana: katakana.to_string(),
    }
  }
}
