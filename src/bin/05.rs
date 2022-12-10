use std::ops::Div;

struct Operation {
    count: usize,
    from: usize,
    to: usize,
}

impl Operation {
    fn parse(op_str: &str) -> Operation {
        let values: Vec<_> = op_str.split_ascii_whitespace().collect();
        let operation: Operation = Operation {
            count: values[1].parse::<usize>().unwrap(),
            from: values[3].parse::<usize>().unwrap() - 1,
            to: values[5].parse::<usize>().unwrap() - 1,
        };
        return operation;
    }
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        for (idx, symbol) in line.char_indices() {
            if symbol.is_ascii_uppercase() {
                let idx_to_insert = idx.div(4);
                while stacks.len() <= idx_to_insert {
                    stacks.push(Vec::new());
                }
                stacks[idx_to_insert].push(symbol);
            }
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }
    //dbg!(&stacks);
    return stacks;
}

pub fn part_one(input: &str) -> Option<String> {
    let mut stacks = parse_stacks(input);
    for line in input.lines() {
        if !line.contains("move") {
            continue;
        }
        let operation = Operation::parse(line);
        for _ in 0..operation.count {
            let last_crate = stacks[operation.from].last().cloned().unwrap();
            stacks[operation.to].push(last_crate);
            let last_idx = stacks[operation.from].len() - 1;
            stacks[operation.from].remove(last_idx);
        }
    }
    let mut message = String::new();
    for stack in stacks.iter() {
        message.push(stack.last().cloned().unwrap());
    }
    return Some(message);
}

pub fn part_two(input: &str) -> Option<String> {
    let mut stacks = parse_stacks(input);
    for line in input.lines() {
        if !line.contains("move") {
            continue;
        }
        let operation = Operation::parse(line);
        let cut_off_idx = stacks[operation.from].len() - operation.count;
        let mut top_crates = stacks[operation.from].split_off(cut_off_idx);
        stacks[operation.to].append(&mut top_crates);
    }
    let mut message = String::new();
    for stack in stacks.iter() {
        message.push(stack.last().cloned().unwrap());
    }
    return Some(message);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_owned()));
    }
}
