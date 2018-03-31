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
pub fn prime_sieve(n: usize) -> (Vec<usize>, Vec<bool>) {
  let mut prime = Vec::new();
  let mut is_prime = Vec::with_capacity(n + 1);
  is_prime.resize(n + 1, true);
  is_prime[0] = false;
  is_prime[1] = false;
  for i in 2..n + 1 {
    if is_prime[i] {
      prime.push(i);
      {
        let mut j = 2 * i;
        while j <= n {
          is_prime[j] = false;
          j += i;
        }
      }
    }
  }

  (prime, is_prime)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_prime() {
    assert_eq!(
      (
        vec![2, 3, 5, 7],
        vec![
          false, false, true, true, false, true, false, true, false, false, false
        ]
      ),
      prime_sieve(10)
    );
    assert_eq!(
      (
        vec![2, 3, 5, 7, 11],
        vec![
          false, false, true, true, false, true, false, true, false, false, false, true
        ]
      ),
      prime_sieve(11)
    );
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
