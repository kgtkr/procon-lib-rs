use graph::FlatGraph;

fn kruskal(FlatGraph(len, mut edges): FlatGraph) {
  edges.sort_by_key(|&(_, _, x)| x);
}
