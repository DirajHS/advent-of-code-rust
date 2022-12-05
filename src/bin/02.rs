use crate::Result::{Draw, Lose, Win};
use crate::Type::{Paper, Rock, Scissors};

#[derive(Eq, PartialEq)]
enum Type {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Eq, PartialEq)]
enum Result {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

struct RoundPartOne {
    opponent: Type,
    player: Type,
}

struct RoundPartTwo {
    opponent: Type,
    result: Result
}

impl RoundPartOne {
    fn get_score_part_one(&self) -> Option<u64> {
        let mut score = 0;
        if self.player == self.opponent {
            score += 3;
        }
        match self.player {
            Rock => {
                score += Rock as u64;
                if self.opponent == Scissors {
                    score += 6;
                }
            }
            Paper => {
                score += Paper as u64;
                if self.opponent == Rock {
                    score += 6;
                }
            }
            Scissors => {
                score += Scissors as u64;
                if self.opponent == Paper {
                    score += 6;
                }
            }
        }
        return Some(score);
    }
}

impl RoundPartTwo {
    fn get_score_part_two(&self) -> Option<u64> {
        let mut score = 0;
        match self.opponent {
            Rock => {
                match self.result {
                    Win => {
                        score += Win as u64;
                        score += Paper as u64;
                    }
                    Lose => {
                        score += Lose as u64;
                        score += Scissors as u64;
                    }
                    Draw => {
                        score += Draw as u64;
                        score += Rock as u64;
                    }
                }
            }
            Paper => {
                match self.result {
                    Win => {
                        score += Win as u64;
                        score += Scissors as u64;
                    }
                    Lose => {
                        score += Lose as u64;
                        score += Rock as u64;
                    }
                    Draw => {
                        score += Draw as u64;
                        score += Paper as u64;
                    }
                }
            }
            Scissors => {
                match self.result {
                    Win => {
                        score += Win as u64;
                        score += Rock as u64;
                    }
                    Lose => {
                        score += Lose as u64;
                        score += Paper as u64;
                    }
                    Draw => {
                        score += Draw as u64;
                        score += Scissors as u64;
                    }
                }
            }
        }

        return Some(score);
    }
}

fn get_result_type(char: &char) -> Result {
    return if char.eq(&'X') {
        Lose
    } else if char.eq(&'Y') {
        Draw
    } else {
        Win
    }
}

fn get_player_type(char: &char) -> Type {
    return if char.eq(&'X') {
        Rock
    } else if char.eq(&'Y') {
        Paper
    } else {
        Scissors
    }
}

fn get_opponent_type(char: &char) -> Type {
    return if char.eq(&'A') {
        Rock
    } else if char.eq(&'B') {
        Paper
    } else {
        Scissors
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut rounds: Vec<RoundPartOne> = Vec::new();
    for line in input.lines() {
        let current_round = RoundPartOne {
            player: get_player_type(&line.chars().nth(2).unwrap()),
            opponent: get_opponent_type(&line.chars().nth(0).unwrap())
        };
        rounds.push(current_round)
    }
    let mut final_score: u64 = 0;
    for round in rounds {
        final_score += round.get_score_part_one().unwrap();
    }
    return Some(final_score);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut rounds: Vec<RoundPartTwo> = Vec::new();
    for line in input.lines() {
        let current_round = RoundPartTwo {
            result: get_result_type(&line.chars().nth(2).unwrap()),
            opponent: get_opponent_type(&line.chars().nth(0).unwrap())
        };
        rounds.push(current_round)
    }
    let mut final_score: u64 = 0;
    for round in rounds {
        final_score += round.get_score_part_two().unwrap();
    }
    return Some(final_score);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
