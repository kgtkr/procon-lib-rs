//階差数列
//-1
//累積和

pub fn diff_seq(v: Vec<i64>) -> Vec<i64> {
  v.clone()
    .into_iter()
    .skip(1)
    .zip(v.into_iter())
    .map(|(a, b)| a - b)
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn diff_seq_test() {
    assert_eq!(vec![-9, 1, 0, 1], diff_seq(vec![10, 1, 2, 2, 3]));
  }
}
