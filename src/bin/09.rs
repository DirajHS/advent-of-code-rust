use std::collections::HashSet;
use std::vec;

#[derive(Clone, Eq, PartialEq, Hash, Copy, Default)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn update(&mut self, row: i32, col: i32) {
        self.row = row;
        self.col = col;
    }
}

struct Move {
    new_row_idx: i32,
    new_col_idx: i32,
}

fn is_row_adj(tail_row: i32, tail_col: i32, head_row: i32, head_col: i32) -> bool {
    // Adj. in same row
    if tail_row == head_row && (head_col == tail_col - 1 || head_col == tail_col + 1) {
        return true;
    }
    return false;
}

fn is_col_adj(tail_row: i32, tail_col: i32, head_row: i32, head_col: i32) -> bool {
    // Adj. in same col
    if tail_col == head_col && (head_row == tail_row - 1 || head_row == tail_row + 1) {
        return true;
    }
    return false;
}

fn is_tail_adjacent(tail_row: i32, tail_col: i32, head_row: i32, head_col: i32) -> bool {
    if tail_row == head_row && tail_col == head_col {
        return true;
    }

    if is_row_adj(tail_row, tail_col, head_row, head_col) {
        return true;
    }

    if is_col_adj(tail_row, tail_col, head_row, head_col) {
        return true;
    }

    // Adj. diagonally right up
    if tail_row - 1 == head_row && tail_col + 1 == head_col {
        return true;
    }
    // Adj. diagonally right down
    if tail_row + 1 == head_row && tail_col + 1 == head_col {
        return true;
    }
    // Adj. diagonally left up
    if tail_row - 1 == head_row && tail_col - 1 == head_col {
        return true;
    }
    // Adj. diagonally left down
    if tail_row + 1 == head_row && tail_col - 1 == head_col {
        return true;
    }
    return false;
}

fn get_new_tail_pos(tail_row: i32, tail_col: i32, head_row: i32, head_col: i32) -> Option<Move> {
    //Same row
    if tail_row == head_row {
        return if tail_col < head_col {
            Some(Move {
                new_row_idx: tail_row,
                new_col_idx: tail_col + 1,
            })
        } else {
            Some(Move {
                new_row_idx: tail_row,
                new_col_idx: tail_col - 1,
            })
        };
    }

    //Same col
    if tail_col == head_col {
        return if tail_row < head_row {
            Some(Move {
                new_row_idx: tail_row + 1,
                new_col_idx: tail_col,
            })
        } else {
            Some(Move {
                new_row_idx: tail_row - 1,
                new_col_idx: tail_col,
            })
        };
    }

    //Diagonally right
    if tail_col > head_col {
        return if tail_row < head_row {
            Some(Move {
                new_row_idx: tail_row + 1,
                new_col_idx: tail_col - 1,
            })
        } else {
            Some(Move {
                new_row_idx: tail_row - 1,
                new_col_idx: tail_col - 1,
            })
        };
    }

    //Diagonally left
    if tail_col < head_col {
        return if tail_row < head_row {
            Some(Move {
                new_row_idx: tail_row + 1,
                new_col_idx: tail_col + 1,
            })
        } else {
            Some(Move {
                new_row_idx: tail_row - 1,
                new_col_idx: tail_col + 1,
            })
        };
    }
    return None;
}

fn check_and_update_tail(tail: &Position, head: &Position) -> Option<Position> {
    if !is_tail_adjacent(tail.row, tail.col, head.row, head.col) {
        let new_pos = get_new_tail_pos(tail.row, tail.col, head.row, head.col);
        match new_pos {
            Some(pos) => {
                let new_pos = Position {
                    row: pos.new_row_idx,
                    col: pos.new_col_idx,
                };
                return Some(new_pos);
            }
            None => {
                println!("wtf!!!")
            }
        }
    }
    None
}

fn solve(tail: &Position, head: &mut Position, direction: &str) -> Option<Position> {
    match direction {
        "L" => {
            head.update(head.row, head.col - 1);
            return check_and_update_tail(&tail, &head);
        }
        "R" => {
            head.update(head.row, head.col + 1);
            return check_and_update_tail(&tail, &head);
        }
        "U" => {
            head.update(head.row - 1, head.col);
            return check_and_update_tail(&tail, &head);
        }
        "D" => {
            head.update(head.row + 1, head.col);
            return check_and_update_tail(&tail, &head);
        }
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut head = Position::default();
    let mut tail = Position::default();
    let mut visited_set: HashSet<Position> = HashSet::new();
    visited_set.insert(Position::default());

    for line in input.lines() {
        let movement: Vec<&str> = line.split_ascii_whitespace().collect();
        let moves = movement[1].parse::<usize>().unwrap();
        for _ in 0..moves {
            let new_tail_pos = solve(&tail, &mut head, movement.first().unwrap());
            match new_tail_pos {
                Some(pos) => {
                    visited_set.insert(pos);
                    tail = pos;
                }
                None => {}
            }
        }
    }
    return Some(visited_set.len() as u32);
}

fn update_tail(mut tail: &mut Position, head: &Position) {
    let dx = head.row - tail.row;
    let dy = head.col - tail.col;
    if (dx.abs() + dy.abs() > 1) && (dx.abs() > 1 || dy.abs() > 1) {
        tail.row += dx.signum();
        tail.col += dy.signum();
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut head = Position::default();
    let mut tail: Vec<Position> = vec![Position::default(); 9];
    let mut visited_set: HashSet<Position> = HashSet::new();
    visited_set.insert(Position::default());
    for line in input.lines() {
        let movement: Vec<&str> = line.split_ascii_whitespace().collect();
        let moves = movement[1].parse::<usize>().unwrap();
        for _ in 0..moves {
            match movement[0] {
                "L" => {
                    head.col -= 1;
                }
                "R" => {
                    head.col += 1;
                }
                "U" => {
                    head.row -= 1;
                }
                "D" => {
                    head.row += 1;
                }
                _ => {
                    println!("wtf....{}", movement[0]);
                }
            }
            update_tail(&mut tail[0], &head);

            for i in 1..9 {
                let (new_head, new_tail) = tail.split_at_mut(i);
                update_tail(&mut new_tail[0], &new_head[i - 1]);
            }
            visited_set.insert(*&tail[8]);
        }
    }
    return Some(visited_set.len() as u32);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
