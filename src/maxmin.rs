use std::cmp::{max, min};

macro_rules! max {
  ($x:expr) => {
    $x
  };
  ($x:expr, $($xs:tt)+) => {
    max($x,max!($($xs)+))
  };
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn max_test() {
    assert_eq!(10, max!(10));
    assert_eq!(10, max!(1, 2, 10));
    assert_eq!(10, max!(2, 10, 1));
    assert_eq!(10, max!(10, 2, 1));
  }
}
