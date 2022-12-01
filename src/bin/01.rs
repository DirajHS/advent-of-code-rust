pub fn part_one(input: &str) -> Option<u32> {
    let mut max_calories: u32 = 0;
    let mut calories_so_far: u32 = 0;
    let mut calories_to_add = String::from("");
    for calories in input.lines() {
        if !calories.is_empty() {
            calories_so_far += calories.parse::<u32>().unwrap();
            calories_to_add.push_str(calories);
            calories_to_add.push('+');
        } else {
            if calories_so_far > max_calories {
                max_calories = calories_so_far;
            }
            println!("{} = {}", calories_to_add, calories_so_far);
            calories_so_far = 0;
            calories_to_add.clear();
        }
    }
    return Some(max_calories);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calories_so_far: u32 = 0;
    let mut calories_to_add = Vec::new();
    for calories in input.lines() {
        if !calories.is_empty() {
            calories_so_far += calories.parse::<u32>().unwrap();
        } else {
            calories_to_add.push(calories_so_far);
            calories_so_far = 0;
        }
    }
    calories_to_add.sort_by(|a, b| b.cmp(a));
    let max_calories: u32 = calories_to_add[0..=2].iter().sum();
    return Some(max_calories);
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
