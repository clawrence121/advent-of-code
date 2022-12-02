use std::fs;

mod elfs {
    pub fn get_max_cals(contents: &String) -> u32 {
        let max_cals = contents
            .split("\n\n")
            .map(|elf| {
                elf.split("\n")
                    .map(|line| line.parse::<u32>().unwrap())
                    .sum::<u32>()
            })
            .max()
            .unwrap();

        return max_cals;
    }

    pub fn get_top_3_cals(contents: &String) -> u32 {
        let mut max_cals = contents
            .split("\n\n")
            .map(|elf| {
                elf.split("\n")
                    .map(|line| line.parse::<u32>().unwrap())
                    .sum::<u32>()
            })
            .collect::<Vec<_>>();

        max_cals.sort_by(|a, b| b.cmp(a));

        return max_cals[0..3].iter().sum::<u32>();
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    println!("Max cals: {}", elfs::get_max_cals(&contents));

    println!("Top 3 cals: {}", elfs::get_top_3_cals(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_max_cals() {
        let contents = String::from("1\n\n2\n\n3\n\n4\n\n5\n\n6\n\n7\n\n8\n\n9");

        assert_eq!(elfs::get_max_cals(&contents), 9);

        let contents = String::from("1\n\n2\n\n3\n\n4\n\n5\n\n6\n\n7\n\n8\n\n9\n\n10\n\n11\n\n12");

        assert_eq!(elfs::get_max_cals(&contents), 12);
    }

    #[test]
    fn test_get_top_3_cals() {
        let contents = String::from("1\n\n2\n\n3\n\n4\n\n5\n\n6\n\n7\n\n8\n\n9");

        assert_eq!(elfs::get_top_3_cals(&contents), 24);

        let contents = String::from("1\n\n2\n\n3\n\n4\n\n5\n\n6\n\n7\n\n8\n\n9\n\n10\n\n11\n\n12");

        assert_eq!(elfs::get_top_3_cals(&contents), 33);
    }
}
