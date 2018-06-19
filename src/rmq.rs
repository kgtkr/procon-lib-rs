//蟻本
use std::cmp::min;

struct RMQ(pub Vec<i64>);

impl RMQ {
  pub fn new(n: usize) -> RMQ {
    let mut n2 = 1;
    while n2 < n {
      n2 *= 2;
    }
    let size = n2 * 2 - 1;
    let mut v = Vec::with_capacity(size);
    v.resize(size, <i64>::max_value());
    RMQ(v)
  }

  pub fn len(&self) -> usize {
    (self.0.len() + 1) / 2
  }

  pub fn update(&mut self, mut k: usize, a: i64) {
    k += self.len() - 1;
    self.0[k] = a;
    while k > 0 {
      k = (k - 1) / 2;
      self.0[k] = min(self.0[k * 2 + 1], self.0[k * 2 + 2]);
    }
  }

  fn query_f(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
    if r <= a || b <= l {
      <i64>::max_value()
    } else if a <= l && r <= b {
      self.0[k]
    } else {
      min(
        self.query_f(a, b, k * 2 + 1, l, (l + r) / 2),
        self.query_f(a, b, k * 2 + 2, (l + r) / 2, r),
      )
    }
  }

  pub fn query(&self, a: usize, b: usize) -> i64 {
    self.query_f(a, b, 0, 0, self.len())
  }
}
