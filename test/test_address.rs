use gimei::Address;

#[test]
fn prefecture() {
  {
    let address = Address::new();
    {
      let re = regex!(r"^\p{Han}+$");
      assert!(re.is_match(&address.prefecture.kanji));
    }
    {
      let re = regex!(r"^\p{Hiragana}+$");
      assert!(re.is_match(&address.prefecture.hiragana));
    }
    {
      let re = regex!(r"^\p{Katakana}+$");
      assert!(re.is_match(&address.prefecture.katakana));
    }
  }
}

#[test]
fn city() {
  {
    let address = Address::new();
    {
      let re = regex!(r"^[\p{Han}\p{Hiragana}\p{Katakana}]+$");
      assert!(re.is_match(&address.city.kanji));
    }
    {
      let re = regex!(r"^\p{Hiragana}+$");
      assert!(re.is_match(&address.city.hiragana));
    }
    {
      let re = regex!(r"^\p{Katakana}+$");
      assert!(re.is_match(&address.city.katakana));
    }
  }
}

#[test]
fn town() {
  {
    let address = Address::new();
    {
      let re = regex!(r"^[\p{Han}\p{Hiragana}\p{Katakana}]+$");
      assert!(re.is_match(&address.town.kanji));
    }
    {
      let re = regex!(r"^\p{Hiragana}+$");
      assert!(re.is_match(&address.town.hiragana));
    }
    {
      let re = regex!(r"^\p{Katakana}+$");
      assert!(re.is_match(&address.town.katakana));
    }
  }
}

#[test]
fn to_kanji() {
  let address = Address::new();
  let re = regex!(r"^[\p{Han}\p{Hiragana}\p{Katakana}]+$");
  assert!(re.is_match(&address.to_kanji()));
}

#[test]
fn to_hiragana() {
  let address = Address::new();
  let re = regex!(r"^\p{Hiragana}+$");
  assert!(re.is_match(&address.to_hiragana()));
}

#[test]
fn to_katakana() {
  let address = Address::new();
  let re = regex!(r"^\p{Katakana}+$");
  assert!(re.is_match(&address.to_katakana()));
}

#[test]
fn display() {
  let address = Address::new();
  let re = regex!(r"^[\p{Han}\p{Hiragana}\p{Katakana}]+$");
  println!("{}", address.to_kanji());
  println!("{}", address);
  assert!(re.is_match(&format!("{}", address)));
}
