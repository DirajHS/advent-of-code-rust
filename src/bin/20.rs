pub fn part_one(input: &str) -> Option<i32> {
    let mut initial_state: Vec<(usize, i32)> = Vec::new();

    for (line_number, line) in input.lines().enumerate() {
        let num = line.parse::<i32>().unwrap();
        initial_state.push((line_number, num));
    }

    let mut rotated_nums: Vec<(usize, i32)> = initial_state.clone();

    for (idx, _) in initial_state.iter().enumerate() {
        let next_num_idx = rotated_nums.iter().position(|state| state.0 == idx).unwrap();
        let num_to_remove = rotated_nums.remove(next_num_idx);
        let new_idx = (next_num_idx as i32 + num_to_remove.1).rem_euclid(rotated_nums.len() as i32);
        rotated_nums.insert(new_idx as usize, num_to_remove);
        //dbg!(&rotated_nums);
    }
    let zero_position: usize = rotated_nums.iter().position(|state| state.1 == 0).unwrap();
    let a = rotated_nums[(1000 + zero_position) % rotated_nums.len()].1;
    let b = rotated_nums[(2000 + zero_position) % rotated_nums.len()].1;
    let c = rotated_nums[(3000 + zero_position) % rotated_nums.len()].1;
    Some(a + b + c)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut initial_state: Vec<(usize, i64)> = Vec::new();
    const DECRYPTION_KEY: i64 = 811589153;

    for (line_number, line) in input.lines().enumerate() {
        let num = line.parse::<i64>().unwrap() * DECRYPTION_KEY;
        initial_state.push((line_number, num));
    }

    let mut rotated_nums: Vec<(usize, i64)> = initial_state.clone();

    const MIXING_COUNT: i32 = 10;
    for _ in 0..MIXING_COUNT {
        for (idx, _) in initial_state.iter().enumerate() {
            let next_num_idx = rotated_nums.iter().position(|state| state.0 == idx).unwrap();
            let num_to_remove = rotated_nums.remove(next_num_idx);
            let new_idx = (next_num_idx as i64 + num_to_remove.1).rem_euclid(rotated_nums.len() as i64);
            rotated_nums.insert(new_idx as usize, num_to_remove);
            //dbg!(&rotated_nums);
        }
    }
    let zero_position: usize = rotated_nums.iter().position(|state| state.1 == 0).unwrap();
    let a = rotated_nums[(1000 + zero_position) % rotated_nums.len()].1;
    let b = rotated_nums[(2000 + zero_position) % rotated_nums.len()].1;
    let c = rotated_nums[(3000 + zero_position) % rotated_nums.len()].1;
    Some(a + b + c)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
