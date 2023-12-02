use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Error reading input.txt");

    let lines = input.split("\n");
    let result: i32 = lines
        .into_iter()
        .map(|line| {
            let mut nums = line.to_string();
            nums.retain(|c| c.is_numeric());

            let first_and_last = format!(
                "{}{}",
                nums.chars().next().unwrap(),
                nums.chars().last().unwrap()
            );
            first_and_last.parse().unwrap_or(0)
        })
        .sum();

    println!("Result: {}", result);
}
