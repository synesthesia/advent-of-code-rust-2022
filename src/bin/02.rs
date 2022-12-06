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

    // win_map maps P1 move to winning P2 move
    let win_map = HashMap::from([("p", "s"), ("r", "p"), ("s", "r")]);

    // player1 maps strategy column 1 to P1 move
    let player1 = HashMap::from([("A", "r"), ("B", "p"), ("C", "s")]);

    // player1 maps strategy column 2 to P2 move
    let player2 = HashMap::from([("X", "r"), ("Y", "p"), ("Z", "s")]);

    // choice_points shows per-move scores
    let choice_points = HashMap::from([("r", 1), ("p", 2), ("s", 3)]);

    let games = input.split('\n');

    let mut points = 0;
    for game in games {
        let choices = game.split(' ').map(|c| c.trim()).collect::<Vec<&str>>();

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
    /*
     *   This time round interpret the map as P1 play | Desired Outcome
     *   Again, work out total score from following the guide
     */

    // player1 maps strategy column 1 to P1 move
    let player1 = HashMap::from([("A", "r"), ("B", "p"), ("C", "s")]);

    // win_map maps P1 move to winning P2 move
    let win_map = HashMap::from([("p", "s"), ("r", "p"), ("s", "r")]);

    // lose_map maps P1 move to losing P2 move
    let lose_map = HashMap::from([("s", "p"), ("p", "r"), ("r", "s")]);

    // desired_outcomes maps strategy column 2 to desired outcome code
    let desired_outcomes = HashMap::from([("X", "l"), ("Y", "d"), ("Z", "w")]);

    // shape_points shows per-move scores
    let shape_points = HashMap::from([("r", 1), ("p", 2), ("s", 3)]);

    let games = input.split('\n');

    let mut points = 0;

    for game in games {
        let strategies = game.split(' ').map(|c| c.trim()).collect::<Vec<&str>>();

        if strategies.len() > 1 {
            let p1_shape = *player1.get(strategies[0]).unwrap();
            let outcome = *desired_outcomes.get(strategies[1]).unwrap();

            let mut gamepoints = 0;
            let shapepoints;
            match outcome {
                "l" => {
                    // use P1 shape to find equivalent losing shape for P2
                    let p2_shape = *lose_map.get(p1_shape).unwrap();
                    shapepoints = *shape_points.get(p2_shape).unwrap();
                }
                "d" => {
                    // match P2 shape to P1 shape for draw
                    let p2_shape = p1_shape;
                    gamepoints = 3;
                    shapepoints = *shape_points.get(p2_shape).unwrap();
                }
                "w" => {
                    // use P1 shape to find equivalent winning shape for P2
                    let p2_shape = *win_map.get(p1_shape).unwrap();
                    gamepoints = 6;
                    shapepoints = *shape_points.get(p2_shape).unwrap();
                }
                &_ => continue,
            }
            points += shapepoints;
            points += gamepoints;
        }
    }
    Some(points)
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
