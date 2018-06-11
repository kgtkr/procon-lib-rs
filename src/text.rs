pub fn char2id(c: char) -> usize {
  c as usize - 'a' as usize
}

pub fn id2char(id: usize) -> char {
  (id + 'a' as usize) as u8 as char
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn char2id_test() {
    assert_eq!(0, char2id('a'));
    assert_eq!(1, char2id('b'));
  }

  #[test]
  fn id2char_test() {
    assert_eq!('a', id2char(0));
    assert_eq!('b', id2char(1));
  }
}
