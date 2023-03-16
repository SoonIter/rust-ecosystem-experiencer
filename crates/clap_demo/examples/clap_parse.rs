use clap::{Parser, builder::Str};

// cargo run --example clap_parse -- --type=hello --changed --summary="1212"

enum Type {
    Major,
    Minor,
    Patch
}


/// 运行参数
#[derive(Parser, Debug)]
#[clap(name = "Anti Facist Indoctrination")]
#[clap(
  about = "--------------------\n   clap-rs\n--------------------\n\nauthor: foo\nclap_parse"
)]
#[clap(version, author = "野兽先辈")]
pub struct Args {
  #[clap(long="type", short, value_name="type_name")]
  pub ty: Option<String>,

  #[clap(long, short)]
  pub preview: bool,

  #[clap(long, short)]
  pub changed: bool,

  #[clap(long, short)]
  pub summary: Option<String>,

  pub cookie: Vec<String>,
}

pub fn get_args() -> Args {
  Args::parse()
}

fn main() {
  let b = get_args();
  println!("{:?}", b);
}
