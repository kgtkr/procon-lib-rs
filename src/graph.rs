//http://agtn.hatenablog.com/entry/2017/01/16/151745

pub type NodeId = usize;

pub type Cost = i64;

#[derive(PartialEq, Debug, Clone)]
pub struct MinMemListGraph(pub Vec<Vec<(NodeId, Cost)>>);

#[derive(PartialEq, Debug, Clone)]
pub struct ListGraph(pub Vec<Node>);

pub type Node = Vec<Edge>;

pub type Edge = (NodeId, NodeId, Cost);

impl From<MinMemListGraph> for ListGraph {
  fn from(MinMemListGraph(data): MinMemListGraph) -> ListGraph {
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

impl From<ListGraph> for MinMemListGraph {
  fn from(ListGraph(data): ListGraph) -> MinMemListGraph {
    MinMemListGraph(
      data
        .into_iter()
        .map(|edges| edges.into_iter().map(|(_, to, cost)| (to, cost)).collect())
        .collect(),
    )
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FlatGraph(pub Vec<Edge>);

//迷路
#[derive(PartialEq, Debug, Clone)]
pub struct Maze(pub Vec<Vec<bool>>);

//id=x+y*w
pub fn maze_to_graph(Maze(maze): Maze) -> MinMemListGraph {
  if maze.len() == 0 {
    return MinMemListGraph(Vec::new());
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
  MinMemListGraph(graph)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let mut maze = vec![
      vec![true, false, true, false], //0-3
      vec![true, true, true, true],   //4-7
      vec![true, false, true, false], //8-11
    ];
    assert_eq!(
      MinMemListGraph(vec![
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
      maze_to_graph(Maze(maze)).into()
    );
  }
}
