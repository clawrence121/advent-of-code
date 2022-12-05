use std::fs;

mod cleanup {
    #[derive(Clone, Copy, Debug)]
    pub struct Assignment {
        max: u32,
        min: u32,
    }

    impl Assignment {
        fn new(max: u32, min: u32) -> Assignment {
            Assignment { max, min }
        }
    }

    pub type Group = [Assignment; 2];

    pub fn create_pairs(assignments: &String) -> Vec<Group> {
        assignments
            .split("\n")
            .map(|line| {
                let mut pairs = line.split(",").map(|assignment| {
                    let mut assignment = assignment.split("-");

                    let min = assignment.next().unwrap().parse::<u32>().unwrap();
                    let max = assignment.next().unwrap().parse::<u32>().unwrap();

                    Assignment::new(max, min)
                });
                [pairs.next().unwrap(), pairs.next().unwrap()]
            })
            .collect::<Vec<Group>>()
    }

    pub fn count_assignment_pairs_where_one_assignment_contains_the_other(
        pairs: &Vec<Group>,
    ) -> u32 {
        pairs
            .iter()
            .filter(|pair| {
                let first = pair[0];
                let second = pair[1];

                (first.min <= second.min && first.max >= second.max)
                    || (second.min <= first.min && second.max >= first.max)
            })
            .count() as u32
    }

    pub fn count_number_of_pairs_that_have_assignments_that_overlap(pairs: &Vec<Group>) -> u32 {
        pairs
            .iter()
            .filter(|pair| {
                let first = pair[0];
                let second = pair[1];

                (first.min <= second.max && first.max >= second.min)
                    || (second.min <= first.max && second.max >= first.min)
            })
            .count() as u32
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    // println!("With text:\n{}", contents);

    let pairs = cleanup::create_pairs(&contents);

    let count = cleanup::count_assignment_pairs_where_one_assignment_contains_the_other(&pairs);

    println!("count: {}", count);

    let count = cleanup::count_number_of_pairs_that_have_assignments_that_overlap(&pairs);

    println!("count: {}", count);
}
