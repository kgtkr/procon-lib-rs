use std::cmp::Ordering;
use std::collections::BinaryHeap;
use graph;

#[derive(PartialEq, Debug, Clone)]
pub struct Node {
  pub edges: Vec<graph::Edge>,
  pub done: bool,
  pub cost: Option<i32>,
  pub before: Option<usize>,
}

impl Node {
  pub fn new(node: graph::Node) -> Node {
    Node {
      edges: node.edges,
      done: false,
      cost: None,
      before: None,
    }
  }

  pub fn set_start(&mut self) {
    self.cost = Some(0);
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Dijsktra {
  pub nodes: Vec<Node>,
}

impl Dijsktra {
  pub fn new(nodes: graph::Graph) -> Dijsktra {
    Dijsktra {
      nodes: nodes
        .into_iter()
        .map(|node| Node::new(node))
        .collect::<Vec<_>>(),
    }
  }

  pub fn cost(&self, node: graph::NodeId) -> Option<i32> {
    self.nodes[node].cost
  }

  pub fn dijsktra(&mut self, start: graph::NodeId) {
    let mut heap = BinaryHeap::new();
    heap.push(State {
      cost: 0,
      node: start,
    });
    self.nodes[start].set_start();

    self.dijsktra_r(&mut heap);
  }
  //スタートのコストを0とすること
  fn dijsktra_r(&mut self, heap: &mut BinaryHeap<State>) {
    let done_node = heap.pop().map(|State { node, cost: _ }| node);

    if let Some(done_node) = done_node {
      self.nodes[done_node].done = true;
      for edge in self.nodes[done_node].edges.clone() {
        let cost = self.nodes[done_node].cost.unwrap() + edge.cost;
        if self.nodes[edge.to]
          .cost
          .map(|to_cost| cost < to_cost)
          .unwrap_or(true)
        {
          self.nodes[edge.to].cost = Some(cost);
          self.nodes[edge.to].before = Some(done_node);
          heap.push(State {
            node: edge.to,
            cost: cost,
          });
        }
      }

      self.dijsktra_r(heap);
    }
  }

  pub fn path(&self, goal: usize) -> Vec<usize> {
    let mut path = Vec::new();
    path.push(goal);
    let mut current = goal;
    while let Some(node) = self.nodes[current].before {
      path.push(node);
      current = node;
    }
    path.reverse();
    path
  }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let mut graph = vec![
      graph::Node {
        edges: vec![
          graph::Edge { to: 2, cost: 10 },
          graph::Edge { to: 1, cost: 1 },
        ],
      },
      graph::Node {
        edges: vec![graph::Edge { to: 3, cost: 2 }],
      },
      graph::Node {
        edges: vec![
          graph::Edge { to: 1, cost: 1 },
          graph::Edge { to: 3, cost: 3 },
          graph::Edge { to: 4, cost: 1 },
        ],
      },
      graph::Node {
        edges: vec![
          graph::Edge { to: 0, cost: 7 },
          graph::Edge { to: 4, cost: 2 },
        ],
      },
      graph::Node { edges: vec![] },
    ];

    let mut dij = Dijsktra::new(graph);

    dij.dijsktra(0);

    assert_eq!(Some(0), dij.cost(0));
    assert_eq!(Some(1), dij.cost(1));
    assert_eq!(Some(10), dij.cost(2));
    assert_eq!(Some(3), dij.cost(3));
    assert_eq!(Some(5), dij.cost(4));

    assert_eq!(vec![0], dij.path(0));
    assert_eq!(vec![0, 1], dij.path(1));
    assert_eq!(vec![0, 2], dij.path(2));
    assert_eq!(vec![0, 1, 3], dij.path(3));
    assert_eq!(vec![0, 1, 3, 4], dij.path(4));
  }
}
