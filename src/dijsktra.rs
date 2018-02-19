#[derive(Debug, PartialEq, Clone)]
pub struct Node {
  pub edges: Vec<(usize, f64)>,
  pub done: bool,
  pub cost: f64,
  pub id: usize,
  pub previous_node: Option<usize>,
}

impl Node {
  pub fn new(id: usize) -> Node {
    Node {
      edges: Vec::new(),
      done: false,
      cost: -1.0,
      id: id,
      previous_node: None,
    }
  }

  pub fn add_node(&mut self, id: usize, cost: f64) {
    self.edges.push((id, cost));
  }
}

pub fn min_route(mut nodes: Vec<Node>) -> (f64, Vec<Node>) {
  nodes[0].cost = 0.0;

  loop {
    let mut process_node: Option<usize> = None;

    for node in &nodes {
      // 訪問済み or まだコストが未設定
      if node.done || node.cost < 0.0 {
        continue;
      }

      match process_node {
        Option::None => process_node = Some(node.id),
        Option::Some(pn) => {
          if node.cost < nodes[pn].cost {
            process_node = Some(node.id);
          }
        }
      }
    }

    match process_node {
      Option::Some(pn) => {
        nodes[pn].done = true;

        for edge in nodes[pn].edges.clone() {
          let node = edge.0;
          let cost = nodes[pn].cost + edge.1;

          // コストが未設定 or コストの少ない経路がある場合はアップデート
          let needs_update = (nodes[node].cost < 0.0) || (nodes[node].cost > cost);
          if needs_update {
            nodes[node].cost = cost;
            nodes[node].previous_node = Some(pn);
          }
        }
      }
      None => {
        break;
      }
    }
  }

  let goal_node = nodes.last().unwrap();

  let mut path: Vec<Node> = Vec::new();
  let mut current_node = goal_node;
  loop {
    let next_node = current_node.previous_node;
    match next_node {
      Option::None => {
        break;
      }
      Option::Some(next_node) => {
        path.push(nodes[next_node].clone());
        current_node = &nodes[next_node];
      }
    }
  }

  let sum = path
    .clone()
    .into_iter()
    .filter_map(|x| {
      x.previous_node
        .map(|id| x.edges.iter().find(|x| x.0 == id).unwrap().1)
    })
    .sum::<f64>() + path[0].edges.last().unwrap().1;

  (sum, path)
}
