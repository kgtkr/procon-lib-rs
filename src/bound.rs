//参考 http://www.pandanoir.info/entry/2015/12/26/190000

//(小さい,同じ,大きい)
pub fn bound_count<T: Ord>(vec: &Vec<T>, val: &T) -> (usize, usize, usize) {
  let lb = lower_bound(vec, 0, vec.len(), val);
  let ub = upper_bound(vec, 0, vec.len(), val);
  (lb, ub - lb, vec.len() - ub)
}

//より小さい
pub fn less_count<T: Ord>(vec: &Vec<T>, val: &T) -> usize {
  lower_bound(vec, 0, vec.len(), val)
}

//より大きい
pub fn larger_count<T: Ord>(vec: &Vec<T>, val: &T) -> usize {
  vec.len() - upper_bound(vec, 0, vec.len(), val)
}

//同じ
pub fn eq_count<T: Ord>(vec: &Vec<T>, val: &T) -> usize {
  upper_bound(vec, 0, vec.len(), val) - lower_bound(vec, 0, vec.len(), val)
}

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
    assert_eq!(0, less_count(&vec, &-2));
    assert_eq!(0, eq_count(&vec, &-2));
    assert_eq!(10, larger_count(&vec, &-2));
    assert_eq!((0, 0, 10), bound_count(&vec, &-2));

    assert_eq!(0, lower_bound(&vec, 0, len, &-1));
    assert_eq!(1, upper_bound(&vec, 0, len, &-1));
    assert_eq!(0, less_count(&vec, &-1));
    assert_eq!(1, eq_count(&vec, &-1));
    assert_eq!(9, larger_count(&vec, &-1));
    assert_eq!((0, 1, 9), bound_count(&vec, &-1));

    assert_eq!(1, lower_bound(&vec, 0, len, &0));
    assert_eq!(4, upper_bound(&vec, 0, len, &0));
    assert_eq!(1, less_count(&vec, &0));
    assert_eq!(3, eq_count(&vec, &0));
    assert_eq!(6, larger_count(&vec, &0));
    assert_eq!((1, 3, 6), bound_count(&vec, &0));

    assert_eq!(9, lower_bound(&vec, 0, len, &9));
    assert_eq!(10, upper_bound(&vec, 0, len, &9));
    assert_eq!(9, less_count(&vec, &9));
    assert_eq!(1, eq_count(&vec, &9));
    assert_eq!(0, larger_count(&vec, &9));
    assert_eq!((9, 1, 0), bound_count(&vec, &9));

    assert_eq!(10, lower_bound(&vec, 0, len, &10));
    assert_eq!(10, upper_bound(&vec, 0, len, &10));
    assert_eq!(10, less_count(&vec, &10));
    assert_eq!(0, eq_count(&vec, &10));
    assert_eq!(0, larger_count(&vec, &10));
    assert_eq!((10, 0, 0), bound_count(&vec, &10));
  }
}
