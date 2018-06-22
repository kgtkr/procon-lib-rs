use std::cmp::min;

#[derive(PartialEq, Debug, Clone)]
struct Edge {
  pub to: usize,
  pub cap: i64,
  pub rev: usize,
}

struct FordFulkerson(Vec<(Vec<Edge>, bool)>);

impl FordFulkerson {
  pub fn new(n: usize) -> FordFulkerson {
    let v = (Vec::new(), false);
    let mut nodes = Vec::with_capacity(n);
    nodes.resize(n, v);
    FordFulkerson(nodes)
  }

  pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
    let f_size = self.0[from].0.len();
    let t_size = self.0[to].0.len();
    self.0[from].0.push(Edge {
      to: to,
      cap: cap,
      rev: t_size,
    });
    self.0[to].0.push(Edge {
      to: from,
      cap: 0,
      rev: f_size,
    });
  }

  pub fn dfs(&mut self, v: usize, t: usize, f: i64) -> i64 {
    if v == t {
      return f;
    }
    self.0[v].1 = true;
    for i in 0..self.0[v].0.len() {
      if !self.0[self.0[v].0[i].to].1 && self.0[v].0[i].cap > 0 {
        let d = {
          let x = self.0[v].0[i].to;
          let y = min(f, self.0[v].0[i].cap);
          self.dfs(x, t, y)
        };
        if d > 0 {
          self.0[v].0[i].cap -= d;
          {
            let x = self.0[v].0[i].to;
            let y = self.0[v].0[i].rev;
            self.0[x].0[y].cap += d;
          }
          return d;
        }
      }
    }
    0
  }

  pub fn max_flow(&mut self, s: usize, t: usize) -> i64 {
    let mut flow = 0;
    loop {
      for i in 0..self.0.len() {
        self.0[i].1 = false;
      }
      let f = self.dfs(s, t, <i64>::max_value());
      if f == 0 {
        return flow;
      }
      flow += f;
    }
  }
}

mod tests {
  use super::*;

  #[test]
  fn test1() {
    let mut flow = FordFulkerson::new(5);
    flow.add_edge(0, 1, 10);
    flow.add_edge(0, 2, 2);
    flow.add_edge(1, 2, 6);
    flow.add_edge(1, 3, 6);
    flow.add_edge(2, 4, 5);
    flow.add_edge(3, 2, 3);
    flow.add_edge(3, 4, 8);
    assert_eq!(11, flow.max_flow(0, 4));
  }
}
