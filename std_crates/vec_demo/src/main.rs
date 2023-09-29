use std::iter::*;

fn main() {
  let numbers = vec![1, 3, 2, 3, 4, 5];

  // 使用 take_while 方法
  let take_while_result: Vec<i32> = numbers.iter().take_while(|n| **n < 3).copied().collect();
  println!("Take while result: {:?}", take_while_result);

  // 使用 filter 方法
  let filter_result: Vec<i32> = numbers.iter().filter(|n| **n < 3).copied().collect();
  println!("Filter result: {:?}", filter_result);
  println!("result: {:?}", numbers);
}
