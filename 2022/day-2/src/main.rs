use std::fs;

mod game {
    #[derive(PartialEq)]
    struct Shape {
        points: u32,
        option: Options,
        beats: Options,
        loses: Options,
    }

    #[derive(PartialEq)]
    enum Options {
        Rock,
        Paper,
        Scissors,
    }

    const ROCK: Shape = Shape {
        points: 1,
        option: Options::Rock,
        beats: Options::Scissors,
        loses: Options::Paper,
    };

    const PAPER: Shape = Shape {
        points: 2,
        option: Options::Paper,
        beats: Options::Rock,
        loses: Options::Scissors,
    };

    const SCISSORS: Shape = Shape {
        points: 3,
        option: Options::Scissors,
        beats: Options::Paper,
        loses: Options::Rock,
    };

    fn option_to_shape(option: &Options) -> Shape {
        match option {
            Options::Rock => ROCK,
            Options::Paper => PAPER,
            Options::Scissors => SCISSORS,
        }
    }

    fn calculate_points(opponent: &Shape, player: &Shape) -> u32 {
        if opponent.option == player.beats {
            return player.points + 6;
        } else if opponent.option == player.option {
            return player.points + 3;
        }

        return player.points;
    }

    pub fn strategy_one(hands: &Vec<&str>) -> u32 {
        hands
            .iter()
            .map(|hand| {
                let rolls = hand.split(" ").collect::<Vec<&str>>();

                let opponent = match rolls.get(0) {
                    Some(&"A") => option_to_shape(&Options::Rock),
                    Some(&"B") => option_to_shape(&Options::Paper),
                    Some(&"C") => option_to_shape(&Options::Scissors),
                    _ => panic!("Invalid hand"),
                };
                let player = match rolls.get(1) {
                    Some(&"X") => option_to_shape(&Options::Rock),
                    Some(&"Y") => option_to_shape(&Options::Paper),
                    Some(&"Z") => option_to_shape(&Options::Scissors),
                    _ => panic!("Invalid hand"),
                };

                return calculate_points(&opponent, &player);
            })
            .sum()
    }

    pub fn strategy_two(hands: &Vec<&str>) -> u32 {
        hands
            .iter()
            .map(|hand| {
                let rolls = hand.split(" ").collect::<Vec<&str>>();

                let opponent = match rolls.get(0) {
                    Some(&"A") => ROCK,
                    Some(&"B") => PAPER,
                    Some(&"C") => SCISSORS,
                    _ => panic!("Invalid hand"),
                };
                let player = match rolls.get(1) {
                    Some(&"X") => option_to_shape(&opponent.beats),
                    Some(&"Y") => option_to_shape(&opponent.option),
                    Some(&"Z") => option_to_shape(&opponent.loses),
                    _ => panic!("Invalid hand"),
                };

                return calculate_points(&opponent, &player);
            })
            .sum()
    }
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    let hands = data.split("\n").collect::<Vec<&str>>();

    let total_1 = game::strategy_one(&hands);

    println!("Total 1: {}", total_1);

    let total_2 = game::strategy_two(&hands);

    println!("Total 2: {}", total_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_one() {
        let hands = vec!["A Y", "B X", "C Z"];
        let total = game::strategy_one(&hands);

        assert_eq!(total, 15);
    }

    #[test]
    fn test_strategy_two() {
        let hands = vec!["A Y", "B X", "C Z"];
        let total = game::strategy_two(&hands);

        assert_eq!(total, 12);
    }
}
