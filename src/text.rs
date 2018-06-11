pub fn char2id(c: char) -> usize {
  c as usize - 'a' as usize
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn char2id_test() {
    assert_eq!(0, char2id('a'));
    assert_eq!(1, char2id('b'));
  }
}
