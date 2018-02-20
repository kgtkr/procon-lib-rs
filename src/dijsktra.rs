use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Debug, Clone)]
pub struct Edge {
  pub to: usize,
  pub cost: i32,
}

impl Node {
  pub fn new(edges: Vec<Edge>) -> Node {
    Node {
      edges: edges,
      done: false,
      cost: None,
    }
  }

  pub fn set_start(&mut self) {
    self.cost = Some(0);
  }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
  cost: i32,
  node: usize,
}

impl Ord for State {
  fn cmp(&self, other: &State) -> Ordering {
    other.cost.cmp(&self.cost)
  }
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &State) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Node {
  pub edges: Vec<Edge>,
  pub done: bool,
  pub cost: Option<i32>,
}

//スタートのコストを0とすること
pub fn dijsktra(nodes: &mut Vec<Node>, heap: &mut BinaryHeap<State>) {
  let done_node = heap.pop().map(|State { node, cost: _ }| node);

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
        heap.push(State {
          node: edge.to,
          cost: cost,
        });
      }
    }

    dijsktra(nodes, heap);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let mut heap = BinaryHeap::new();
    heap.push(State { cost: 0, node: 0 });

    let mut nodes = vec![
      Node::new(vec![Edge { to: 2, cost: 10 }, Edge { to: 1, cost: 1 }]),
      Node::new(vec![Edge { to: 3, cost: 2 }]),
      Node::new(vec![
        Edge { to: 1, cost: 1 },
        Edge { to: 3, cost: 3 },
        Edge { to: 4, cost: 1 },
      ]),
      Node::new(vec![Edge { to: 0, cost: 7 }, Edge { to: 4, cost: 2 }]),
      Node::new(vec![]),
    ];

    nodes[0].set_start();
    dijsktra(&mut nodes, &mut heap);

    assert_eq!(
      vec![
        Node {
          edges: vec![Edge { to: 2, cost: 10 }, Edge { to: 1, cost: 1 }],
          done: true,
          cost: Some(0),
        },
        Node {
          edges: vec![Edge { to: 3, cost: 2 }],
          done: true,
          cost: Some(1),
        },
        Node {
          edges: vec![
            Edge { to: 1, cost: 1 },
            Edge { to: 3, cost: 3 },
            Edge { to: 4, cost: 1 },
          ],
          done: true,
          cost: Some(10),
        },
        Node {
          edges: vec![Edge { to: 0, cost: 7 }, Edge { to: 4, cost: 2 }],
          done: true,
          cost: Some(3),
        },
        Node {
          edges: vec![],
          done: true,
          cost: Some(5),
        },
      ],
      nodes
    );
  }
}
