use graph::FlatGraph;
use union_find::UnionFind;

fn kruskal(FlatGraph(node_len, mut edges): FlatGraph) -> FlatGraph {
  edges.sort_by_key(|&(_, _, x)| x);
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
