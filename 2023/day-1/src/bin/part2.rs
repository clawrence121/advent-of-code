use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Error reading input.txt");

    let mut nums: HashMap<&str, i32> = HashMap::new();
    nums.insert("one", 1);
    nums.insert("two", 2);
    nums.insert("three", 3);
    nums.insert("four", 4);
    nums.insert("five", 5);
    nums.insert("six", 6);
    nums.insert("seven", 7);
    nums.insert("eight", 8);
    nums.insert("nine", 9);

    let lines = input.split("\n");

    let result: i32 = lines
        .into_iter()
        .map(|line| {
            let first_and_last_nums = line
                .char_indices()
                .map(|(idx, char)| {
                    if char.is_numeric() {
                        return Some(char.to_string().parse().unwrap());
                    }

                    let mut buf: Vec<char> = Vec::new();

                    for i in idx..line.len() {
                        buf.push(line.chars().nth(i).unwrap());

                        let needle: String = buf.clone().into_iter().collect();
                        match nums.get(&*needle) {
                            Some(num) => {
                                return Some(*num);
                            }
                            None => {}
                        }
                    }

                    None
                })
                .filter_map(|num| num);

            let first_and_last = format!(
                "{}{}",
                first_and_last_nums.clone().next().unwrap(),
                first_and_last_nums.clone().last().unwrap()
            );
            first_and_last.parse().unwrap_or(0)
        })
        .sum();

    println!("Result: {}", result);
}
