use graph;

pub fn warshall_floyd(nodes: graph::Graph) -> Vec<Vec<Option<i32>>> {
  //初期化
  let len = nodes.len();
  let mut vec = {
    let mut vec = Vec::with_capacity(len);
    let mut vec2 = Vec::with_capacity(len);
    vec2.resize(len, None);
    vec.resize(len, vec2);
    for node in nodes {
      vec[node.id][node.id] = Some(0);
      for edge in node.edges {
        vec[node.id][edge.to] = Some(edge.cost);
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
    let mut graph = vec![
      graph::Node {
        id: 0,
        edges: vec![
          graph::Edge { to: 2, cost: 10 },
          graph::Edge { to: 1, cost: 1 },
        ],
      },
      graph::Node {
        id: 1,
        edges: vec![graph::Edge { to: 3, cost: 2 }],
      },
      graph::Node {
        id: 2,
        edges: vec![
          graph::Edge { to: 1, cost: 1 },
          graph::Edge { to: 3, cost: 3 },
          graph::Edge { to: 4, cost: 1 },
        ],
      },
      graph::Node {
        id: 3,
        edges: vec![
          graph::Edge { to: 0, cost: 7 },
          graph::Edge { to: 4, cost: 2 },
        ],
      },
      graph::Node {
        id: 4,
        edges: vec![],
      },
    ];
    let min = warshall_floyd(graph);

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
