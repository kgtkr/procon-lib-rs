//最大公約数
pub fn gcd(a: i64, b: i64) -> i64 {
  if b == 0 {
    a
  } else {
    gcd(b, a % b)
  }
}

//最小公倍数
pub fn lcm(a: i64, b: i64) -> i64 {
  a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_gcd() {
    assert_eq!(5, gcd(5, 0));
    assert_eq!(11, gcd(0, 11));
    assert_eq!(7, gcd(182, 1029));
  }

  #[test]
  fn test_lcm() {
    assert_eq!(1873326, lcm(182, 10293));
    assert_eq!(66356, lcm(212, 313));
  }
}
