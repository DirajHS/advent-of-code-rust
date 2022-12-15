#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos {
  x: usize,
  y: usize,
}

fn get_pos(input_str: &str) -> Option<Pos> {
  let mut position = Pos {
    x: 0,
    y: 0,
  };
  let mut i = 0;
  input_str.split(',')
    .into_iter()
    .for_each(|pos| {
      match i {
        //swap here
        0 => position.y = pos.trim().parse::<usize>().unwrap(),
        1 => position.x = pos.trim().parse::<usize>().unwrap(),
        _ => println!("wtf...{}", i)
      }
      i += 1;
    });
  return Some(position);
}

fn parse_input(input: &str, is_part_one: bool) -> Option<(Vec<Vec<char>>, Pos)> {
  const CAVE_SIZE: usize = 1000;
  let mut sand_cave: Vec<Vec<char>> = vec![vec!['.'; CAVE_SIZE]; CAVE_SIZE];
  let mut max_depth: Pos = Pos { x: 0, y: 0 };
  for line in input.lines() {
    line.split("->")
      .fold(line.split_at(line.find("->").unwrap()).0, |a, b| {
        let start = get_pos(a).unwrap();
        let end = get_pos(b).unwrap();
        if start.x < end.x {
          if max_depth.x < end.x {
            max_depth.x = end.x;
          }
          for i in start.x..=end.x {
            sand_cave[i][start.y] = '#';
          }
        } else {
          if max_depth.x < start.x {
            max_depth.x = start.x;
          }
          for i in end.x..=start.x {
            sand_cave[i][start.y] = '#';
          }
        }

        if start.y < end.y {
          for j in start.y..=end.y {
            sand_cave[start.x][j] = '#';
          }
        } else {
          for j in end.y..=start.y {
            sand_cave[start.x][j] = '#';
          }
        }

        b
      });
  }
  sand_cave[0][500] = '+';
  if !is_part_one {
    max_depth.x += 2;
    sand_cave[max_depth.x] = vec!['#'; CAVE_SIZE];
  }
  //print_cave(&sand_cave, Pos{ x: 0, y: 492}, Pos{ x: 10, y: 505});
  return Some((sand_cave, max_depth));
}

fn print_cave(cave: &Vec<Vec<char>>, start_pos: Pos, end_pos: Pos) {
  for i in start_pos.x..=end_pos.x {
    for j in start_pos.y..=end_pos.y {
      print!("{}", cave[i][j]);
    }
    println!();
  }
}

fn can_go_down(cave: &Vec<Vec<char>>, pos: &Pos, max_depth: &Pos) -> Option<Pos> {
  let neighbors: Vec<(i32, i32)> = vec![(1, 0), (1, -1), (1, 1)];
  for neighbor in neighbors {
    let next_pos = Pos {
      x: (pos.x as i32 + neighbor.0) as usize,
      y: (pos.y as i32 + neighbor.1) as usize,
    };
    if cave[next_pos.x][next_pos.y] == '.' && next_pos.x < max_depth.x {
      return Some(next_pos);
    }
  }

  return None;
}

fn dfs(mut cave: &mut Vec<Vec<char>>, start_pos: &mut Pos, max_depth: &Pos, mut result: &mut u32, mut overflown: &mut bool) {
  let neighbors: Vec<(i32, i32)> = vec![(1, 0), (1, -1), (1, 1)];
  for neighbor in neighbors {
    let mut next_pos = Pos {
      x: (start_pos.x as i32 + neighbor.0) as usize,
      y: (start_pos.y as i32 + neighbor.1) as usize,
    };
    if next_pos.x > max_depth.x {
      *overflown = true;
      return;
    }
    if cave[next_pos.x][next_pos.y] == '.' && cave[next_pos.x][next_pos.y] != 'o' {
      dfs(&mut cave, &mut next_pos, &max_depth, &mut result, &mut overflown);

      if *overflown {
        return;
      }
    }
  }
  if !*overflown {
    cave[start_pos.x][start_pos.y] = 'o';
    *result += 1;
    return;
  }
}

pub fn part_one(input: &str) -> Option<u32> {
  let (mut cave, max_depth) = parse_input(&input, true).unwrap();
  let mut start = Pos { x: 0, y: 500 };
  let mut result: u32 = 0;
  let mut overflown: bool = false;
  dfs(&mut cave, &mut start, &max_depth, &mut result, &mut overflown);

  // DFS Iterative
  /*let mut overflow_occurred = false;
  let mut stack: VecDeque<Pos> = VecDeque::new();
  let mut result = 0;
  while true {
    stack.clear();
    stack.push_back(start);
    if overflow_occurred {
      break;
    }
    while true {
      let current_pos = stack.pop_front().unwrap();
      match can_go_down(&cave, &current_pos, &max_depth) {
        Some(next_pos) => {
          stack.push_back(next_pos.clone());
          if next_pos.x > max_depth.x {
            overflow_occurred = true;
            break;
          }
        },
        None => {
          cave[current_pos.x][current_pos.y] = 'o';
          break;
        }
      }
    }
    result += 1;
    if overflow_occurred {
      break;
    }
    //print_cave(&cave, Pos{ x: 0, y: 492}, Pos{ x: 10, y: 505});
    //println!();
  }*/
  return Some(result);
}

pub fn part_two(input: &str) -> Option<u32> {
  let (mut cave, max_depth) = parse_input(&input, false).unwrap();
  let mut start = Pos { x: 0, y: 500 };
  let mut result: u32 = 0;
  let mut overflown: bool = false;
  dfs(&mut cave, &mut start, &max_depth, &mut result, &mut overflown);

  //DFS Iterative
  /*let mut overflow_occurred = false;
  let mut stack: VecDeque<Pos> = VecDeque::new();
  let mut result = 0;
  while true {
    stack.clear();
    stack.push_back(start);
    if overflow_occurred {
      break;
    }
    while true {
      let current_pos = stack.pop_front().unwrap();
      match can_go_down(&cave, &current_pos, &max_depth) {
        Some(next_pos) => {
          stack.push_back(next_pos.clone());
          if next_pos.x > max_depth.x {
            overflow_occurred = true;
            break;
          }
        },
        None => {
          if current_pos == start {
            cave[current_pos.x][current_pos.y] = 'o';
            overflow_occurred = true;
            break;
          }
          cave[current_pos.x][current_pos.y] = 'o';
          break;
        }
      }
    }
    result += 1;
    if overflow_occurred {
      break;
    }
    //print_cave(&cave, Pos{ x: 0, y: 492}, Pos{ x: 11, y: 505});
    //println!();
  }*/
  return Some(result);
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 14);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 14);
    assert_eq!(part_one(&input), Some(24));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 14);
    assert_eq!(part_two(&input), Some(93));
  }
}
