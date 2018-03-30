pub type NodeId = usize;

pub type Cost = i64;

pub type Node = Vec<Edge>;

pub type Edge = (NodeId, NodeId, Cost);

#[derive(PartialEq, Debug, Clone)]
pub struct MatrixGraph(pub Vec<Vec<Option<Cost>>>);

impl From<Vec<Vec<Option<Cost>>>> for MatrixGraph {
  fn from(data: Vec<Vec<Option<Cost>>>) -> MatrixGraph {
    MatrixGraph(data)
  }
}

impl From<ListGraph> for MatrixGraph {
  fn from(graph: ListGraph) -> MatrixGraph {
    FlatGraph::from(graph).into()
  }
}

impl From<FlatGraph> for MatrixGraph {
  fn from(FlatGraph(len, data): FlatGraph) -> MatrixGraph {
    let mut vec = Vec::with_capacity(len);
    vec.resize(len, {
      let mut v = Vec::with_capacity(len);
      v.resize(len, None);
      v
    });

    for (from, to, cost) in data {
      vec[from][to] = Some(cost);
    }

    vec.into()
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ListGraph(pub Vec<Vec<Edge>>);

impl From<Vec<Vec<(NodeId, Cost)>>> for ListGraph {
  fn from(data: Vec<Vec<(NodeId, Cost)>>) -> ListGraph {
    ListGraph(
      data
        .into_iter()
        .enumerate()
        .map(|(from, edges)| {
          edges
            .into_iter()
            .map(|(to, cost)| (from, to, cost))
            .collect()
        })
        .collect(),
    )
  }
}

impl From<Vec<Vec<Edge>>> for ListGraph {
  fn from(data: Vec<Vec<Edge>>) -> ListGraph {
    ListGraph(data)
  }
}

impl From<FlatGraph> for ListGraph {
  fn from(FlatGraph(len, data): FlatGraph) -> ListGraph {
    let mut vec = Vec::with_capacity(len);
    vec.resize(len, Vec::new());

    for (from, to, cost) in data {
      vec[from].push((from, to, cost));
    }

    ListGraph(vec)
  }
}

impl From<MatrixGraph> for ListGraph {
  fn from(graph: MatrixGraph) -> ListGraph {
    FlatGraph::from(graph).into()
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FlatGraph(usize, pub Vec<Edge>);

impl From<ListGraph> for FlatGraph {
  fn from(ListGraph(data): ListGraph) -> FlatGraph {
    let len = data.len();
    (len, data.into_iter().flat_map(|x| x).collect::<Vec<_>>()).into()
  }
}

impl From<(usize, Vec<(NodeId, NodeId, Cost)>)> for FlatGraph {
  fn from((len, data): (usize, Vec<(NodeId, NodeId, Cost)>)) -> FlatGraph {
    FlatGraph(len, data)
  }
}

impl From<MatrixGraph> for FlatGraph {
  fn from(MatrixGraph(data): MatrixGraph) -> FlatGraph {
    let mut vec = Vec::new();
    let len = data.len();

    for (from, v) in data.into_iter().enumerate() {
      for (to, cost) in v.into_iter().enumerate() {
        if let Some(cost) = cost {
          vec.push((from, to, cost));
        }
      }
    }

    (len, vec).into()
  }
}

//迷路
pub type Maze = Vec<Vec<bool>>;

impl From<Maze> for ListGraph {
  fn from(maze: Maze) -> ListGraph {
    if maze.len() == 0 {
      return Vec::<Vec<(NodeId, Cost)>>::new().into();
    }

    let h = maze.len();
    let w = maze[0].len();

    let mut graph = Vec::new();
    for y in 0..h {
      for x in 0..w {
        if maze[y][x] {
          let mut edges = Vec::new();
          if y != 0 && maze[y - 1][x] {
            edges.push((x + (y - 1) * w, 1));
          }
          if x != 0 && maze[y][x - 1] {
            edges.push(((x - 1) + y * w, 1));
          }
          if x != w - 1 && maze[y][x + 1] {
            edges.push(((x + 1) + y * w, 1));
          }
          if y != h - 1 && maze[y + 1][x] {
            edges.push((x + (y + 1) * w, 1));
          }
          graph.push(edges);
        } else {
          graph.push(Vec::new());
        }
      }
    }
    graph.into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn maze_to_list() {
    let mut maze = vec![
      vec![true, false, true, false], //0-3
      vec![true, true, true, true],   //4-7
      vec![true, false, true, false], //8-11
    ];
    assert_eq!(
      ListGraph::from(vec![
        vec![(4, 1)],
        vec![],
        vec![(6, 1)],
        vec![],
        vec![(0, 1), (5, 1), (8, 1)],
        vec![(4, 1), (6, 1)],
        vec![(2, 1), (5, 1), (7, 1), (10, 1)],
        vec![(6, 1)],
        vec![(4, 1)],
        vec![],
        vec![(6, 1)],
        vec![],
      ]),
      maze.into()
    );
  }

  #[test]
  fn list_to_matrix() {
    assert_eq!(
      MatrixGraph::from(vec![
        vec![None, Some(1), None, Some(3)],
        vec![Some(10), Some(1), None, None],
        vec![None, None, Some(5), None],
        vec![None, None, None, None],
      ]),
      ListGraph::from(vec![
        vec![(1, 1), (3, 3)],
        vec![(0, 10), (1, 1)],
        vec![(2, 5)],
        vec![],
      ]).into()
    );
  }

  #[test]
  fn flat_to_matrix() {
    assert_eq!(
      MatrixGraph::from(vec![
        vec![None, Some(1), None, Some(3)],
        vec![Some(10), Some(1), None, None],
        vec![None, None, Some(5), None],
        vec![None, None, None, None],
      ]),
      FlatGraph::from((
        4,
        vec![(0, 1, 1), (0, 3, 3), (1, 0, 10), (1, 1, 1), (2, 2, 5)]
      )).into()
    );
  }

  #[test]
  fn data_to_list() {
    assert_eq!(
      ListGraph::from(vec![
        vec![(0, 1, 1), (0, 3, 3)],
        vec![(1, 0, 10), (1, 1, 1)],
        vec![(2, 2, 5)],
        vec![],
      ]),
      ListGraph::from(vec![
        vec![(1, 1), (3, 3)],
        vec![(0, 10), (1, 1)],
        vec![(2, 5)],
        vec![],
      ])
    );
  }
}
