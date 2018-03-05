//参考 http://www.pandanoir.info/entry/2015/12/26/190000
//xを境に
//xと同じ: upper_bound-lower_bound
//xより小さい: lower_bound
//xより大きい: len-upper_bound

//vecは昇順ソート済み
//以上
pub fn lower_bound<T: Ord>(vec: &Vec<T>, mut first: usize, mut last: usize, val: &T) -> usize {
  let mut mid;
  while last - first > 1 {
    mid = (first + last) / 2;
    if &vec[mid] < val {
      first = mid;
    } else {
      last = mid;
    }
  }
  if &vec[first] < val {
    last
  } else {
    first
  }
}

//より大きい
pub fn upper_bound<T: Ord>(vec: &Vec<T>, mut first: usize, mut last: usize, val: &T) -> usize {
  let mut mid;
  while last - first > 1 {
    mid = (first + last) / 2;
    if &vec[mid] <= val {
      first = mid;
    } else {
      last = mid;
    }
  }
  if &vec[first] <= val {
    last
  } else {
    first
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let vec = vec![-1, 0, 0, 0, 1, 2, 2, 5, 5, 9];
    let len = vec.len();

    assert_eq!(0, lower_bound(&vec, 0, len, &-2));
    assert_eq!(0, upper_bound(&vec, 0, len, &-2));

    assert_eq!(0, lower_bound(&vec, 0, len, &-1));
    assert_eq!(1, upper_bound(&vec, 0, len, &-1));

    assert_eq!(1, lower_bound(&vec, 0, len, &0));
    assert_eq!(4, upper_bound(&vec, 0, len, &0));

    assert_eq!(9, lower_bound(&vec, 0, len, &9));
    assert_eq!(10, upper_bound(&vec, 0, len, &9));

    assert_eq!(10, lower_bound(&vec, 0, len, &10));
    assert_eq!(10, upper_bound(&vec, 0, len, &10));
  }
}