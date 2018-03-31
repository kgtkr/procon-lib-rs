//max以下の素数列挙
pub fn divisor(max: usize) -> Vec<usize> {
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
  fn test1() {
    assert_eq!(vec![2, 3, 5, 7], divisor(10));
    assert_eq!(vec![2, 3, 5, 7, 11], divisor(11));
  }
}
