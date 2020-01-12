use graph;

//graphは全てのコストが等しい
pub fn bfs(
  &graph::ListGraph(ref graph): &graph::ListGraph,
  start: graph::NodeId,
  goal: graph::NodeId,
) -> Option<Vec<graph::NodeId>> {
  let mut visited = Vec::with_capacity(graph.len());
  visited.resize(graph.len(), None);

  // Queue に初期値を積む
  let mut queue = vec![start];
  while queue.len() > 0 {
    // queueから取り出す
    let n = queue[0];
    queue.remove(0);
    // 行けるノード
    for &(_, next_node, _) in &graph[n] {
      if visited[next_node].is_none() {
        visited[next_node] = Some(n);
        // Queueに積む
        queue.push(next_node)
      }

      if next_node == goal {
        visited[start] = None;
        let mut path = vec![goal];
        let mut c = visited[goal];
        while let Some(id) = c {
          path.push(id);
          c = visited[id];
        }
        path.reverse();
        return Some(path);
      }
    }
  }
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let mut maze = graph::Maze(vec![
      vec![true, false, true, false, true],
      vec![true, false, true, true, true],
      vec![true, false, true, false, true],
      vec![true, true, true, false, true],
      vec![true, false, true, false, true],
    ]);
    //y*5+x
    assert_eq!(
      Some(vec![0, 3, 7, 10, 11, 12, 8, 4, 5, 6, 9, 13, 16]),
      bfs(
        &graph::ListGraph::from(graph::MazeID::from(maze.clone())),
        0,
        16
      )
    );

    assert_eq!(
      Some(vec![16, 13, 9, 6, 5, 4, 8, 12, 11, 10, 7, 3, 0]),
      bfs(
        &graph::ListGraph::from(graph::MazeID::from(maze.clone())),
        16,
        0
      )
    );

    maze.0[3][1] = false;
    assert_eq!(
      None,
      bfs(
        &graph::ListGraph::from(graph::MazeID::from(maze.clone())),
        0,
        16
      )
    );
  }
}
