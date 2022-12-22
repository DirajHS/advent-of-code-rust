use std::cmp::max;
use std::collections::HashMap;

static SIZE: usize = 7;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Pos {
  x: usize,
  y: usize,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Rock {
  offset: Pos,
  shape: Vec<Vec<char>>,
}

impl Rock {
  fn go_left(&mut self, tunnel: &Vec<Vec<char>>) -> Option<Pos> {
    for col in (0..self.shape[0].len()).rev() {
      for row in (0..self.shape.len()).rev() {
        let mut col_offset = self.offset.y + col;
        if col_offset <= 0 {
          return None;
        }
        col_offset -= 1;
        if self.shape[row][col] == '#' && tunnel[self.offset.x + row][col_offset] != '.' {
          return None;
        }
      }
    }
    return Some(Pos {
      x: self.offset.x,
      y: self.offset.y - 1,
    });
  }
  fn go_right(&mut self, tunnel: &Vec<Vec<char>>) -> Option<Pos> {
    for col in (0..self.shape[0].len()).rev() {
      for row in (0..self.shape.len()).rev() {
        let col_offset = self.offset.y + col + 1;
        if col_offset >= SIZE {
          return None;
        }
        if self.shape[row][col] == '#' && tunnel[self.offset.x + row][col_offset] != '.' {
          return None;
        }
      }
    }
    return Some(Pos {
      x: self.offset.x,
      y: self.offset.y + 1,
    });
  }
  fn go_up(&mut self, tunnel: &Vec<Vec<char>>) -> Option<Pos> {
    for col in (0..self.shape[0].len()).rev() {
      for row in (0..self.shape.len()).rev() {
        let mut row_offset = self.offset.x + row;
        if row_offset <= 0 {
          return None;
        }
        row_offset -= 1;
        if self.shape[row][col] == '#' && tunnel[row_offset][self.offset.y + col] != '.' {
          return None;
        }
      }
    }
    return Some(Pos {
      x: self.offset.x - 1,
      y: self.offset.y,
    });
  }
}

fn get_default_rocks() -> Vec<Rock> {
  let mut rocks: Vec<Rock> = Vec::new();
  rocks.push(Rock {
    offset: Pos { x: 0, y: 0 },
    shape: vec![vec!['#'; 4]; 1],
  });
  rocks.push(
    Rock {
      offset: Pos { x: 0, y: 0 },
      shape: vec![
        vec!['.', '#', '.'],
        vec!['#', '#', '#'],
        vec!['.', '#', '.'],
      ],
    });
  rocks.push(
    Rock {
      offset: Pos { x: 0, y: 0 },
      shape: vec![
        vec!['#', '#', '#'],
        vec!['#', '.', '.'],
        vec!['#', '.', '.'],
      ],
    });
  rocks.push(
    Rock {
      offset: Pos { x: 0, y: 0 },
      shape: vec![
        vec!['#'],
        vec!['#'],
        vec!['#'],
        vec!['#'],
      ],
    });
  rocks.push(
    Rock {
      offset: Pos { x: 0, y: 0 },
      shape: vec![
        vec!['#', '#'],
        vec!['#', '#'],
      ],
    });
  return rocks;
}

fn update_tunnel(rock: &Rock, max_height: &mut usize, tunnel: &mut Vec<Vec<char>>) {
  for row in rock.offset.x..rock.offset.x + rock.shape.len() {
    for col in rock.offset.y..rock.offset.y + rock.shape[0].len() {
      if rock.shape[row - rock.offset.x][col - rock.offset.y] != '.' {
        tunnel[row][col] = rock.shape[row - rock.offset.x][col - rock.offset.y];
      }
    }
  }
  'outer: for row in (0..4 + *max_height).rev() {
    for col in 0..SIZE {
      if tunnel[row as usize][col] == '#' {
        *max_height = row + 1;
        break 'outer;
      }
    }
  }
}

fn get_new_rock(rock_counter: &mut usize, rocks: &Vec<Rock>, max_height: usize) -> Rock {
  let mut rock = rocks[*rock_counter % 5].clone();
  *rock_counter += 1;
  rock.offset.x += 3 + max_height as usize;
  rock.offset.y += (SIZE - rock.shape[0].len()) - 2;
  return rock;
}

fn try_going_up(rock: &mut Rock, tunnel: &mut Vec<Vec<char>>, counter: &mut usize, max_height: &mut usize, rock_counter: &mut usize, rocks: &Vec<Rock>) -> Rock {
  let new_offset = rock.go_up(&tunnel);
  let mut new_rock = rock.clone();
  match new_offset {
    Some(new_offset) => {
      new_rock.offset = new_offset;
    }
    None => {
      *counter += 1;
      update_tunnel(&rock, max_height, tunnel);
      //print_tunnel(&tunnel, (max_height + 3) as usize);
      new_rock = get_new_rock(rock_counter, rocks, *max_height).clone();

      //println!("new rock: {:?} {} {} {}", &new_rock, free_rows, tunnel.len(), max_height);
      let required_rows = *max_height + (new_rock.offset.x + new_rock.shape.len()) - 1;
      if tunnel.len() <= required_rows {
        increase_tunnel(tunnel, required_rows - tunnel.len());
      }
    }
  }
  return new_rock;
}

