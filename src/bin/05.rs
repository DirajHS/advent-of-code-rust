struct Operation {
    count: usize,
    from: usize,
    to: usize
}

impl Operation {
    fn parse(op_str: &str) -> Operation {
        let values: Vec<_> = op_str.split_ascii_whitespace().collect();
        let operation: Operation = Operation {
            count: values[1].parse::<usize>().unwrap(),
            from: values[3].parse::<usize>().unwrap() - 1 ,
            to: values[5].parse::<usize>().unwrap() - 1,
        };
        return operation;
    }
}

fn get_stacks() -> Vec<Vec<char>>{
    /*let stacks: Vec<Vec<char>> = vec![
        vec!['Z', 'N'],
        vec!['M', 'C', 'D'],
        vec!['P'],
    ];*/
    let stacks: Vec<Vec<char>> = vec![
        vec!['D', 'L', 'J', 'R', 'V', 'G', 'F'],
        vec!['T', 'P', 'M', 'B', 'V', 'H', 'J', 'S'],
        vec!['V', 'H', 'M', 'F', 'D', 'G', 'P', 'C'],
        vec!['M', 'D', 'P', 'N', 'G', 'Q'],
        vec!['J', 'L', 'H', 'N', 'F'],
        vec!['N', 'F', 'V', 'Q', 'D', 'G', 'T', 'Z'],
        vec!['F', 'D', 'B', 'L'],
        vec!['M', 'J', 'B', 'S', 'V', 'D', 'N'],
        vec!['G', 'L', 'D'],
    ];
    return stacks;
}
pub fn part_one(input: &str) -> Option<String> {
    let mut stacks = get_stacks();
    for line in input.lines() {
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
    let mut stacks = get_stacks();
    for line in input.lines() {
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


