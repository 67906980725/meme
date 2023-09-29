// pinyin = "0.9"

use pinyin::ToPinyin;
use regex::Regex;

pub fn str_to_pinyin_and_py(string: &str) -> (String, String) {
  let pinyin_arr = string.to_pinyin();
  let mut pinyin_str = String::new();
  let mut py_str = String::new();
  for one_pinyin in pinyin_arr {
    if let Some(one_pinyin) = one_pinyin {
      let one_pinyin_str = one_pinyin.plain();
      pinyin_str.push_str(one_pinyin_str);
      py_str.push(String::from(one_pinyin_str).chars().next().unwrap());
    }
  }
  return (pinyin_str, py_str);
}

pub fn str_to_pinyin(string: &str) -> String {
  let pinyin_arr = string.to_pinyin();
  let mut pinyin_str = String::new();
  for pinyin in pinyin_arr {
    if let Some(pinyin) = pinyin {
      pinyin_str.push_str(pinyin.plain());
    }
  }
  return pinyin_str;
}

pub fn str_to_py(string: &str) -> String {
  let pinyin_arr = string.to_pinyin();
  let mut pinyin_str = String::new();
  for pinyin in pinyin_arr {
    if let Some(pinyin) = pinyin {
      let mut tmp_str = String::new();
      tmp_str.push_str(pinyin.plain());
      pinyin_str.push(tmp_str.chars().next().unwrap());
    }
  }
  return pinyin_str;
}

pub fn contains_chinese(input: &str) -> bool{
  let re = Regex::new(r"[\p{Script=Han}|\p{Script=Bopomofo}|\p{Script=Hiragana}|\p{Script=Katakana}|\p{Script=Hangul}]+").unwrap();
  re.is_match(input)
}
