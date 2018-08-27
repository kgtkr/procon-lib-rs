use std::collections::HashMap;

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

pub fn sum_seq_2d(mut vec: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
  for i in 0..vec.len() {
    for j in 0..vec[0].len() {
      vec[i][j] += if j != 0 { vec[i][j - 1] } else { 0 };
    }
  }

  for i in 0..vec.len() {
    for j in 0..vec[0].len() {
      vec[i][j] += if i != 0 { vec[i - 1][j] } else { 0 };
    }
  }

  vec
}

pub fn sum_seq_2d_rect(
  sum_vec: &Vec<Vec<i64>>,
  (x1, y1): (usize, usize),
  (x2, y2): (usize, usize),
) -> i64 {
  sum_vec[x2][y2] + (if x1 != 0 && y1 != 0 {
    sum_vec[x1 - 1][y1 - 1]
  } else {
    0
  }) - (if x1 != 0 { sum_vec[x1 - 1][y2] } else { 0 }) - (if y1 != 0 {
    sum_vec[x2][y1 - 1]
  } else {
    0
  })
}

pub fn map_add(map: &mut HashMap<i64, i64>, key: i64, add: i64) {
  let v = match map.get(&key) {
    Some(v) => *v + add,
    None => add,
  };
  map.insert(key, v);
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

  #[test]
  fn sum_seq_2d_test() {
    assert_eq!(vec![] as Vec<Vec<i64>>, sum_seq_2d(vec![]));
    assert_eq!(
      vec![vec![1, 3], vec![1, 13], vec![3, 18], vec![13, 28]],
      sum_seq_2d(vec![vec![1, 2], vec![0, 10], vec![2, 3], vec![10, 0]])
    );
  }

  #[test]
  fn sum_seq_2d_rect_test() {
    let sum = sum_seq_2d(vec![vec![1, 2], vec![0, 10], vec![2, 3], vec![10, 0]]);
    assert_eq!(1, sum_seq_2d_rect(&sum, (0, 0), (0, 0)));
    assert_eq!(0, sum_seq_2d_rect(&sum, (1, 0), (1, 0)));
    assert_eq!(2, sum_seq_2d_rect(&sum, (0, 1), (0, 1)));
    assert_eq!(28, sum_seq_2d_rect(&sum, (0, 0), (3, 1)));
    assert_eq!(13, sum_seq_2d_rect(&sum, (1, 1), (3, 1)));
  }

  #[test]
  fn map_add_test() {
    let mut map = HashMap::new();
    map_add(&mut map, 0, 1);
    assert_eq!(
      {
        let mut v = HashMap::new();
        v.insert(0, 1);
        v
      },
      map
    );

    map_add(&mut map, 0, 1);
    assert_eq!(
      {
        let mut v = HashMap::new();
        v.insert(0, 2);
        v
      },
      map
    );

    map_add(&mut map, 10, 5);
    assert_eq!(
      {
        let mut v = HashMap::new();
        v.insert(0, 2);
        v.insert(10, 5);
        v
      },
      map
    );

    map_add(&mut map, 8, 0);
    assert_eq!(
      {
        let mut v = HashMap::new();
        v.insert(0, 2);
        v.insert(10, 5);
        v.insert(8, 0);
        v
      },
      map
    );
  }
}
