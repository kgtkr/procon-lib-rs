#[derive(PartialEq, Debug, Clone)]
pub struct Edges {
  to: Node,
  cost: i32,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Node {
  edges: Vec<Edges>,
  done: bool,
  cost: Option<i32>,
}

//スタートのコストを0とすること
pub fn dijsktra(nodes: Vec<Node>) -> Vec<Node> {
  let done = nodes
    .clone()
    .into_iter()
    .enumerate()
    .filter(|&(_, ref x)| !x.done)
    .fold(None, |min, (i, x)| match min {
      Option::None => Some((i, x)),
      Option::Some((_, min)) => match (min.cost, x.cost) {
        (Some(min_cost), Some(x_cost)) => {
          if min_cost > x_cost {
            Some((i, x))
          } else {
            None
          }
        }
        _ => None,
      },
    })
    .map(|(i, done_node)| {
      (
        i,
        Node {
          done: true,
          edges: done_node
            .edges
            .clone()
            .into_iter()
            .map(|edge| {
              let cost = done_node.cost.unwrap() + edge.cost;
              if edge.to.cost.map(|to_cost| cost < to_cost).unwrap_or(true) {
                Edges {
                  to: Node {
                    cost: Some(cost),
                    ..edge.to
                  },
                  ..edge
                }
              } else {
                edge
              }
            })
            .collect::<Vec<_>>(),
          ..done_node
        },
      )
    });
  if let Some((i, done)) = done {
    dijsktra({
      let mut n = nodes;
      n[i] = done;
      n
    })
  } else {
    nodes
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let mut map = vec![
      vec![true, false, true, false, true],
      vec![true, false, true, true, true],
      vec![true, false, true, false, true],
      vec![true, true, true, false, true],
      vec![true, false, true, false, true],
    ];
    assert_eq!(
      Some(vec![
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (3, 1),
        (3, 2),
        (2, 2),
        (1, 2),
        (1, 3),
        (1, 4),
        (2, 4),
        (3, 4),
        (4, 4),
      ]),
      bfs(&map)
    );
  }
}
