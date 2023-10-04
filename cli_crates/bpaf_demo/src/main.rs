use bpaf::*;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options("hackerman"))]
pub enum Action {
  #[bpaf(command("explain"))]
  Explain {
    #[bpaf(positional("CRATE"))]
    krate: String,
    #[bpaf(external(feature_if))]
    feature: Option<String>,
    #[bpaf(external(version_if))]
    version: Option<String>,
  },
}

fn feature_if() -> impl Parser<Option<String>> {
  // here feature starts as any string on a command line that does not start with a dash
  positional::<String>("FEATURE")
    // guard restricts it such that it can't be a valid version
    .guard(move |s| !is_version(s), "")
    // last two steps describe what to do with strings in this position but are actually
    // versions.
    // optional allows parser to represent an ignored value with None
    .optional()
    // and catch lets optional to handle parse failures coming from guard
    .catch()
}

fn version_if() -> impl Parser<Option<String>> {
  positional::<String>("VERSION")
    .guard(move |s| is_version(s), "")
    .optional()
    .catch()
}

fn is_version(v: &str) -> bool {
  v.chars().all(|c| c.is_numeric())
}

fn main() {
  println!("{:?}", action().fallback_to_usage().run());
}