fn increase_tunnel(tunnel: &mut Vec<Vec<char>>, rows: usize) {
  for _ in 0..rows {
    tunnel.push(vec!['.'; 7]);
  }
}

pub fn part_one(input: &str) -> Option<u32> {
  let mut max_height = 0;
  let rocks: Vec<Rock> = get_default_rocks();
  let mut tunnel: Vec<Vec<char>> = Vec::new();

  const MAX: usize = 2022;

  let mut falling_rocks_ctr = 0;
  let mut direction_counter = 0;
  let mut rock_shape_counter = 0;
  falling_rocks_ctr += 1;
  let mut rock = rocks[rock_shape_counter % 5].clone();
  rock_shape_counter += 1;
  rock.offset.x += max_height + 3;
  rock.offset.y += (SIZE - rock.shape[0].len()) - 2;

  increase_tunnel(&mut tunnel, 4);

  loop {
    let direction = input.chars().nth(direction_counter % input.len()).unwrap();
    let new_offset = match direction {
      '>' => rock.go_left(&tunnel),
      '<' => rock.go_right(&tunnel),
      _ => {
        println!("wtf....{}", direction);
        None
      }
    };
    match new_offset {
      Some(new_offset) => {
        rock.offset = new_offset;
      }
      None => {
        // Do nothing
      }
    }
    rock = try_going_up(&mut rock, &mut tunnel, &mut falling_rocks_ctr, &mut max_height, &mut rock_shape_counter, &rocks);
    if falling_rocks_ctr == MAX {
      break;
    }
    direction_counter += 1;
  }
  return Some(max_height as u32 + 3);
}

fn print_tunnel(tunnel: &Vec<Vec<char>>, max_height: usize) {
  for row in (0..max_height).rev() {
    for col in (0..tunnel[0].len()).rev() {
      print!("{}", tunnel[row][col]);
    }
    println!();
  }
  println!();
}

pub fn part_two(input: &str) -> Option<u64> {
  let mut max_height = 0;
  let rocks: Vec<Rock> = get_default_rocks();
  let mut tunnel: Vec<Vec<char>> = Vec::new();

  const MAX: usize = 1000000000000;

  let mut falling_rocks_ctr = 0;
  let mut direction_counter = 0;
  let mut rock_shape_counter = 0;

  falling_rocks_ctr += 1;
  let mut rock = rocks[rock_shape_counter % 5].clone();
  rock_shape_counter += 1;
  rock.offset.x += max_height + 3;
  rock.offset.y += (SIZE - rock.shape[0].len()) - 2;

  let mut result: u64 = 0;

  increase_tunnel(&mut tunnel, 4);

  let mut memory: HashMap<(Vec<char>, Vec<Vec<char>>, usize), usize> = HashMap::new();
  let mut max_height_memory: HashMap<usize, usize> = HashMap::new();

  loop {
    let direction = input.chars().nth(direction_counter % input.len()).unwrap();
    let new_offset = match direction {
      '>' => rock.go_left(&tunnel),
      '<' => rock.go_right(&tunnel),
      _ => {
        println!("wtf....{}", direction);
        None
      }
    };
    match new_offset {
      Some(new_offset) => {
        rock.offset = new_offset;
      }
      None => {
        // Do nothing
      }
    }
    let prev_shape: Vec<Vec<char>> = rock.shape.clone();
    rock = try_going_up(&mut rock, &mut tunnel, &mut falling_rocks_ctr, &mut max_height, &mut rock_shape_counter, &rocks);
    if rock.shape != prev_shape {
      max_height_memory.insert(falling_rocks_ctr - 1, max_height);
      let peaks: Vec<char> = (0..SIZE)
        .map(|x| tunnel[max_height][x])
        .collect();
      let rock_shape = rock.shape.clone();
      let key = (peaks, rock_shape, direction_counter % input.len());
      if falling_rocks_ctr >= 250 {
        if memory.contains_key(&key) {
          let prev_rock_ctr = memory[&key];
          let highest_then = max_height_memory.get(&prev_rock_ctr).unwrap();
          let highest_now = max_height;
          let height_diff = highest_now - highest_then;
          let cycle_size = falling_rocks_ctr - 1 - prev_rock_ctr;
          let goal = MAX - prev_rock_ctr;
          let num_cycles = goal / cycle_size;
          let remaining_rocks_in_cycle = goal % cycle_size;
          let remaining_height = max_height_memory.get(&(prev_rock_ctr + remaining_rocks_in_cycle)).unwrap() - highest_then;
          result = (highest_then + remaining_height + (num_cycles * height_diff)) as u64;
          break;
        }
        memory.insert(key.clone(), falling_rocks_ctr - 1);
      }
    }
    direction_counter += 1;
  }
  return Some(result);
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 17);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 17);
    assert_eq!(part_one(&input), Some(3068));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 17);
    assert_eq!(part_two(&input), Some(1514285714288));
  }
}
