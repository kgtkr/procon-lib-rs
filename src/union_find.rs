#[derive(PartialEq, Debug, Clone)]
pub struct UnionFindNode {
  pub par: usize,
  pub rank: usize,
}

#[derive(PartialEq, Debug, Clone)]
pub struct UnionFind(pub Vec<UnionFindNode>);
