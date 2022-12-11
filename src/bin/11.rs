type MonkeyId = u32;

#[derive(Clone)]
pub enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    hypothesis_val: u64,
    next_monkey_true: MonkeyId,
    next_monkey_false: MonkeyId,
    inspection_count: u64,
}

impl Monkey {
    fn inspected_item(&mut self) {
        self.inspection_count += 1;
    }
}

impl Clone for Monkey {
    fn clone(&self) -> Self {
        Monkey {
            items: self.items.clone(),
            operation: self.operation.clone(),
            hypothesis_val: *&self.hypothesis_val,
            next_monkey_true: self.next_monkey_true,
            next_monkey_false: self.next_monkey_false,
            inspection_count: self.inspection_count,
        }
    }
}

impl Default for Monkey {
    fn default() -> Self {
        Monkey {
            items: Vec::new(),
            operation: Operation::Mul(0),
            hypothesis_val: 0,
            next_monkey_true: 0,
            next_monkey_false: 0,
            inspection_count: 0,
        }
    }
}

fn get_hypothesis_val(line: &str) -> MonkeyId {
    return line.split("monkey").collect::<Vec<&str>>().last().unwrap().trim().parse().unwrap();
}

fn get_test_val(line: &str) -> u64 {
    return line.split("by").collect::<Vec<&str>>().last()
      .unwrap().trim().parse::<u64>().unwrap();
}

fn get_operation(line: &str) -> Option<Operation> {
    let rhs: Vec<&str> = line.split_at(line.find("=").unwrap())
      .1
      .split_ascii_whitespace()
      .collect::<Vec<&str>>();
    match rhs[2] {
        "*" => {
            return match rhs[3].parse::<u64>() {
                Ok(num) => {
                    Some(Operation::Mul(num))
                }
                Err(_err) => {
                    Some(Operation::Square)
                }
            }
        }
        "+" => {
            return Some(Operation::Add(rhs[3].parse::<u64>().unwrap()));
        }
        _ => {
            println!("wtf....{}", rhs[1]);
        }
    }
    return None;
}

fn get_items(line: &str) -> Vec<u64> {
    let mut items: Vec<u64> = Vec::new();
    items = line.
      trim_start()
      .split_at(line.find(':').unwrap())
      .1
      .split(',')
      .collect::<Vec<&str>>()
      .iter()
      .map(|&str_num|
        str_num.trim().parse::<u64>()
      )
      .collect::<Result<Vec<u64>, _>>()
      .unwrap();
    return items;
}

pub fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey = Monkey::default();
    for line in input.lines() {
        match line.trim_start().split_ascii_whitespace().next() {
            _ if line.contains("Starting items") => {
                let items = get_items(line);
                monkey.items = items;
            }
            _ if line.contains("Operation") => {
                let operation = get_operation(line);
                monkey.operation = operation.unwrap();
            }
            _ if line.contains("Test") => {
                let test_val: u64 = get_test_val(line);
                monkey.hypothesis_val = test_val;
            }
            _ if line.contains("If true") => {
                let test_val = get_hypothesis_val(line);
                monkey.next_monkey_true = test_val;
            }
            _ if line.contains("If false") => {
                let test_val = get_hypothesis_val(line);
                monkey.next_monkey_false = test_val;
            }
            _ if line.contains("Monkey") => {
                monkey = Monkey::default();
            }
            _ => {
                monkeys.push(monkey.clone());
            }
        }
    }
    monkeys.push(monkey.clone());
    return monkeys;
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkeys = parse_input(input);
    for _ in 0..20 {
        for j in 0..monkeys.len() {
            let monkey = &mut monkeys[j].clone();
            for item in monkey.items.clone() {
                let mut worry_level = match monkey.operation {
                    Operation::Add(num) => item + num,
                    Operation::Mul(num) => item * num,
                    Operation::Square => item * item
                };
                worry_level = worry_level / 3;
                if worry_level % monkey.hypothesis_val == 0 {
                    monkeys[monkey.next_monkey_true as usize].items.push(worry_level);
                } else {
                    monkeys[monkey.next_monkey_false as usize].items.push(worry_level);
                }
                monkey.items.remove(0);
                monkey.inspected_item();
            }
            monkeys[j] = monkey.clone();
        }
    }
    let mut sorted_inspection_count = monkeys.iter()
      .map(|monkey| monkey.inspection_count)
      .collect::<Vec<u64>>();

    sorted_inspection_count.sort();
    return Some(sorted_inspection_count[sorted_inspection_count.len() - 2..].iter().product::<u64>());
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys = parse_input(input);
    let modulus: u64 = monkeys.iter().map(|monkey| monkey.hypothesis_val).product();
    for _ in 0..10_000 {
        for j in 0..monkeys.len() {
            let monkey = &mut monkeys[j].clone();
            for item in monkey.items.clone() {
                let worry_level: u64 = match monkey.operation {
                    Operation::Add(num) => (item + num) % modulus,
                    Operation::Mul(num) => (item * num) % modulus,
                    Operation::Square => (item * item) % modulus
                };
                if worry_level % monkey.hypothesis_val == 0 {
                    monkeys[monkey.next_monkey_true as usize].items.push(worry_level);
                } else {
                    monkeys[monkey.next_monkey_false as usize].items.push(worry_level);
                }
                monkey.items.remove(0);
                monkey.inspected_item();
            }
            monkeys[j] = monkey.clone();
        }
    }
    let mut sorted_inspection_count = monkeys.iter()
      .map(|monkey| monkey.inspection_count)
      .collect::<Vec<u64>>();

    sorted_inspection_count.sort();
    return Some(sorted_inspection_count[sorted_inspection_count.len() - 2..].iter().product::<u64>());
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
