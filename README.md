# rust-gimei

[![Crates.io](https://img.shields.io/crates/v/gimei.svg?style=flat-square)](https://crates.io/crates/gimei)

Rust implementation of [gimei](https://github.com/willnet/gimei); Generate fake name for Japanese.

## Installation

Adding the following to the Cargo.toml in your project:

```toml
[dependencies]
gimei = "0.2"
```

and import using `extern crate`:

```rust
extern crate gimei;
```

## Usage

Generate fake name.

```rust
let name = gimei::name();
println!("{}", name);                // 松原 春希

println!("{}", name.to_kanji());     // 松原 春希
println!("{}", name.to_hiragana());  // まつばら はるき
println!("{}", name.to_katakana());  // マツバラ ハルキ

println!("{}", name.last.kanji);     // 松原
println!("{}", name.last.hiragana);  // まつばら
println!("{}", name.last.katakana);  // マツバラ

println!("{}", name.first.kanji);    // 春希
println!("{}", name.first.hiragana); // はるき
println!("{}", name.first.katakana); // ハルキ
```

```rust
let name = gimei::male();
println!("{}", name.is_male());   // true
println!("{}", name.is_female()); // false
println!("{}", name.to_kanji());  // 高山 伴幸
```

```rust
let name = gimei::female();
println!("{}", name.is_male());   // false
println!("{}", name.is_female()); // true
println!("{}", name.to_kanji());  // 清水 心鈴
```

Generate fake address.

```rust
let address = gimei::address();
println!("{}", address);                     // 山梨県余市郡余市町東原町

println!("{}", address.to_kanji());          // 山梨県余市郡余市町東原町
println!("{}", address.to_hiragana());       // やまなしけんよいちぐんよいちちょうひがしはらまち
println!("{}", address.to_katakana());       // ヤマナシケンヨイチグンヨイチチョウヒガシハラマチ

println!("{}", address.prefecture.kanji);    // 山梨県
println!("{}", address.prefecture.hiragana); // やまなしけん
println!("{}", address.prefecture.katakana); // ヤマナシケン

println!("{}", address.city.kanji);          // 余市郡余市町
println!("{}", address.city.hiragana);       // よいちぐんよいちちょう
println!("{}", address.city.katakana);       // ヨイチグンヨイチチョウ

println!("{}", address.town.kanji);          // 東原町
println!("{}", address.town.hiragana);       // ひがしはらまち
println!("{}", address.town.katakana);       // ヒガシハラマチ
```

## License
Copyright (c) 2015 woxtu

Licensed under the MIT license.
