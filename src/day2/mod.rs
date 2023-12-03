use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code)]
#[allow(unused_imports)]
pub fn run1(file: &File) -> Result<String, Box<dyn std::error::Error>> {
    let _reader = io::BufReader::new(file);
    let mut mymap = HashMap::new();
    mymap.insert("red", ",2");
    mymap.insert("green", ",1");
    mymap.insert("blue", ",0");
    let mut acc: i32 = 0;
    for (game_i, line) in _reader.lines().enumerate() {
        let mut mapped: String = line?.clone();
        for (key, value) in &mymap {
            mapped = mapped.replace(key, value);
        }
        let split = mapped.split(":").collect::<Vec<&str>>();
        let sets_to_ints: Vec<i32> = split[1]
            .replace(" ", "")
            .split(|c| c == ',' || c == ';')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let possible = sets_to_ints
            .chunks(2)
            .map(|c| c.iter().sum::<i32>())
            .max()
            .unwrap()
            <= 14;
        if possible {
            acc += (game_i as i32) + 1;
        }
    }
    Ok((acc).to_string())
}

pub fn run2(file: &File) -> Result<String, Box<dyn std::error::Error>> {
    let _reader = io::BufReader::new(file);
    let mut acc: i32 = 0;
    for line in _reader.lines() {
        let mut myacc = 1;
        let line = &line?;
        for color in vec!["red", "blue", "green"] {
            let pattern = format!(r"[,|;|:] (\d*) {}", color);
            let re = Regex::new(&pattern).unwrap();
            let maxcol = re
                .captures_iter(line)
                .map(|c| c[1].parse::<i32>().unwrap())
                .max()
                .unwrap();
            myacc *= maxcol;
        }
        acc += myacc;
    }
    Ok((acc).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        let example_answer_part1 = "8";
        let result = run1(&File::open("src/day2/example_input.txt").unwrap());
        assert_eq!(result.unwrap(), example_answer_part1.to_string());
    }

    #[test]
    fn test_run2() {
        let example_answer_part2 = "2286";
        let result = run2(&File::open("src/day2/example_input.txt").unwrap());
        assert_eq!(result.unwrap(), example_answer_part2.to_string());
    }
}
