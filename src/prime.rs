//nの約数列挙
pub fn divisor(n: usize) -> Vec<usize> {
  let mut res = Vec::new();
  for i in (1..).take_while(|x| x * x <= n) {
    if n % i == 0 {
      res.push(i);
      if i != n / i {
        res.push(n / i);
      }
    }
  }
  res
}

//max以下の素数列挙
pub fn prime(max: usize) -> Vec<usize> {
  let sqrt_max = (max as f64).sqrt();
  let mut prime_list = Vec::new();

  let mut search_list = (2..max + 1).collect::<Vec<_>>();

  while {
    let prime = search_list[0];
    prime_list.push(prime);
    search_list = search_list.into_iter().filter(|n| n % prime != 0).collect();

    (prime as f64) < sqrt_max
  } {}

  prime_list.append(&mut search_list);

  prime_list
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_prime() {
    assert_eq!(vec![2, 3, 5, 7], prime(10));
    assert_eq!(vec![2, 3, 5, 7, 11], prime(11));
  }

  #[test]
  fn test_divisor() {
    assert_eq!(vec![1, 10, 2, 5], divisor(10));
    assert_eq!(
      vec![1, 150, 2, 75, 3, 50, 5, 30, 6, 25, 10, 15],
      divisor(150)
    );
    assert_eq!(vec![1, 9, 3], divisor(9));
  }
}
