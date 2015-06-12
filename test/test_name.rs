use gimei::{Gender, Name};

#[test]
fn first_name() {
  {
    let name = Name::from_gender(Gender::Male);
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
  {
    let name = Name::from_gender(Gender::Female);
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
}

#[test]
fn last_name() {
  {
    let name = Name::from_gender(Gender::Male);
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
  {
    let name = Name::from_gender(Gender::Female);
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
}

#[test]
fn to_kanji() {
  let name = Name::from_gender(Gender::Male);
  let re = regex!(r"^\p{Han}+\s\p{Han}+$");
  assert!(re.is_match(&name.to_kanji()));
}

#[test]
fn to_hiragana() {
  let name = Name::from_gender(Gender::Male);
  let re = regex!(r"^\p{Hiragana}+\s\p{Hiragana}+$");
  assert!(re.is_match(&name.to_hiragana()));
}

#[test]
fn to_katakana() {
  let name = Name::from_gender(Gender::Male);
  let re = regex!(r"^\p{Katakana}+\s\p{Katakana}+$");
  assert!(re.is_match(&name.to_katakana()));
}

#[test]
fn is_male() {
  {
    let name = Name::from_gender(Gender::Male);
    assert!(name.is_male());
  }
  {
    let name = Name::from_gender(Gender::Female);
    assert!(!name.is_male());
  }
}

#[test]
fn is_female() {
  {
    let name = Name::from_gender(Gender::Male);
    assert!(!name.is_female());
  }
  {
    let name = Name::from_gender(Gender::Female);
    assert!(name.is_female());
  }
}

#[test]
fn display() {
  let name = Name::from_gender(Gender::Male);
  let re = regex!(r"^\p{Han}+\s\p{Han}+$");
  assert!(re.is_match(&format!("{}", name)));
}
