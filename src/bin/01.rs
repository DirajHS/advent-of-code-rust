pub fn part_one(input: &str) -> Option<u64> {
    let mut parsed: Vec<Vec<u64>> = Vec::new();
    let mut calories_to_add = String::from("");
    let mut elf: Vec<u64> = Vec::new();
    for calories in input.lines() {
        if !calories.is_empty() {
            elf.push(calories.parse::<u64>().unwrap());
        } else {
            parsed.push(elf.clone());
            elf.clear();
            calories_to_add.clear();
        }
    }
    parsed.push(elf.clone());
    let max: u64 = parsed
      .iter()
      .map(|cals: &Vec<u64>| cals.iter().sum())
      .max()
      .unwrap();
    println!("{:?}", parsed);
    return Some(max);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut parsed: Vec<Vec<u64>> = Vec::new();
    let mut calories_to_add = String::from("");
    let mut elf: Vec<u64> = Vec::new();
    for calories in input.lines() {
        if !calories.is_empty() {
            elf.push(calories.parse::<u64>().unwrap());
        } else {
            parsed.push(elf.clone());
            elf.clear();
            calories_to_add.clear();
        }
    }
    parsed.push(elf.clone());
    let mut max = parsed
      .iter()
      .map(|cals: &Vec<u64>| cals.iter().sum::<u64>())
      .collect::<Vec<_>>();
    max.sort_by(|a, b| b.cmp(a));
    let val_to_ret: u64 = max.iter().take(3).sum();
    println!("{:?}", max);
    return Some(val_to_ret);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
