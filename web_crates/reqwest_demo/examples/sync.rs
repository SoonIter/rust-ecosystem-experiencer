fn main() -> Result<(), Box<dyn std::error::Error>> {
  let url = "https://jsonplaceholder.typicode.com/posts";
  let resp = reqwest::blocking::get(url)?;
  dbg!(resp);
  Ok(())
}
