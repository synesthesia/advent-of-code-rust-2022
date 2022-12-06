use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    // A X --> Rock -> 1 point
    // B Y --> paper -> 2 point
    // C Z --> scissors -> 3 point
    // wins:
    // s -> p -> r -> s
    // 0 lost
    // 3 draw
    // 6 won

    let win_map = HashMap::from([("p", "s"), ("r", "p"), ("s", "r")]);
    let player1 = HashMap::from([("A", "r"), ("B", "p"), ("C", "s")]);
    let player2 = HashMap::from([("X", "r"), ("Y", "p"), ("Z", "s")]);
    let choice_points = HashMap::from([("r", 1), ("p", 2), ("s", 3)]);

    let games = input.split("\n");

    let mut points = 0;
    for game in games {
        let choices = game.split(" ").map(|c| c.trim()).collect::<Vec<&str>>();

        if choices.len() > 1 {
            let p1_choice = player1.get(choices[0]).unwrap();
            let p2_choice = player2.get(choices[1]).unwrap();

            if p1_choice == p2_choice {
                points += 3; //draw]
            } else if win_map.get(p1_choice).unwrap() == p2_choice {
                points += 6; //win
            }

            let p2_choice_points = choice_points.get(p2_choice).unwrap();
            points += p2_choice_points;
        }
    }
    Some(points)
}

pub fn part_two(input: &str) -> Option<u32> {
       None
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
