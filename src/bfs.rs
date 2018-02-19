#[derive(PartialEq, Debug, Clone)]
enum Cell {
  Wall,
  Empty,
  Visited(usize, usize),
}
pub fn bfs(m: &Vec<Vec<bool>>) -> Option<Vec<(usize, usize)>> {
  let mut m = m.iter()
    .map(|x| {
      x.iter()
        .map(|&x| if x { Cell::Empty } else { Cell::Wall })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();
  let goal = (m.len() - 1, m[0].len() - 1);
  let mut n: (usize, usize) = (0, 0);
  // Queue に初期値を積む
  let mut queue = vec![n];
  while queue.len() > 0 {
    // queueから取り出す
    n = queue[0];
    queue.remove(0);
    // 行けるセルとをチェック
    for d in vec![(1i32, 0i32), (0i32, 1i32), (-1i32, 0i32), (0i32, -1i32)] {
      let next_x = (n.0 as i32) + d.0;
      let next_y = (n.1 as i32) + d.1;
      if next_x >= 0 && next_x <= (goal.0 as i32) && next_y >= 0 && next_y <= (goal.1 as i32) {
        let next_x = next_x as usize;
        let next_y = next_y as usize;

        if m[next_x][next_y] == Cell::Empty {
          m[next_x][next_y] = Cell::Visited(n.0, n.1);
          // Queueに積む
          queue.push((next_x, next_y))
        }

        if (next_x, next_y) == goal {
          m[0][0] = Cell::Empty;
          let mut path = vec![(goal.0, goal.1)];
          let mut c = &m[goal.0][goal.1];
          while let &Cell::Visited(x, y) = c {
            path.push((x, y));
            c = &m[x][y];
          }
          path.reverse();
          return Some(path);
        }
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
    let mut map = vec![
      vec![true, false, true, false, true],
      vec![true, false, true, true, true],
      vec![true, false, true, false, true],
      vec![true, true, true, false, true],
      vec![true, false, true, false, true],
    ];
    assert_eq!(
      Some(vec![
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (3, 1),
        (3, 2),
        (2, 2),
        (1, 2),
        (1, 3),
        (1, 4),
        (2, 4),
        (3, 4),
        (4, 4),
      ]),
      bfs(&map)
    );

    map[3][1] = false;
    assert_eq!(None, bfs(&map));
  }
}
