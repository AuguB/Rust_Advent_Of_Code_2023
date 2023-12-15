use std::fs::File;
use std::io::{self, BufRead};
#[allow(dead_code)]
#[allow(unused_imports)]
pub fn run1(file: &File) -> Result<String, Box<dyn std::error::Error>> {
    let _reader = io::BufReader::new(file);
    let mut acc = 0;
    for line in _reader.lines() {
        let line = line?;
        let split = line.split(|c| c == ':' || c == '|').collect::<Vec<&str>>();

        //split all the numbers into a vector, without empty strings
        let winning_numbers = split[1].split_whitespace().collect::<Vec<&str>>();
        let my_numbers = split[2].split_whitespace().collect::<Vec<&str>>();
        let mut score_count = 0;
        // the score is the size of the intersection of the two sets
        for num in winning_numbers {
            if my_numbers.contains(&num) {
                score_count += 1;
            }
        }
        acc += 2f32.powf(score_count as f32 - 1 as f32) as i32;
    }
    Ok(acc.to_string())
}

pub fn run2(file: &File) -> Result<String, Box<dyn std::error::Error>> {
    let _reader = io::BufReader::new(file);
    let mut acc = 0;
    // create a hashmap with a default value of 1
    let mut map = std::collections::HashMap::<usize, usize>::new();

    for (i_line, line) in _reader.lines().enumerate() {
        let card_number = &i_line + 1;
        let n_copies_of_this_card = *map.entry(card_number).or_insert(1);
        acc += n_copies_of_this_card;

        let line = line?;
        let split = line.split(|c| c == ':' || c == '|').collect::<Vec<&str>>();

        //split all the numbers into a vector, without empty strings
        let winning_numbers = split[1].split_whitespace().collect::<Vec<&str>>();
        let my_numbers = split[2].split_whitespace().collect::<Vec<&str>>();

        let mut score_count = 0;

        // the score count is the size of the intersection of the two sets
        for num in winning_numbers {
            if my_numbers.contains(&num) {
                score_count += 1;
            }
        }

        for next in 1..=score_count {
            *map.entry(card_number + next).or_insert(1) += n_copies_of_this_card;
        }
    }
    Ok(acc.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        let example_answer_part1 = "13";
        let result = run1(&File::open("src/day4/example_input.txt").unwrap());
        assert_eq!(result.unwrap(), example_answer_part1.to_string());
    }

    #[test]
    fn test_run2() {
        let example_answer_part2 = "30";
        let result = run2(&File::open("src/day4/example_input.txt").unwrap());
        assert_eq!(result.unwrap(), example_answer_part2.to_string());
    }
}
