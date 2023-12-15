#!/bin/bash

cd "/home/stijn/Projects/Rust_Advent_Of_Code_2023/src"
# Get the day number from the first command line argument
day_number=$1

# Create the directory
mkdir "day$day_number"

# Create the input.txt, instructions.txt, and mod.rs files
touch "day$day_number/input.txt"
touch "day$day_number/example_input.txt"

# Add the new day to the lib.rs file
echo "pub mod day$day_number;" >> "lib.rs"

# Add the new day to the main.rs file
cat << EOF > "day$day_number/mod.rs"
use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code)]
#[allow(unused_imports)]
pub fn run1(file: &File) -> Result<String, Box<dyn std::error::Error>> {
    let _reader = io::BufReader::new(file);
    for line in _reader.lines() {
        let _line = line?;
    }
    Err("Not implemented".into())
}

pub fn run2(file: &File) -> Result<String, Box<dyn std::error::Error>> {
    let _reader = io::BufReader::new(file);
    Err("Not implemented".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        let example_answer_part1 = "8";
        let result = run1(&File::open("src/day$day_number/example_input.txt").unwrap());
        assert_eq!(result.unwrap(), example_answer_part1.to_string());
    }

    #[test]
    fn test_run2() {
        let example_answer_part2 = "";
        let result = run2(&File::open("src/day$day_number/example_input.txt").unwrap());
        assert_eq!(result.unwrap(), example_answer_part2.to_string());
    }
}

EOF