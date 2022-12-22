use std::collections::{BTreeSet, HashSet};
use std::collections::vec_deque::VecDeque;

#[derive(Clone, Copy, Debug, Default, Ord, PartialOrd, PartialEq, Eq, Hash)]
struct Pos3D {
    x: isize,
    y: isize,
    z: isize,
}

impl Pos3D {
    fn add_point(&self, point: &Pos3D) -> Pos3D {
        let mut new_point = self.clone();
        new_point.x += point.x;
        new_point.y += point.y;
        new_point.z += point.z;
        return new_point;
    }
}

const DIRECTIONS: [Pos3D; 6] = [
    Pos3D { x: 0, y: 0, z: 1 }, Pos3D { x: 0, y: 0, z: -1 },
    Pos3D { x: 1, y: 0, z: 0 }, Pos3D { x: -1, y: 0, z: 0 },
    Pos3D { x: 0, y: 1, z: 0 }, Pos3D { x: 0, y: -1, z: 0 },
];

pub fn part_one(input: &str) -> Option<u32> {
    let points: HashSet<Pos3D> = input.lines()
      .into_iter()
      .map(|line| {
          let points: Vec<isize> =
            line.split(",")
              .map(|num_str| {
                  num_str.parse().unwrap()
              })
              .collect();
          Pos3D {
              x: points[0],
              y: points[1],
              z: points[2],
          }
      })
      .collect();

    let mut result: u32 = (6 * points.len()) as u32;
    for point in &points {
        for direction in DIRECTIONS {
            let new_point = point.add_point(&direction);
            if points.contains(&new_point) {
                result -= 1;
            }
        }
    }
    return Some(result);
}

fn can_reach_out(point: Pos3D, goes_in: &mut HashSet<Pos3D>, goes_out: &mut HashSet<Pos3D>, points: &HashSet<Pos3D>) -> bool {
    if goes_out.contains(&point) {
        return true;
    }
    if goes_in.contains(&point) {
        return false;
    }
    let mut bfs_queue: VecDeque<Pos3D> = VecDeque::new();
    let mut visited: HashSet<Pos3D> = HashSet::new();

    bfs_queue.push_back(point);
    while !bfs_queue.is_empty() {
        let next_point = bfs_queue.pop_front().unwrap();
        if points.contains(&next_point) {
            continue;
        }
        if visited.contains(&next_point) {
            continue;
        }
        visited.insert(next_point);
        if visited.len() > 7000 {
            for point in visited.clone() {
                goes_out.insert(point);
            }
            return true;
        }
        for neighbor in DIRECTIONS {
            bfs_queue.push_back(next_point.add_point(&neighbor));
        }
    }
    for point in visited {
        goes_in.insert(point);
    }
    return false;
}

pub fn part_two(input: &str) -> Option<u32> {
    let points: HashSet<Pos3D> = input.lines()
      .into_iter()
      .map(|line| {
          let points: Vec<isize> =
            line.split(",")
              .map(|num_str| {
                  num_str.parse().unwrap()
              })
              .collect();
          Pos3D {
              x: points[0],
              y: points[1],
              z: points[2],
          }
      })
      .collect();

    let mut result: u32 = 0;

    let mut goes_in: HashSet<Pos3D> = HashSet::new();
    let mut goes_out: HashSet<Pos3D> = HashSet::new();

    for point in points.clone() {
        for neighbor in DIRECTIONS {
            if can_reach_out(point.add_point(&neighbor), &mut goes_in, &mut goes_out, &points) {
                result += 1;
            }
        }
    }
    return Some(result);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
