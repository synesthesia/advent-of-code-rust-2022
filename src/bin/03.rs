use std::collections::HashSet;
pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);

        let set1: HashSet<char> = HashSet::from_iter(first.chars());
        let set2: HashSet<char> = HashSet::from_iter(second.chars());

        let common: Vec<&char> = set1.intersection(&set2).collect();

        // we assume there is one and only one - possible bug
        let duplicate = common[0];

        let priority = match duplicate {
            'a'..='z' => *duplicate as u8 - b'a' + 1,
            'A'..='Z' => *duplicate as u8 - b'A' + 27,
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

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    let mut lines = Vec::new();

    for line in input.lines() {
        lines.push(line.to_string());
        if lines.len() == 3 {
            // bit from here =>
            let sack1: HashSet<char> = HashSet::from_iter(lines[0].chars());
            let sack2: HashSet<char> = HashSet::from_iter(lines[1].chars());
            let sack3: HashSet<char> = HashSet::from_iter(lines[2].chars());
            let intersect1: HashSet<char> = sack1.intersection(&sack2).copied().collect();
            let intersect2: HashSet<&char> = sack3.intersection(&intersect1).collect();
            // => to here was in own function
            // but I couldn't work out how to return the final HAshSet
            // without breaking ownership rules
            intersect2.iter().for_each(|item| total += priority(**item));
            lines.clear();
        }
    }
    // we assume we have an exact multiple of 3 lines in the collection
    // possible bug
    // if not true will need to handle remaining lines
    if total == 0 {
        None
    } else {
        Some(total)
    }
}

fn priority(item: char) -> u32 {
    match item {
        'a'..='z' => (item as u8 - b'a' + 1) as u32,
        'A'..='Z' => (item as u8 - b'A' + 27) as u32,
        _ => 0,
    }
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
        assert_eq!(part_two(&input), Some(70));
    }
}
