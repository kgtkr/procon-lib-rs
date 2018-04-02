use graph;
use std::cmp::min;

pub fn warshall_floyd(
  graph::MatrixGraph(mut vec): graph::MatrixGraph,
) -> Vec<Vec<Option<graph::Cost>>> {
  for k in 0..vec.len() {
    for i in 0..vec.len() {
      for j in 0..vec.len() {
        let now = vec[i][j];
        let new = match (vec[i][k], vec[k][j]) {
          (Some(a), Some(b)) => Some(a + b),
          _ => None,
        };
        vec[i][j] = match (now, new) {
          (Some(a), Some(b)) => Some(min(a, b)),
          (None, Some(x)) => Some(x),
          (Some(x), None) => Some(x),
          (None, None) => None,
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
    let mut graph = graph::MatrixGraph::from(graph::ListGraph::from(vec![
      vec![(2, 10), (1, 1)],
      vec![(3, 2)],
      vec![(1, 1), (3, 3), (4, 1)],
      vec![(0, 7), (4, 2)],
      vec![],
    ]));

    assert_eq!(
      vec![
        vec![Some(10), Some(1), Some(10), Some(3), Some(5)],
        vec![Some(9), Some(10), Some(19), Some(2), Some(4)],
        vec![Some(10), Some(1), Some(20), Some(3), Some(1)],
        vec![Some(7), Some(8), Some(17), Some(10), Some(2)],
        vec![None, None, None, None, None],
      ],
      warshall_floyd(graph)
    );
  }
}
