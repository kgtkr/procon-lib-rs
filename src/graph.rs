//http://agtn.hatenablog.com/entry/2017/01/16/151745

pub type NodeId = usize;

pub type Graph = Vec<Node>;

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
