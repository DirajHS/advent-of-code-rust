use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<i32> {
    let mut curr_cycle: usize = 0;
    let cycles: HashMap<usize, i32> = HashMap::from([
        (20, 20),
        (60, 60),
        (100, 100),
        (140, 140),
        (180, 180),
        (220, 220),
    ]);
    let mut cycle_values: Vec<i32> = Vec::new();
    let mut register_value: i32 = 1;

    for line in input.lines() {
        let cmd: Vec<&str> = line.split_ascii_whitespace().collect();
        match cmd[0] {
            "noop" => {
                curr_cycle += 1;
                if cycles.contains_key(&curr_cycle) {
                    cycle_values.push(register_value * cycles[&curr_cycle]);
                }
            }
            "addx" => {
                for i in 1..=2 {
                    curr_cycle += 1;
                    if cycles.contains_key(&curr_cycle) {
                        cycle_values.push(register_value * cycles[&curr_cycle]);
                    }
                    if i == 2 {
                        register_value += cmd[1].parse::<i32>().unwrap();
                    }
                }
            }
            _ => {
                println!("wtf....{}", cmd[0]);
            }
        }
    }
    return Some(cycle_values.iter().sum());
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut curr_cycle: usize = 0;
    let mut register_value: i32 = 1;

    let mut sprite_start: i32 = 0;
    let mut pixel_pos = 0;
    let mut crt: String = String::new();

    for line in input.lines() {
        let cmd: Vec<&str> = line.split_ascii_whitespace().collect();
        match cmd[0] {
            "noop" => {
                curr_cycle += 1;
                if pixel_pos % 40 >= sprite_start && pixel_pos % 40 <= sprite_start + 2 {
                    crt.push('#');
                } else {
                    crt.push(' ');
                }
                pixel_pos += 1;
            }
            "addx" => {
                for i in 1..=2 {
                    curr_cycle += 1;
                    if pixel_pos % 40 >= sprite_start && pixel_pos % 40 <= sprite_start + 2 {
                        crt.push('#');
                    } else {
                        crt.push(' ');
                    }
                    pixel_pos += 1;
                    if i == 2 {
                        register_value += cmd[1].parse::<i32>().unwrap();
                        sprite_start = register_value - 1;
                    }
                }
            }
            _ => {
                println!("wtf....{}", cmd[0]);
            }
        }
    }
    let solution = crt
      .as_bytes()
      .chunks(40)
      .map(|line| std::str::from_utf8(line).unwrap())
      .fold(String::with_capacity(240), |mut s, l| {
          s.push('\n');
          s.push_str(l);
          s
      });
    println!("{}", solution);
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
