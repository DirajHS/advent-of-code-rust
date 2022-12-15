use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Pos {
  x: i32,
  y: i32,
}

#[derive(Clone)]
struct Pair {
  sensor: Pos,
  beacon: Pos,
  distance: i32,
}

impl Default for Pair {
  fn default() -> Self {
    Pair {
      sensor: Pos { x: 0, y: 0 },
      beacon: Pos { x: 0, y: 0 },
      distance: 0,
    }
  }
}

fn get_distance(start: &Pos, end: &Pos) -> u32 {
  return start.x.abs_diff(end.x) + start.y.abs_diff(end.y);
}

fn parse_x(input: &str) -> i32 {
  return input.split(",").next().unwrap()
    .split("x").last().unwrap()
    .split("=").last().unwrap().trim().parse::<i32>().unwrap();
}

fn parse_y(input: &str) -> i32 {
  return input.split(",").last().unwrap()
    .split("y").last().unwrap()
    .split("=").last().unwrap().trim().parse::<i32>().unwrap();
}

fn parse_input(input: &str) -> Vec<Pair> {
  let mut map: Vec<_> = Vec::new();
  for line in input.lines() {
    let mut pair = Pair::default();
    line.split(":")
      .into_iter()
      .for_each(|input| {
        if input.contains("Sensor") {
          pair.sensor = Pos {
            x: parse_x(&input),
            y: parse_y(&input),
          };
        }
        if input.contains("beacon") {
          pair.beacon = Pos {
            x: parse_x(&input),
            y: parse_y(&input),
          };
        }
      });
    pair.distance = get_distance(&pair.sensor, &pair.beacon) as i32;
    map.push(pair);
  }
  return map;
}

pub fn part_one(input: &str) -> Option<u32> {
  let mapping: Vec<Pair> = parse_input(input);
  let target_row = if cfg!(test) {
    10
  } else {
    20_00_000
  };
  let mut empty_places: HashSet<i32> = HashSet::new();

  for pos in mapping.clone() {
    let target_sensor_distance: i32 = pos.sensor.y.abs_diff(target_row) as i32;
    if target_sensor_distance > pos.distance {
      continue;
    }
    let cols_diff = pos.distance - target_sensor_distance;
    let start = pos.sensor.x - cols_diff;
    let end = pos.sensor.x + cols_diff;
    for i in start..=end {
      let sensor_beacon_pos_vec: Vec<Pair> = mapping.clone().into_iter()
        .filter(|pos| {
          pos.beacon.x == i && pos.beacon.y == target_row
        })
        .collect();
      if sensor_beacon_pos_vec.is_empty() {
        empty_places.insert(i);
      }
    }
  }
  let res = empty_places.len() as u32;
  //dbg!(output);
  return Some(res);
}


pub fn part_two(input: &str) -> Option<u64> {
  let mapping: Vec<Pair> = parse_input(input);
  let target_row = if cfg!(test) {
    20
  } else {
    40_00_000
  };
  let mut final_x: u64 = 0;
  let mut final_y: u64 = 0;

  'solution: for pair in mapping.clone() {
    for distance in 0..=pair.distance + 1 {
      let diamond_corners = vec![
        Pos { x: pair.sensor.x - pair.distance - 1 + distance, y: pair.sensor.y - distance },
        Pos { x: pair.sensor.x + pair.distance + 1 - distance, y: pair.sensor.y - distance },
        Pos { x: pair.sensor.x - pair.distance - 1 + distance, y: pair.sensor.y + distance },
        Pos { x: pair.sensor.x + pair.distance + 1 - distance, y: pair.sensor.y + distance },
      ];
      'corner_check: for next_corner in diamond_corners.iter() {
        if next_corner.x < 0 || next_corner.x > target_row || next_corner.y < 0 || next_corner.y > target_row {
          continue;
        }

        for nested_pair in mapping.clone() {
          let distance = get_distance(&nested_pair.sensor, &next_corner);
          if distance <= nested_pair.distance as u32 {
            continue 'corner_check;
          }
        }
        final_x = next_corner.x as u64;
        final_y = next_corner.y as u64;
        break 'solution;
      }
    }
  }
  //println!("final x: {}, final y: {}", final_x, final_y);
  return Some((final_x * 40_00_000) + final_y);
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 15);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 15);
    assert_eq!(part_one(&input), Some(26));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 15);
    assert_eq!(part_two(&input), Some(56000011));
  }
}
