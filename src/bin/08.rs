fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut forest: Vec<Vec<char>> = vec![vec![]; input.lines().count()];
    let mut line_count = 0;
    for line in input.lines() {
        forest[line_count] = vec![' '; input.lines().count()];
        for (idx, size) in line.char_indices() {
            forest[line_count][idx] = size;
        }
        line_count += 1;
    }
    return forest;
}

fn is_visible_left(
    tree_size: char,
    row_idx: usize,
    col_idx: usize,
    forest: &Vec<Vec<char>>,
) -> bool {
    let mut i: i32 = (col_idx - 1) as i32;
    while i >= 0 {
        if tree_size <= forest[row_idx][i as usize] {
            return false;
        }
        i -= 1;
    }
    return true;
}

fn is_visible_right(
    tree_size: char,
    row_idx: usize,
    col_idx: usize,
    forest: &Vec<Vec<char>>,
) -> bool {
    let cols = forest[0].len();

    for i in col_idx + 1..cols {
        if tree_size <= forest[row_idx][i] {
            return false;
        }
    }
    return true;
}

fn is_visible_top(
    tree_size: char,
    row_idx: usize,
    col_idx: usize,
    forest: &Vec<Vec<char>>,
) -> bool {
    let mut i: i32 = (row_idx - 1) as i32;

    while i >= 0 {
        if tree_size <= forest[i as usize][col_idx] {
            return false;
        }
        i -= 1;
    }
    return true;
}

fn is_visible_down(
    tree_size: char,
    row_idx: usize,
    col_idx: usize,
    forest: &Vec<Vec<char>>,
) -> bool {
    let rows = forest.len();
    for i in row_idx + 1..rows {
        if tree_size <= forest[i][col_idx] {
            return false;
        }
    }
    return true;
}

fn get_visible_left(
    tree_size: char,
    row_idx: usize,
    col_idx: usize,
    forest: &Vec<Vec<char>>,
) -> Option<i32> {
    let mut visibility = 0;
    let mut i: i32 = (col_idx - 1) as i32;
    while i >= 0 {
        if tree_size > forest[row_idx][i as usize] {
            visibility += 1;
        } else if tree_size == forest[row_idx][i as usize] {
            visibility += 1;
            return Some(visibility);
        } else {
            visibility += 1;
            return Some(visibility);
        }
        i -= 1;
    }
    return Some(visibility);
}

fn get_visible_right(
    tree_size: char,
    row_idx: usize,
    col_idx: usize,
    forest: &Vec<Vec<char>>,
) -> Option<i32> {
    let cols = forest[0].len();
    let mut visibility = 0;
    for i in col_idx + 1..cols {
        if tree_size > forest[row_idx][i] {
            visibility += 1;
        } else if tree_size == forest[row_idx][i] {
            visibility += 1;
            return Some(visibility);
        } else {
            visibility += 1;
            return Some(visibility);
        }
    }
    return Some(visibility);
}

fn get_visible_top(
    tree_size: char,
    row_idx: usize,
    col_idx: usize,
    forest: &Vec<Vec<char>>,
) -> Option<i32> {
    let mut i: i32 = (row_idx - 1) as i32;
    let mut visibility = 0;
    while i >= 0 {
        if tree_size > forest[i as usize][col_idx] {
            visibility += 1;
        } else if tree_size == forest[i as usize][col_idx] {
            visibility += 1;
            return Some(visibility);
        } else {
            visibility += 1;
            return Some(visibility);
        }
        i -= 1;
    }
    return Some(visibility);
}

fn get_visible_down(
    tree_size: char,
    row_idx: usize,
    col_idx: usize,
    forest: &Vec<Vec<char>>,
) -> Option<i32> {
    let rows = forest.len();
    let mut visibility = 0;
    for i in row_idx + 1..rows {
        if tree_size > forest[i][col_idx] {
            visibility += 1;
        } else if tree_size == forest[i][col_idx] {
            visibility += 1;
            return Some(visibility);
        } else {
            visibility += 1;
            return Some(visibility);
        }
    }
    return Some(visibility);
}

pub fn part_one(input: &str) -> Option<u32> {
    let forest: Vec<Vec<char>> = parse_input(input);
    let rows = forest.len();
    let cols = forest[0].len();
    let mut visible_count: u32 = ((forest.len() * 2) + (forest[0].len() * 2) - 4) as u32;
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            let tree_size = forest[i][j];
            if is_visible_left(tree_size, i, j, &forest)
              || is_visible_right(tree_size, i, j, &forest)
              || is_visible_top(tree_size, i, j, &forest)
              || is_visible_down(tree_size, i, j, &forest)
            {
                visible_count += 1;
            }
        }
    }
    return Some(visible_count);
}

pub fn part_two(input: &str) -> Option<u32> {
    let forest: Vec<Vec<char>> = parse_input(input);
    let rows = forest.len();
    let cols = forest[0].len();
    let mut max_visibility = 0;
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            let tree_size = forest[i][j];
            //dbg!(tree_size);

            let left = get_visible_left(tree_size, i, j, &forest).unwrap();
            let right = get_visible_right(tree_size, i, j, &forest).unwrap();
            let top = get_visible_top(tree_size, i, j, &forest).unwrap();
            let down = get_visible_down(tree_size, i, j, &forest).unwrap();
            let total_visibility = left * right * top * down;
            //dbg!(total_visibility, left, right, top, down);
            if total_visibility > max_visibility {
                max_visibility = total_visibility;
            }
        }
    }
    return Some(max_visibility as u32);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
