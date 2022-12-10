use std::collections::HashMap;
use std::ops::Div;

pub fn part_one(input: &str) -> Option<u32> {
    const ALPHABETS_ARRAY_SIZE: usize = 26 * 2;
    let mut priority: u32 = 0;
    for line in input.lines() {
        let mut items_freq: [usize; ALPHABETS_ARRAY_SIZE] = [0; ALPHABETS_ARRAY_SIZE];
      let mid = line.len().div(2);
        let first = &line[..mid];
        let second = &line[mid..];
        //dbg!(first, second);
        for item in first.chars() {
            if item.is_ascii_lowercase() {
                items_freq[(item as usize) - 97] += 1;
            } else {
                items_freq[(item as usize) - 65 + 26] += 1;
            }
        }
        for item in second.chars() {
            if item.is_ascii_lowercase() {
                if items_freq[(item as usize) - 97] != 0 {
                    priority += ((item as u32) - 97) + 1;
                    //dbg!(priority, item);
                    items_freq[(item as usize) - 97] = 0;
                }
            } else if item.is_ascii_uppercase() {
                if items_freq[(item as usize) - 65 + 26] != 0 {
                    priority += ((item as u32) - 65 + 26) + 1;
                    //dbg!(priority, item);
                    items_freq[(item as usize) - 65 + 26] = 0;
                }
            }
        }
    }
    return Some(priority);
}

//TODO: Optimise this further
pub fn part_two(input: &str) -> Option<u32> {
  const ALPHABETS_ARRAY_SIZE: usize = 26 * 2;
  let mut priority: usize = 0;
  let mut line_count = 1;
  let mut items_freq: HashMap<_, _> = HashMap::new();
  [0; ALPHABETS_ARRAY_SIZE];
  for line in input.lines() {
    if line_count % 3 == 0 {
      let mut chars_list: Vec<char> = line.to_owned().chars().collect();
      chars_list.sort();
      chars_list.dedup();
      //dbg!(&chars_list, line_count);
      for item in chars_list {
        if item.is_ascii_lowercase() {
          let lower_case_key = (item as usize) - 97;
          if items_freq.contains_key(&lower_case_key) {
            *items_freq.entry(lower_case_key).or_insert(1) += 1;
          } else {
                        items_freq.insert(lower_case_key, 1);
                    }
                } else {
                    let upper_case_key = (item as usize) - 65 + 26;
                    if items_freq.contains_key(&upper_case_key) {
                        *items_freq.entry(upper_case_key).or_insert(1) += 1;
                    } else {
                        items_freq.insert(upper_case_key, 1);
                    }
        }
      }
      for (idx, freq) in &items_freq {
        if *freq == 3 {
          priority += *idx + 1;
        }
        //dbg!(priority);
      }
      line_count = 1;
      items_freq.clear();
      items_freq = HashMap::new();
      [0; ALPHABETS_ARRAY_SIZE];
    } else {
            let mut chars_list: Vec<char> = line.to_owned().chars().collect();
            chars_list.sort();
            chars_list.dedup();
            //dbg!(&chars_list);
            for item in chars_list {
                if item.is_ascii_lowercase() {
                    let lower_case_key = (item as usize) - 97;
                    if items_freq.contains_key(&lower_case_key) {
                        *items_freq.entry(lower_case_key).or_insert(1) += 1;
                    } else {
                        items_freq.insert(lower_case_key, 1);
                    }
                } else {
                    let upper_case_key = (item as usize) - 65 + 26;
                    if items_freq.contains_key(&upper_case_key) {
                        *items_freq.entry(upper_case_key).or_insert(1) += 1;
                    } else {
                        items_freq.insert(upper_case_key, 1);
                    }
                }
            }
            //dbg!(&items_freq);
            line_count += 1;
        }
    }
    return Some(priority as u32);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
