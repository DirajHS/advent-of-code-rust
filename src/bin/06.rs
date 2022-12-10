use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut set: HashSet<char> = HashSet::new();
    let mut substr: String = String::new();

    for (idx, c) in input.char_indices() {
        let previous_len = set.len();

        set.insert(c);
        substr.push(c);
        if set.len() == 4 {
            return Some((idx + 1) as u32);
        }
        if previous_len == set.len() {
            for _ in 0..substr.len() - 1 {
                //dbg!(&substr);
                let ch = substr.chars().nth(0).unwrap();
                substr.remove(0);
                if ch == c {
                    break;
                } else {
                    set.remove(&ch);
                }
            }
        }
        //dbg!(&substr, &set);
    }
    return None;
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut set: HashSet<char> = HashSet::new();
    let mut substr: String = String::new();

    for (idx, c) in input.char_indices() {
        let previous_len = set.len();

        set.insert(c);
        substr.push(c);
        if set.len() == 14 {
            return Some((idx + 1) as u32);
        }
        if previous_len == set.len() {
            for _ in 0..substr.len() - 1 {
                //dbg!(&substr);
                let ch = substr.chars().nth(0).unwrap();
                substr.remove(0);
                if ch == c {
                    break;
                } else {
                    set.remove(&ch);
                }
            }
        }
        //dbg!(&substr, &set);
    }
    return None;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(26));
    }
}
