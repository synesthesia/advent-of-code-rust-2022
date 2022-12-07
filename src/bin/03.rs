pub fn part_one(input: &str) -> Option<u32> {
    use std::collections::HashSet;

    let mut total: u32 = 0;

    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);

        let set1: HashSet<char> = HashSet::from_iter(first.chars());
        let set2: HashSet<char> = HashSet::from_iter(second.chars());

        let common: Vec<&char> = set1.intersection(&set2).collect();

        // we assume there is one and only one - possible bug
        let duplicate = common[0];

        let priority = match duplicate {
            'a'..='z' => *duplicate as u8 - b'a'  + 1,
            'A'..='Z' => *duplicate as u8 - b'A'  + 27,
            _ => 0,
        } as u32;

        total += priority;
    }
    if total == 0 {
        None
    } else {
        Some(total)
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
