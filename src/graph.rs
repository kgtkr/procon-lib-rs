//http://agtn.hatenablog.com/entry/2017/01/16/151745

pub type NodeId = usize;

pub type Cost = i64;

pub type Graph = Vec<Node>;

pub type Node = Vec<Edge>;

pub type Edge = (NodeId, Cost);
