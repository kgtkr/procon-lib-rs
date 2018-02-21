//http://agtn.hatenablog.com/entry/2017/01/16/151745

pub type NodeId = usize;

pub struct NodeArena {
  pub arena: Vec<Node>,
}

impl NodeArena {
  pub fn new() -> NodeArena {
    NodeArena { arena: Vec::new() }
  }

  pub fn alloc(&mut self) -> NodeId {
    let id = self.arena.len();
    let node = Node {
      id: id,
      edges: Vec::new(),
    };
    self.arena.push(node);
    id
  }

  pub fn add_edge(&mut self, node: NodeId, to: NodeId, cost: i32) {
    self.arena[node].edges.push(Edge { to: to, cost: cost });
  }

  pub fn get(&self, id: NodeId) -> &Node {
    &self.arena[id]
  }
  pub fn get_mut(&mut self, id: NodeId) -> &mut Node {
    &mut self.arena[id]
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Node {
  pub id: NodeId,
  pub edges: Vec<Edge>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Edge {
  pub to: NodeId,
  pub cost: i32,
}
