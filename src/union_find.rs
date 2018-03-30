#[derive(PartialEq, Debug, Clone)]
pub struct UnionFindNode {
  pub par: usize,
  pub rank: usize,
}

#[derive(PartialEq, Debug, Clone)]
pub struct UnionFind(pub Vec<UnionFindNode>);

impl UnionFind {
  pub fn new(size: usize) -> UnionFind {
    let mut vec = Vec::new();
    for i in 0..size {
      vec.push(UnionFindNode { par: i, rank: 0 });
    }

    UnionFind(vec)
  }
}
