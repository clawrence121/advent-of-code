use std::fs;

struct Hand {
    red: i32,
    green: i32,
    blue: i32,
}

impl Hand {
    pub fn from_string(input_string: &str) -> Hand {
        // input_string: "8 green, 6 blue, 20 red"
        let colors = input_string.split(", ");

        let mut hand = Hand {
            red: 0,
            green: 0,
            blue: 0,
        };

        colors.for_each(|color| match color.split(" ").last().unwrap() {
            "red" => hand.red = color.split(" ").next().unwrap_or("0").parse().unwrap(),
            "green" => hand.green = color.split(" ").next().unwrap_or("0").parse().unwrap(),
            "blue" => hand.blue = color.split(" ").next().unwrap_or("0").parse().unwrap(),
            &_ => {}
        });

        hand
    }
    pub fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: i32,
    hands: Vec<Hand>,
}

impl Game {
    // Check hands to see if they match requirements:
    // No more than 12 red cubes, 13 green cubes, and 14 blue cubes
    pub fn check_hands(&self) -> bool {
        let total: usize = self.hands.iter().count();
        &self
            .hands
            .iter()
            .filter(|hand| hand.red <= 12 && hand.blue <= 14 && hand.green <= 13)
            .count()
            == &total
    }
    // Calc the max for each color, then multiply together
    pub fn calculate_power(&self) -> i32 {
        let mut max_hand = Hand {
            red: 0,
            green: 0,
            blue: 0,
        };

        self.hands.iter().for_each(|hand| {
            if hand.red > max_hand.red {
                max_hand.red = hand.red;
            }
            if hand.green > max_hand.green {
                max_hand.green = hand.green;
            }
            if hand.blue > max_hand.blue {
                max_hand.blue = hand.blue;
            }
        });

        max_hand.power()
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Error reading input.txt");

    let lines = input.split("\n");

    let games: Vec<Game> = lines
        .map(|line| {
            let id: i32 = line
                .split(": ")
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse()
                .unwrap();

            let hands: Vec<Hand> = line
                .split(": ")
                .last()
                .unwrap()
                .split("; ")
                .map(Hand::from_string)
                .collect();

            Game { id, hands }
        })
        .collect();

    let result = games
        .iter()
        .map(|game| game.calculate_power())
        .reduce(|a, b| a + b)
        .unwrap();

    println!("Result: {}", result);
}
