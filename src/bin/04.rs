struct Section {
  from: usize,
  to: usize,
}

impl Section {
    fn contains(&self, section: &Section) -> bool {
        if self.from >= section.from && self.to <= section.to {
            return true;
        } else if self.from <= section.from && self.to >= section.to {
            return true;
        }
        return false;
    }

    fn overlaps(&self, section: &Section) -> bool {
        if self.from <= section.from && self.to >= section.from {
            return true;
        } else if self.from >= section.from && self.from <= section.to {
            return true;
        }
        return false;
    }

    fn parse(section_str: &str) -> Section {
        let from_to_vec: Vec<&str> = section_str.split('-').collect();
      return Section {
        from: from_to_vec.first().unwrap().parse().unwrap(),
        to: from_to_vec.last().unwrap().parse().unwrap(),
      };
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut similar_sections: u32 = 0;
    for line in input.lines() {
        let pairs: Vec<_> = line.split(',').collect();

        let first_section = Section::parse(pairs.first().unwrap());
        let second_section = Section::parse(pairs.last().unwrap());

        if first_section.contains(&second_section) || second_section.contains(&first_section) {
            similar_sections += 1;
        }
    }
    return Some(similar_sections);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut overlap_sections: u32 = 0;
    for line in input.lines() {
        let pairs: Vec<_> = line.split(',').collect();

        let first_section = Section::parse(pairs.first().unwrap());
        let second_section = Section::parse(pairs.last().unwrap());

        if first_section.overlaps(&second_section) || second_section.overlaps(&first_section) {
            overlap_sections += 1;
        }
    }
    return Some(overlap_sections);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
