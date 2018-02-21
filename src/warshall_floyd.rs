use graph;

pub fn warshall_floyd(nodes: graph::NodeArena) -> Vec<Vec<Option<i32>>> {
  //初期化
  let len = nodes.arena.len();
  let mut vec = {
    let mut vec = Vec::with_capacity(len);
    let mut vec2 = Vec::with_capacity(len);
    vec2.resize(len, None);
    vec.resize(len, vec2);
    for node in nodes.arena {
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
    let mut arena = graph::NodeArena::new();
    let node0 = arena.alloc();
    let node1 = arena.alloc();
    let node2 = arena.alloc();
    let node3 = arena.alloc();
    let node4 = arena.alloc();

    arena.add_edge(node0, node2, 10);
    arena.add_edge(node0, node1, 1);

    arena.add_edge(node1, node3, 2);

    arena.add_edge(node2, node1, 1);
    arena.add_edge(node2, node3, 3);
    arena.add_edge(node2, node4, 1);

    arena.add_edge(node3, node0, 7);
    arena.add_edge(node3, node4, 2);

    let min = warshall_floyd(arena);

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
