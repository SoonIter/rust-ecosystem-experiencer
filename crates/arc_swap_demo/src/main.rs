use std::sync::Arc;

use arc_swap::ArcSwap;
use crossbeam_utils::thread;
use std::thread::sleep;

fn sleep_two_secs() {
  sleep(std::time::Duration::from_secs(2));
}

fn main() {
  let config = ArcSwap::from(Arc::new(String::default()));
  thread::scope(|scope| {
    scope.spawn(|_| {
      sleep_two_secs();
      let new_conf = Arc::new("New configuration".to_owned());
      config.store(new_conf);
    });
    // for _ in 0..10 {
    scope.spawn(|_| loop {
      println!("loading config");
      let cfg = config.load();
      if !cfg.is_empty() {
        assert_eq!(**cfg, "New configuration");
        println!("New Configuration ---- {}", **cfg);
        return;
      }
    });
    // }
  })
  .unwrap();
}
