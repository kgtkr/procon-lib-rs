use graph;

//graphは全てのコストが等しい
pub fn bfs(
  &graph::GraphFromNodes(ref graph): &graph::GraphFromNodes,
  start: graph::NodeId,
  goal: graph::NodeId,
) -> Option<Vec<graph::NodeId>> {
  let mut visited = Vec::with_capacity(graph.len());
  visited.resize(graph.len(), None);

  let mut n = 0;
  // Queue に初期値を積む
  let mut queue = vec![n];
  while queue.len() > 0 {
    // queueから取り出す
    n = queue[0];
    queue.remove(0);
    // 行けるノード
    for &(next_node, _) in &graph[n] {
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
    let mut maze = vec![
      vec![true, false, true, false, true],
      vec![true, false, true, true, true],
      vec![true, false, true, false, true],
      vec![true, true, true, false, true],
      vec![true, false, true, false, true],
    ];
    //y*5+x
    assert_eq!(
      Some(vec![0, 5, 10, 15, 16, 17, 12, 7, 8, 9, 14, 19, 24]),
      bfs(&graph::maze_to_graph(maze.clone()), 0, 24)
    );

    maze[3][1] = false;
    assert_eq!(None, bfs(&graph::maze_to_graph(maze.clone()), 0, 24));
  }
}
