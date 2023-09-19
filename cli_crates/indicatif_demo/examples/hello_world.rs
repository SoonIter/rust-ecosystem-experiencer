use std::{thread, time::Duration};

use indicatif::ProgressBar;

fn main() {
  let deps = 10;
  let pb = ProgressBar::new(deps);

  for _ in 0..deps {
    thread::sleep(Duration::from_millis(100));
    pb.inc(1);
  }

  pb.finish_and_clear();
}
