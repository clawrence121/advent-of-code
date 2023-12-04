use crate::engine::{find_gear_locations, find_unique_part_numbers, find_unique_symbols};
use std::fs;
use std::str::Split;

mod engine {
    use regex::Regex;
    use std::collections::HashSet;
    use std::str::Split;

    pub fn find_unique_symbols(lines: &Split<&str>) -> Vec<char> {
        let mut symbol_buffer: Vec<char> = Vec::new();
        lines.clone().for_each(|line| {
            line.chars().for_each(|c| {
                if !c.is_alphanumeric() && !(c == '.') {
                    symbol_buffer.push(c);
                }
            });
        });
        let unique_symbols: HashSet<_> = symbol_buffer.into_iter().collect();
        let symbols: Vec<char> = unique_symbols.into_iter().collect();

        symbols
    }

    #[derive(Debug)]
    pub struct PartNumber {
        pub number: usize,
        line: isize,
        column: isize,
        length: isize,
        pub valid: bool,
    }

    impl PartNumber {
        pub fn validate(&mut self, input: &Split<&str>, symbols: &Vec<char>) {
            let mut chars_around_buf: Vec<char> = Vec::new();

            for i in self.line - 1..self.line + 2 {
                match input.clone().into_iter().nth(i as usize) {
                    Some(line) => {
                        let mut chars: Vec<char> = line
                            .chars()
                            .skip((if self.column > 1 { self.column - 1 } else { 0 }) as usize)
                            .take((self.length + (if self.column > 0 { 2 } else { 1 })) as usize)
                            .collect();
                        chars_around_buf.append(&mut chars);
                    }
                    None => {}
                }
            }

            self.valid = chars_around_buf.iter().any(|&c| symbols.contains(&c));
        }
    }

    pub fn find_unique_part_numbers(lines: &Split<&str>, symbols: &Vec<char>) -> Vec<PartNumber> {
        let mut part_number_buffer: Vec<PartNumber> = Vec::new();
        lines.clone().enumerate().for_each(|(idx, line)| {
            line.split(|c| symbols.contains(&c) || c == '.')
                .for_each(|item| {
                    if item.parse::<usize>().is_ok() {
                        let matcher = Regex::new(&format!(r"[^0-9]{}[^0-9]", item)).unwrap();
                        let matcher_eol = Regex::new(&format!(r"[^0-9]{}", item)).unwrap();
                        let column = match matcher.find(line) {
                            Some(matched) => matched.start() + 1,
                            None => match matcher_eol.find(line) {
                                Some(matched) => matched.start() + 1,
                                None => 0,
                            },
                        };

                        part_number_buffer.push(PartNumber {
                            number: item.parse::<usize>().unwrap(),
                            line: idx as isize,
                            column: column as isize,
                            length: item.len() as isize,
                            valid: false,
                        })
                    }
                })
        });

        part_number_buffer
    }

    #[derive(Debug)]
    pub struct GearLocation {
        line: usize,
        column: usize,
        pub valid: bool,
        pub ratio: Option<usize>,
    }

    impl GearLocation {
        pub fn validate_ratio(&mut self, input: &Split<&str>, symbols: &Vec<char>) {
            let mut found_parts: Vec<usize> = Vec::new();

            for i in self.line - 1..self.line + 2 {
                match input.clone().into_iter().nth(i) {
                    Some(line) => {
                        let mut already_found_on_line: Vec<Vec<char>> = Vec::new();

                        // walk through the line to find a number
                        line.chars().enumerate().for_each(|(cidx, c)| {
                            if !(self.column - 1..=self.column + 1).contains(&cidx) {
                                return;
                            }
                            if !c.is_numeric() {
                                return;
                            }
                            let mut num_buf: Vec<char> = line
                                .chars()
                                .rev()
                                .skip(line.len() - cidx - 1)
                                .take_while(|&c| c != '.' && !symbols.contains(&c))
                                .collect();
                            let mut fwd_buf = line
                                .chars()
                                .skip(cidx + 1)
                                .take_while(|&c| c != '.' && !symbols.contains(&c))
                                .collect::<Vec<char>>();
                            num_buf.reverse();
                            num_buf.append(&mut fwd_buf);
                            if already_found_on_line.contains(&num_buf) {
                                return;
                            }
                            already_found_on_line.push(num_buf.clone());
                            found_parts.push(
                                num_buf
                                    .iter()
                                    .collect::<String>()
                                    .parse::<usize>()
                                    .unwrap_or(0),
                            );
                        });
                    }
                    None => {}
                }
            }

            if found_parts.len() != 2 {
                return;
            }

            self.ratio = found_parts.into_iter().reduce(|a, b| a * b);
        }
    }

    pub fn find_gear_locations(lines: &Split<&str>) -> Vec<GearLocation> {
        let mut gear_location_buffer: Vec<GearLocation> = Vec::new();
        lines.clone().enumerate().for_each(|(idx, line)| {
            line.chars().enumerate().for_each(|(cidx, c)| {
                if c == '*' {
                    gear_location_buffer.push(GearLocation {
                        line: idx,
                        column: cidx,
                        valid: false,
                        ratio: None,
                    })
                }
            })
        });

        gear_location_buffer
    }
}

fn run_part1(input: Split<&str>) -> usize {
    let symbols = find_unique_symbols(&input);
    let part_numbers = find_unique_part_numbers(&input, &symbols);

    let result: usize = part_numbers
        .into_iter()
        .map(|mut number| {
            number.validate(&input, &symbols);
            number
        })
        .filter(|number| number.valid)
        .map(|number| number.number)
        .reduce(|a, b| a + b)
        .unwrap_or(0);

    result
}

fn run_part2(input: &Split<&str>) -> usize {
    let symbols = find_unique_symbols(&input);
    let gear_locations = find_gear_locations(&input);

    gear_locations
        .into_iter()
        .filter_map(|mut location| {
            location.validate_ratio(input, &symbols);
            location.ratio
        })
        .reduce(|a, b| a + b)
        .unwrap_or(0)
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Error reading input.txt");

    let lines = input.split("\n");

    let part1 = run_part1(lines.clone());

    println!("Part 1 result: {}", part1);

    let part2 = run_part2(&lines);

    println!("Part 2 result: {}", part2);
}
