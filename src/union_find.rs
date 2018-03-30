#[derive(PartialEq, Debug, Clone)]
pub struct UnionFindNode {
  pub par: usize,
  pub rank: usize,
}

#[derive(PartialEq, Debug, Clone)]
pub struct UnionFind(pub Vec<UnionFindNode>);

impl UnionFind {
  //初期化
  pub fn new(size: usize) -> UnionFind {
    let mut vec = Vec::new();
    for i in 0..size {
      vec.push(UnionFindNode { par: i, rank: 0 });
    }

    UnionFind(vec)
  }

  //根を求める
  pub fn find(&mut self, x: usize) -> usize {
    if self.0[x].par == x {
      x
    } else {
      let par = self.0[x].par;
      let v = self.find(par);
      self.0[x].par = v;
      v
    }
  }

  //xとyの集合を併合
  pub fn unite(&mut self, x: usize, y: usize) {
    let x = self.find(x);
    let y = self.find(y);
    if x == y {
      return;
    }

    if self.0[x].rank < self.0[y].rank {
      self.0[x].par = y;
    } else {
      self.0[y].par = x;
      if self.0[x].rank == self.0[y].rank {
        self.0[x].rank += 1;
      }
    }
  }

  //xとyが同じ集合に属するか
  pub fn same(&mut self, x: usize, y: usize) -> bool {
    self.find(x) == self.find(y)
  }
}
