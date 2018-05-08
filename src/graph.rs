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
pub struct FlatGraph(pub usize, pub Vec<Edge>);

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

#[derive(PartialEq, Debug, Clone)]
pub struct MazeID(pub Vec<Vec<Option<usize>>>);

//迷路
#[derive(PartialEq, Debug, Clone)]
pub struct Maze(pub Vec<Vec<bool>>);

impl From<Maze> for MazeID {
  fn from(Maze(maze): Maze) -> MazeID {
    let mut id = 0;
    MazeID(
      maze
        .into_iter()
        .map(|vec| {
          vec
            .into_iter()
            .map(|x| {
              if x {
                let res: Option<usize> = Some(id);
                id += 1;
                res
              } else {
                None
              }
            })
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>(),
    )
  }
}

impl From<Maze> for ListGraph {
  fn from(Maze(maze): Maze) -> ListGraph {
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
  fn list_to_matrix() {
    assert_eq!(
      MatrixGraph(vec![
        vec![None, Some(1), None, Some(3)],
        vec![Some(10), Some(1), None, None],
        vec![None, None, Some(5), None],
        vec![None, None, None, None],
      ]),
      MatrixGraph::from(ListGraph(vec![
        vec![(0, 1, 1), (0, 3, 3)],
        vec![(1, 0, 10), (1, 1, 1)],
        vec![(2, 2, 5)],
        vec![],
      ]))
    );
  }

  #[test]
  fn flat_to_matrix() {
    assert_eq!(
      MatrixGraph(vec![
        vec![None, Some(1), None, Some(3)],
        vec![Some(10), Some(1), None, None],
        vec![None, None, Some(5), None],
        vec![None, None, None, None],
      ]),
      MatrixGraph::from(FlatGraph(
        4,
        vec![(0, 1, 1), (0, 3, 3), (1, 0, 10), (1, 1, 1), (2, 2, 5)]
      ))
    );
  }

  #[test]
  fn data_to_list() {
    assert_eq!(
      ListGraph(vec![
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

  #[test]
  fn flat_to_list() {
    assert_eq!(
      ListGraph(vec![
        vec![(0, 1, 1), (0, 3, 3)],
        vec![(1, 0, 10), (1, 1, 1)],
        vec![(2, 2, 5)],
        vec![],
      ]),
      ListGraph::from(FlatGraph(
        4,
        vec![(0, 1, 1), (0, 3, 3), (1, 0, 10), (1, 1, 1), (2, 2, 5)]
      ))
    );
  }

  #[test]
  fn list_to_flat() {
    assert_eq!(
      FlatGraph(
        4,
        vec![(0, 1, 1), (0, 3, 3), (1, 0, 10), (1, 1, 1), (2, 2, 5)]
      ),
      FlatGraph::from(ListGraph(vec![
        vec![(0, 1, 1), (0, 3, 3)],
        vec![(1, 0, 10), (1, 1, 1)],
        vec![(2, 2, 5)],
        vec![],
      ]))
    );
  }

  #[test]
  fn mtrix_to_flat() {
    assert_eq!(
      FlatGraph(
        4,
        vec![(0, 1, 1), (0, 3, 3), (1, 0, 10), (1, 1, 1), (2, 2, 5)]
      ),
      FlatGraph::from(MatrixGraph(vec![
        vec![None, Some(1), None, Some(3)],
        vec![Some(10), Some(1), None, None],
        vec![None, None, Some(5), None],
        vec![None, None, None, None],
      ]))
    );
  }

  #[test]
  fn maze_to_list() {
    assert_eq!(
      ListGraph(vec![
        vec![(0, 4, 1)],
        vec![],
        vec![(2, 6, 1)],
        vec![],
        vec![(4, 0, 1), (4, 5, 1), (4, 8, 1)],
        vec![(5, 4, 1), (5, 6, 1)],
        vec![(6, 2, 1), (6, 5, 1), (6, 7, 1), (6, 10, 1)],
        vec![(7, 6, 1)],
        vec![(8, 4, 1)],
        vec![],
        vec![(10, 6, 1)],
        vec![],
      ]),
      ListGraph::from(Maze(vec![
        vec![true, false, true, false],
        vec![true, true, true, true],
        vec![true, false, true, false],
      ]))
    );
  }

  #[test]
  fn maze_to_maze_id() {
    assert_eq!(
      MazeID(vec![
        vec![Some(0), None, Some(1), None],
        vec![Some(2), Some(3), Some(4), Some(5)],
        vec![Some(6), None, Some(7), None],
      ]),
      MazeID::from(Maze(vec![
        vec![true, false, true, false],
        vec![true, true, true, true],
        vec![true, false, true, false],
      ]))
    );
  }
}
