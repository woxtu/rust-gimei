use gimei::{Gender, Name};

#[test]
fn first_name() {
  let name = Name::new();
  {
    let re = regex!(r"^\p{Han}+$");
    assert!(re.is_match(&name.first.kanji));
  }
  {
    let re = regex!(r"^\p{Hiragana}+$");
    assert!(re.is_match(&name.first.hiragana));
  }
  {
    let re = regex!(r"^\p{Katakana}+$");
    assert!(re.is_match(&name.first.katakana));
  }
}

#[test]
fn last_name() {
  let name = Name::new();
  {
    let re = regex!(r"^\p{Han}+$");
    assert!(re.is_match(&name.last.kanji));
  }
  {
    let re = regex!(r"^\p{Hiragana}+$");
    assert!(re.is_match(&name.last.hiragana));
  }
  {
    let re = regex!(r"^\p{Katakana}+$");
    assert!(re.is_match(&name.last.katakana));
  }
}

#[test]
fn new_male() {
  let name = Name::new_male();
  assert_eq!(name.gender, Gender::Male);
}

#[test]
fn new_female() {
  let name = Name::new_female();
  assert_eq!(name.gender, Gender::Female);
}

#[test]
fn to_kanji() {
  let name = Name::new();
  let re = regex!(r"^\p{Han}+\s\p{Han}+$");
  assert!(re.is_match(&name.to_kanji()));
}

#[test]
fn to_hiragana() {
  let name = Name::new();
  let re = regex!(r"^\p{Hiragana}+\s\p{Hiragana}+$");
  assert!(re.is_match(&name.to_hiragana()));
}

#[test]
fn to_katakana() {
  let name = Name::new();
  let re = regex!(r"^\p{Katakana}+\s\p{Katakana}+$");
  assert!(re.is_match(&name.to_katakana()));
}

#[test]
fn is_male() {
  let name = Name::new_male();
  assert!(name.is_male());
  assert!(!name.is_female());
}

#[test]
fn is_female() {
  let name = Name::new_female();
  assert!(!name.is_male());
  assert!(name.is_female());
}

#[test]
fn display() {
  let name = Name::new();
  let re = regex!(r"^\p{Han}+\s\p{Han}+$");
  assert!(re.is_match(&format!("{}", name)));
}
