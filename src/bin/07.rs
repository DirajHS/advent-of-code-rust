use std::collections::HashMap;

use crate::Cmd::{Cd, Listing};

enum Cmd {
    Cd(String),
    Ls,
    Listing(String), //dir or file
}

fn get_command_type(cmd: &str) -> Cmd {
    let args: Vec<_> = cmd.split_whitespace().collect();
    return match args[0] {
        "$" => {
            if args[1] == "cd" {
                Cd(args[2].to_string())
            } else {
                Cmd::Ls
            }
        }
        _ => Listing(cmd.to_string()),
    };
}

pub fn get_cmd_mapping(input: &str) -> HashMap<String, u64> {
    const PARENT_DIR: &str = "..";
    const ROOT_DIR: &str = "/";

    let mut cwd_stack: Vec<String> = Vec::new();
    let mut dirs_size_mapping: HashMap<String, u64> = HashMap::new();
    for line in input.lines() {
        let cmd = get_command_type(line);
        match cmd {
            Cd(dir) => {
                if dir == PARENT_DIR {
                    cwd_stack.pop();
                } else if dir == ROOT_DIR {
                    cwd_stack.push("/".to_string());
                } else {
                    let cwd_dir_full_path =
                      dir + &cwd_stack.last().unwrap().to_string() + &*"/".to_string();
                    cwd_stack.push(cwd_dir_full_path);
                }
            }
            Listing(listing) => {
                if !listing.starts_with("dir") {
                    let file_size = listing
                      .split_ascii_whitespace()
                      .next()
                      .unwrap()
                      .parse::<u64>()
                      .unwrap();
                    for dir in cwd_stack.iter().rev() {
                        dirs_size_mapping
                          .entry(dir.to_string())
                          .and_modify(|size| *size += file_size)
                          .or_insert(file_size);
                    }
                }
            }
            _ => {
                // No Op
            }
        }
    }
    return dirs_size_mapping;
}

pub fn part_one(input: &str) -> Option<u64> {
    const THRESHOLD: u64 = 100_000;
    let dirs_size_mapping: HashMap<String, u64> = get_cmd_mapping(input);
    let result = dirs_size_mapping
      .values()
      .filter(|&dir_size| dir_size <= &THRESHOLD)
      .sum();

    return Some(result);
}

pub fn part_two(input: &str) -> Option<u64> {
    const DEVICE_MAX: u64 = 70_000_000;
    const UPDATE: u64 = 30_000_000;

    let dirs_size_mapping: HashMap<String, u64> = get_cmd_mapping(input);
    let space_required = UPDATE - (DEVICE_MAX - dirs_size_mapping["/"]);
    let result = dirs_size_mapping
      .values()
      .filter(|&dir_size| dir_size >= &space_required)
      .min()
      .unwrap()
      .to_owned();

    return Some(result);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
