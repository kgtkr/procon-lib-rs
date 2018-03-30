use graph;

pub fn warshall_floyd(graph::ListGraph(nodes): graph::ListGraph) -> Vec<Vec<Option<graph::Cost>>> {
  //初期化
  let len = nodes.len();
  let mut vec = {
    let mut vec = Vec::with_capacity(len);
    let mut vec2 = Vec::with_capacity(len);
    vec2.resize(len, None);
    vec.resize(len, vec2);
    for (node_id, node) in nodes.into_iter().enumerate() {
      vec[node_id][node_id] = Some(0);
      for (_, edge_to, edge_cost) in node {
        vec[node_id][edge_to] = Some(edge_cost);
      }
    }
    vec
  };

  for k in 0..len {
    for i in 0..len {
      for j in 0..len {
        let now = vec[i][j];
        let new = match (vec[i][k], vec[k][j]) {
          (Some(a), Some(b)) => Some(a + b),
          _ => None,
        };
        vec[i][j] = match (now, new) {
          (Some(now), Some(new)) if now > new => Some(new),
          (None, Some(_)) => new,
          _ => now,
        };
      }
    }
  }

  vec
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let mut graph = graph::MinMemListGraph(vec![
      vec![(2, 10), (1, 1)],
      vec![(3, 2)],
      vec![(1, 1), (3, 3), (4, 1)],
      vec![(0, 7), (4, 2)],
      vec![],
    ]);
    let min = warshall_floyd(graph.into());

    assert_eq!(
      min,
      vec![
        vec![Some(0), Some(1), Some(10), Some(3), Some(5)],
        vec![Some(9), Some(0), Some(19), Some(2), Some(4)],
        vec![Some(10), Some(1), Some(0), Some(3), Some(1)],
        vec![Some(7), Some(8), Some(17), Some(0), Some(2)],
        vec![None, None, None, None, Some(0)],
      ]
    );
  }
}
