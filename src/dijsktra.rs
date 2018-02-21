use std::cmp::Ordering;
use std::collections::BinaryHeap;
use graph;

#[derive(PartialEq, Debug, Clone)]
pub struct Node {
  pub edges: Vec<graph::Edge>,
  pub done: bool,
  pub cost: Option<i32>,
}

impl Node {
  pub fn new(node: graph::Node) -> Node {
    Node {
      edges: node.edges,
      done: false,
      cost: None,
    }
  }

  pub fn set_start(&mut self) {
    self.cost = Some(0);
  }
}

pub struct Dijsktra {
  pub nodes: Vec<Node>,
}

impl Dijsktra {
  pub fn new(nodes: graph::NodeArena) -> Dijsktra {
    Dijsktra {
      nodes: nodes
        .arena
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
          heap.push(State {
            node: edge.to,
            cost: cost,
          });
        }
      }

      self.dijsktra_r(heap);
    }
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

    let mut dij = Dijsktra::new(arena);

    dij.dijsktra(node0);

    assert_eq!(Some(0), dij.cost(node0));
    assert_eq!(Some(1), dij.cost(node1));
    assert_eq!(Some(10), dij.cost(node2));
    assert_eq!(Some(3), dij.cost(node3));
    assert_eq!(Some(5), dij.cost(node4));
  }
}
