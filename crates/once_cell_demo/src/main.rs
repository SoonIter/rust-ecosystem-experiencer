use once_cell::sync::Lazy;
use regex::Regex;

static IMPORTS_FROM_BLOCK_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r#"import([\w*{}\n\r\t, ]+from)?\s*([\w\d"'\.\/]*)"#)
    .expect("Init IMPORTS_FROM_BLOCK_REGEX failed")
});

fn main() {
  let test_str = "import x from 'pkg'; import b from 'other-pkg';";
  let res = IMPORTS_FROM_BLOCK_REGEX.find(test_str); // return the first matching result
  println!("{:?}", res.unwrap());
}
