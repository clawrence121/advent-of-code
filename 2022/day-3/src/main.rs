use std::fs;

mod rucksacks {
    #[derive(Clone)]
    struct Rucksack {
        all: Vec<char>,
        compartment1: Vec<char>,
        compartment2: Vec<char>,
    }

    // Map char a-z to 1-26 and A-Z to 27-52
    fn char_to_int(c: char) -> u8 {
        match c {
            'a'..='z' => c as u8 - 96,
            'A'..='Z' => c as u8 - 38,
            _ => 0,
        }
    }

    // check for matching chars in two compartments/print
    fn check_compartments(rucksack: &Rucksack) -> Vec<char> {
        // compare compartments for values that exist in both
        let mut common_values = Vec::new();
        for c1 in &rucksack.compartment1 {
            for c2 in &rucksack.compartment2 {
                if c1 == c2 {
                    common_values.push(c1.clone());
                }
            }
        }

        common_values.clone()
    }

    // check for matching chars across 3 bags
    fn check_all(rucksack: [&Rucksack; 3]) -> Vec<char> {
        // compare compartments for values that exist in all
        let mut common_values = Vec::new();
        for c1 in &rucksack[0].all {
            for c2 in &rucksack[1].all {
                for c3 in &rucksack[2].all {
                    if c1 == c2 && c2 == c3 {
                        common_values.push(c1.clone());
                    }
                }
            }
        }

        common_values.clone()
    }

    pub fn reorganise_rucksack(bags: &Vec<&str>) -> u32 {
        bags.iter()
            .map(|bag| {
                // split the bag in half based on it's str length
                let half = bag.len() / 2;
                let (comp1, comp2) = bag.split_at(half);

                Rucksack {
                    all: bag.chars().collect(),
                    compartment1: comp1.chars().collect(),
                    compartment2: comp2.chars().collect(),
                }
            })
            .map(|rucksack| check_compartments(&rucksack))
            .map(|common_values| char_to_int(common_values[0]) as u32)
            .sum::<u32>()
    }

    pub fn sort_badges(bags: &Vec<&str>) -> u32 {
        // maps bags into rucksacks
        let rucksacks: Vec<Rucksack> = bags
            .iter()
            .map(|bag| {
                // split the bag in half based on it's str length
                let half = bag.len() / 2;
                let (comp1, comp2) = bag.split_at(half);

                Rucksack {
                    all: bag.chars().collect(),
                    compartment1: comp1.chars().collect(),
                    compartment2: comp2.chars().collect(),
                }
            })
            .collect();

        // group rucksackes into vectors of 3
        let mut grouped_rucksacks = Vec::new();
        let mut group = Vec::new();
        for rucksack in rucksacks {
            group.push(rucksack);
            if group.len() == 3 {
                grouped_rucksacks.push(group.clone());
                group.clear();
            }
        }

        // check all rucksacks in each group for common values
        grouped_rucksacks
            .iter()
            .map(|group| {
                let rucksacks: [&Rucksack; 3] = [&group[0], &group[1], &group[2]];
                check_all(rucksacks)
            })
            .map(|common_values| char_to_int(common_values[0]) as u32)
            .sum::<u32>()
    }
}

fn main() {
    let contents =
        fs::read_to_string("input-full.txt").expect("Something went wrong reading the file");

    let bags: Vec<&str> = contents.split("\n").collect();

    let reorganise_sum = rucksacks::reorganise_rucksack(&bags);

    let badge_sum = rucksacks::sort_badges(&bags);

    // print results
    println!("Reorganise sum: {}", reorganise_sum);
    println!("Badge sum: {}", badge_sum);
}
