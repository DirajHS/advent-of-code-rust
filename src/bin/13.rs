use std::cmp::{min, Ordering};

use serde::Deserialize;

use crate::Signal::{List, Value};

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(untagged)]
pub enum Signal {
    List(Vec<Signal>),
    Value(i32),
}

pub fn check_pair(left: &Signal, right: &Signal) -> Option<bool> {
    match (left, right) {
        (Value(left), Value(right)) => match left.cmp(right) {
            Ordering::Less => Some(true),
            Ordering::Equal => None,
            Ordering::Greater => Some(false),
        }
        (List(left), List(right)) => {
            if left.len() == 0 && right.len() > 0 {
                Some(true);
            }
            if left.len() > 0 && right.len() == 0 {
                Some(false);
            }
            let min_size = min(left.len(), right.len());
            let it = (0..min_size)
              .fold(None, |acc, idx| acc.or(check_pair(&left[idx], &right[idx])));
            it.or_else(|| match left.len().cmp(&right.len()) {
                Ordering::Less => Some(true),
                Ordering::Greater => Some(false),
                Ordering::Equal => None
            })
        }
        (left @ Value(_), right @ List(_)) => check_pair(&List(vec![left.clone()]), right),
        (left @ List(_), right @ Value(_)) => check_pair(left, &List(vec![right.clone()]))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let result: usize = input.split("\n\n")
      .map(|pair_str| {
          let (left, right) = pair_str.split_once('\n').unwrap();
          let left: Signal = serde_json::from_str(left).unwrap();
          let right: Signal = serde_json::from_str(right).unwrap();
          /*println!("{:?}", left);
          println!("{:?}", right);
          println!();*/
          None.or(check_pair(&left, &right))
      })
      .enumerate()
      .filter_map(|(idx, res)| match res {
          Some(true) => Some(idx + 1),
          _ => None
      })
      .sum();
    return Some(result as u32);
}

pub fn part_two(input: &str) -> Option<u32> {
    let divider_packet_1: Signal = serde_json::from_str("[[2]]").unwrap();
    let divider_packet_2: Signal = serde_json::from_str("[[6]]").unwrap();
    let mut combined_input: Vec<_> = input.lines()
      .filter(|&line| !line.is_empty())
      .map(|line| serde_json::from_str(&line).unwrap())
      .collect();
    combined_input.push(divider_packet_1.clone());
    combined_input.push(divider_packet_2.clone());

    combined_input.sort_by(|left, right| match check_pair(left, right) {
        Some(true) => Ordering::Less,
        _ => Ordering::Greater
    });
    //println!("{:?}", combined_input);
    let result: u32 = combined_input.iter()
      .enumerate()
      .filter_map(|(idx, signal)| {
          if signal == &divider_packet_1 || signal == &divider_packet_2 {
              Some(idx as u32 + 1)
          } else {
              None
          }
      })
      .product();
    return Some(result);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
