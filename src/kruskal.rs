use graph::FlatGraph;
use union_find::UnionFind;

fn kruskal(FlatGraph(node_len, mut edges): FlatGraph) -> FlatGraph {
  edges.sort_by(|&(_, _, ref a), &(_, _, ref b)| a.cmp(b));
  let mut uf = UnionFind::new(node_len);
  let mut res = FlatGraph(node_len, Vec::new());
  for (from, to, cost) in edges {
    if !uf.same(from, to) {
      uf.unite(from, to);
      res.1.push((from, to, cost));
    }
  }
  res
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    assert_eq!(
      FlatGraph(
        7,
        vec![
          (0, 2, 1),
          (3, 5, 1),
          (1, 2, 2),
          (2, 3, 3),
          (3, 6, 5),
          (4, 5, 5),
        ]
      ),
      kruskal(FlatGraph(
        7,
        vec![
          (0, 2, 1),
          (1, 2, 2),
          (1, 4, 10),
          (2, 3, 3),
          (2, 5, 7),
          (3, 5, 1),
          (3, 6, 5),
          (4, 5, 5),
          (5, 6, 8),
        ]
      ))
    );
  }
}
