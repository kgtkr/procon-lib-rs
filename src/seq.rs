//char2id
//id2char

pub fn diff_seq(v: Vec<i64>) -> Vec<i64> {
  v.clone()
    .into_iter()
    .skip(1)
    .zip(v.into_iter())
    .map(|(a, b)| a - b)
    .collect()
}

pub fn one_base_to_zero_base(v: Vec<usize>) -> Vec<usize> {
  v.into_iter().map(|x| x - 1).collect()
}

pub fn sum_seq(v: Vec<i64>) -> Vec<i64> {
  v.into_iter()
    .scan(0, |state, x| {
      *state = *state + x;
      Some(*state)
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn diff_seq_test() {
    assert_eq!(vec![-9, 1, 0, 1], diff_seq(vec![10, 1, 2, 2, 3]));
    assert_eq!(vec![] as Vec<i64>, diff_seq(vec![10]));
  }

  #[test]
  fn one_base_to_zero_base_test() {
    assert_eq!(
      vec![0, 1, 3, 1, 2],
      one_base_to_zero_base(vec![1, 2, 4, 2, 3])
    );
    assert_eq!(vec![] as Vec<usize>, one_base_to_zero_base(vec![]));
  }

  #[test]
  fn sum_seq_test() {
    assert_eq!(vec![1, 1, 3, 7, 9, 12], sum_seq(vec![1, 0, 2, 4, 2, 3]));
    assert_eq!(vec![] as Vec<i64>, sum_seq(vec![]));
  }
}
