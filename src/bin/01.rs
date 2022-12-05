pub fn part_one(input: &str) -> Option<i32> {
    let elfs = input.split("\n\n");

    let mut max_calories = Some(0);

    for elf in elfs {
        let calories = elf
            .split('\n')
            .map(|item| item.parse::<i32>().unwrap_or(0))
            .reduce(|a, b| a + b);

        if calories.is_some() && calories > max_calories {
            max_calories = calories;
        }
    }
    max_calories
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut calories_per_elf: Vec<i32> = vec![];

    let elfs = input.split("\n\n");

    for elf in elfs {
        let calories = elf
            .split('\n')
            .map(|item| item.parse::<i32>().unwrap_or(0))
            .reduce(|a, b| a + b);

        if calories.is_some() {
            calories_per_elf.push(calories.unwrap_or(0));
        }
    }

    calories_per_elf.sort_by(|a, b| b.cmp(a));
    let result: i32 = calories_per_elf[0..3].iter().sum();
    Some(result)
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
