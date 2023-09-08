/*
You can derive a `Diagnostic` from any `std::error::Error` type.

`thiserror` is a great way to define them, and plays nicely with `miette`!
*/
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("eslint(no-hello): say hello is a bad behavior")]
#[diagnostic(
  // severity(warning),
  // code("1:1 hello.js"),
  // url("https://google.com"),
  help("try to search it on the google.com")
)]
struct MyError(
  #[source_code] NamedSource,
  #[label("please do not say hello here")] SourceSpan, // (start, len)
);

/*
Now let's define a function!

Use this `Result` type (or its expanded version) as the return type
throughout your app (but NOT your libraries! Those should always return
concrete types!).
*/
use miette::{NamedSource, Result};

fn this_fails() -> Result<i32> {
  // You can use plain strings as a `Source`, or anything that implements
  // the one-method `Source` trait.
  let src =
    "  const b = 1;\n  console.log('yes')\n  console.log('hello')\n    let a = 1;".to_string();
  let target_word = "console.log('hello')";

  let start = src.find(target_word);
  match start {
    Some(pos) => Err(
      MyError(
        NamedSource::new("bad_file.rs", src),
        (pos, target_word.len()).into(),
      )
      .into(),
    ),
    None => Ok(1),
  }
}

/*
Now to get everything printed nicely, just return a `Result<()>`
and you're all set!

Note: You can swap out the default reporter for a custom one using
`miette::set_hook()`
*/
fn pretend_this_is_main() -> Result<()> {
  // kaboom~
  this_fails()?;

  Ok(())
}

fn main() {
  if let Err(x) = pretend_this_is_main() {
    println!("{:?}", x)
  }
}
