use std::cmp::{max, min};

macro_rules! max {
  ($x:expr) => {
    $x
  };
  ($x:expr, $($xs:tt)+) => {
    max($x,max!($($xs)+))
  };
}

macro_rules! min {
  ($x:expr) => {
    $x
  };
  ($x:expr, $($xs:tt)+) => {
    min($x,min!($($xs)+))
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
    assert_eq!(10, max!(10, 1, 2));
    assert_eq!(10, max!(2, 10, 1, 2, 5, 8, 7));
  }

  #[test]
  fn min_test() {
    assert_eq!(1, min!(1));
    assert_eq!(1, min!(1, 2, 10));
    assert_eq!(1, min!(2, 10, 1));
    assert_eq!(1, min!(10, 1, 2));
    assert_eq!(1, min!(2, 10, 1, 2, 5, 8, 7));
  }
}
