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

//拡張ユークリッド(参照版)
pub fn extgcd_ref(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
  let mut d = a;
  if b != 0 {
    d = extgcd_ref(b, a % b, y, x);
    *y -= (a / b) * *x;
  } else {
    *x = 1;
    *y = 0;
  }
  d
}

//拡張ユークリッド
//(gcd(a,b),x,y)
pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
  let mut x = 0;
  let mut y = 0;
  let gcd = extgcd_ref(a, b, &mut x, &mut y);
  (gcd, x, y)
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

  #[test]
  fn test_extgcd() {
    assert_eq!((1, -7, 12), extgcd(29, 17));
    assert_eq!((12, -7, 18), extgcd(924, 360));
  }
}
