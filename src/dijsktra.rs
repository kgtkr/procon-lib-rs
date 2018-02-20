#[derive(PartialEq, Debug, Clone)]
pub struct Edges {
  to: usize,
  cost: i32,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Node {
  edges: Vec<Edges>,
  done: bool,
  cost: Option<i32>,
}

//スタートのコストを0とすること
pub fn dijsktra(nodes: &mut Vec<Node>) {
  let done_node = nodes
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
    .map(|(i, _)| i);

  if let Some(done_node) = done_node {
    nodes[done_node].done = true;
    for edge in nodes[done_node].edges.clone() {
      let cost = nodes[done_node].cost.unwrap() + edge.cost;
      if nodes[edge.to]
        .cost
        .map(|to_cost| cost < to_cost)
        .unwrap_or(true)
      {
        nodes[edge.to].cost = Some(cost);
      }
    }

    dijsktra(nodes);
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
