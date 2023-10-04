use rayon::prelude::*;
use std::time::Instant;

fn count(i: f64, n: f64) -> f64 {
  (4.0 / (1.0 + ((i + 0.5) / n) * ((i + 0.5) / n))) / n
}

// Calculus get PI
fn main() {
  let start = Instant::now();

  let n = 100000000;
  let n_f64 = 100000000.0;

  let res: f64 = (0..n)
    .into_par_iter()
    .map(|i| count(i as f64, n_f64))
    .sum::<f64>();

  dbg!(res);
  let duration = start.elapsed();
  println!("Time elapsed in expensive_function() is: {:?}", duration);
  /*
   100000 1.1865ms
   100000000 312.462208ms
  */
}
