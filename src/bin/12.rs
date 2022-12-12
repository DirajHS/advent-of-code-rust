use std::collections::VecDeque;
use std::usize;

#[derive(Clone, PartialEq, Copy, Default)]
pub struct Hill {
  height: char,
  row: i32,
  col: i32,
  distance: usize,
}

pub fn get_input_matrix(input: &str, is_second_part: bool) -> (Vec<Vec<Hill>>, Vec<Hill>, Hill) {
  let mut matrix: Vec<Vec<Hill>> = Vec::new();
  let mut start: Vec<Hill> = Vec::new();
  let mut destination: Hill = Hill::default();
  for line in input.lines().enumerate() {
    let mut row: Vec<Hill> = Vec::new();
    for hill_size in line.1.char_indices() {
      row.push(Hill {
        height: if hill_size.1 == 'S' {
          start.push(Hill {
            height: 'a',
            row: line.0 as i32,
            col: hill_size.0 as i32,
            distance: 0,
          });
          'a'
        } else if hill_size.1 == 'E' {
          destination = Hill {
            height: 'z',
            row: line.0 as i32,
            col: hill_size.0 as i32,
            distance: 0,
          };
          'z'
        } else {
          if hill_size.1 == 'a' {
            if is_second_part {
              start.push(Hill {
                height: 'a',
                row: line.0 as i32,
                col: hill_size.0 as i32,
                distance: 0,
              });
            }
          }
          hill_size.1
        },
        row: line.0 as i32,
        col: hill_size.0 as i32,
        distance: 0,
      });
    }
    matrix.push(row);
  }
  return (matrix, start, destination);
}

pub fn can_visit(current_hill: &Hill, next_hill: &Hill) -> bool {
  return if current_hill.height == next_hill.height ||
    (current_hill.height as u32 + 1) == (next_hill.height as u32) {
    true
  } else if (current_hill.height as u32) > (next_hill.height as u32) {
    true
  } else {
    false
  }
}

pub fn update_bfs_queue(bfs_queue: &mut VecDeque<Hill>, visited: &mut Vec<Vec<bool>>, current_hill: &Hill, next_hill: &Hill) {
  if can_visit(&current_hill, &next_hill) {
    bfs_queue.push_back(Hill {
      distance: current_hill.distance + 1,
      ..*next_hill
    });
    visited[next_hill.row as usize][next_hill.col as usize] = true;
  }
}

pub fn bfs(forest_matrix: &Vec<Vec<Hill>>, start: &Hill, destination: &Hill) -> Option<usize> {
  let mut visited: Vec<Vec<bool>> = vec![vec![false; forest_matrix[0].len()]; forest_matrix.len()];
  let mut bfs_queue: VecDeque<Hill> = VecDeque::new();

  bfs_queue.push_back(start.clone());
  visited[start.row as usize][start.col as usize] = true;
  while !bfs_queue.is_empty() {
    let current_hill = bfs_queue.pop_front()?;

    //println!("{} {} {} {}", current_hill.height, current_hill.row, current_hill.col, current_hill.distance);
    if current_hill.row == destination.row && current_hill.col == destination.col {
      return Some(current_hill.distance);
    }

    let mut next_row: usize;
    let mut next_col: usize;

    //up
    if current_hill.row - 1 >= 0 &&
      !visited[current_hill.row as usize - 1][current_hill.col as usize] {
      next_row = (current_hill.row - 1) as usize;
      next_col = current_hill.col as usize;
      update_bfs_queue(&mut bfs_queue, &mut visited, &current_hill, &forest_matrix[next_row][next_col]);
    }

    //down
    if current_hill.row + 1 < forest_matrix.len() as i32 &&
      !visited[current_hill.row as usize + 1][current_hill.col as usize] {
      next_row = (current_hill.row + 1) as usize;
      next_col = current_hill.col as usize;
      update_bfs_queue(&mut bfs_queue, &mut visited, &current_hill, &forest_matrix[next_row][next_col]);
    }

    //left
    if current_hill.col - 1 >= 0 &&
      !visited[current_hill.row as usize][current_hill.col as usize - 1] {
      next_row = current_hill.row as usize;
      next_col = current_hill.col as usize - 1;
      update_bfs_queue(&mut bfs_queue, &mut visited, &current_hill, &forest_matrix[next_row][next_col]);
    }

    //right
    if current_hill.col + 1 < forest_matrix[0].len() as i32 &&
      !visited[current_hill.row as usize][current_hill.col as usize + 1] {
      next_row = current_hill.row as usize;
      next_col = current_hill.col as usize + 1;
      update_bfs_queue(&mut bfs_queue, &mut visited, &current_hill, &forest_matrix[next_row][next_col]);
    }
  }
  return Some(u32::MAX as usize);
}

pub fn part_one(input: &str) -> Option<u32> {
  let (forest_matrix, start, destination) = get_input_matrix(&input, false);
  let distance = bfs(&forest_matrix, &start[0], &destination)?;
  return Some(distance as u32);
}

pub fn part_two(input: &str) -> Option<u32> {
  let (forest_matrix, possible_start, destination) = get_input_matrix(&input, true);
  let mut distances: Vec<u32> = Vec::new();
  for start in possible_start {
    let distance = bfs(&forest_matrix, &start, &destination)?;
    //println!("possible start: {},{}: {}", start.row, start.col, distance);
    distances.push(distance as u32);
  }
  return Some(*distances.iter().min().unwrap());
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 12);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 12);
    assert_eq!(part_one(&input), Some(31));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 12);
    assert_eq!(part_two(&input), Some(29));
  }
}
