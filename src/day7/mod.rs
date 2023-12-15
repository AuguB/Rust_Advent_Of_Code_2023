use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::MAIN_SEPARATOR;
use std::sync::Mutex;

lazy_static! {
    static ref HAND_TYPES: Mutex<HashMap<Vec<u8>, u8>> = {
        let mut map = HashMap::new();
        map.insert(vec![1, 1, 1, 1, 1], 0);
        map.insert(vec![1, 1, 1, 2], 1);
        map.insert(vec![1, 2, 2], 2);
        map.insert(vec![1, 1, 3], 3);
        map.insert(vec![2, 3], 4);
        map.insert(vec![1, 4], 5);
        map.insert(vec![5], 6);
        Mutex::new(map)
    };
    static ref CARD_ORDERING_1: Mutex<Vec<char>> = {
        let vec = vec![
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ];
        Mutex::new(vec)
    };
    static ref CARD_ORDERING_2: Mutex<Vec<char>> = {
        let vec = vec![
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ];
        Mutex::new(vec)
    };
}

#[allow(dead_code)]
#[allow(unused_imports)]
pub fn run1(file: &File) -> Result<String, Box<dyn std::error::Error>> {
    let _reader = io::BufReader::new(file);
    let mut hands: Vec<(u8, u32, i64)> = Vec::new();
    // Lock the mutex
    let card_ordering = CARD_ORDERING_1.lock().unwrap();

    for line in _reader.lines() {
        let line = &line?;
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let hand = split[0].to_string();
        let hand_to_int = hand_as_int(&hand, &card_ordering);
        let hand_type = hand_type(&hand);
        let bid = split[1].parse::<i64>().unwrap();
        hands.push((hand_type, hand_to_int, bid));
    }
    hands.sort_by_key(|c| (c.0, c.1));
    let total_score: i64 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, _, s))| ((i as i64) + 1) * *s)
        .sum();
    return Ok(total_score.to_string());
}

fn hand_type(hand: &str) -> u8 {
    // Lock the mutex
    let hand_types = HAND_TYPES.lock().unwrap();
    hand_types
        .get(&count_unique_characters(hand))
        .unwrap()
        .clone()
}

pub fn run2(file: &File) -> Result<String, Box<dyn std::error::Error>> {
    let _reader = io::BufReader::new(file);
    let mut hands: Vec<(u8, u32, i64)> = Vec::new();
    let card_ordering = CARD_ORDERING_2.lock().unwrap();

    for line in _reader.lines() {
        let line = &line?;
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let hand = split[0].to_string();
        let hand_to_int = hand_as_int(&hand, &card_ordering);
        let hand_type = get_augmented_hand_type(&hand);
        let bid = split[1].parse::<i64>().unwrap();
        hands.push((hand_type, hand_to_int, bid));
    }
    hands.sort_by_key(|c| (c.0, c.1));
    let total_score: i64 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, _, s))| ((i as i64) + 1) * *s)
        .sum();
    return Ok(total_score.to_string());
}

fn hand_as_int(hand: &String, card_ordering: &Vec<char>) -> u32 {
    let hand_to_hex = hand
        .chars()
        .map(|c| {
            format!(
                "{:x}",
                card_ordering.iter().position(|&char| char == c).unwrap()
            )
        })
        .collect::<Vec<String>>()
        .join("");
    let hex_to_int = u32::from_str_radix(&hand_to_hex, 16).unwrap();
    hex_to_int
}

fn get_augmented_hand_type(hand: &str) -> u8 {
    let hand_type = hand_type(&hand);
    let count_j = hand.chars().filter(|&c| c == 'J').count();
    let mut augmented_hand_type = 0;
    match (count_j, hand_type) {
        (0, _) => {
            augmented_hand_type = hand_type;
        }
        (1, 0) | (1, 5) | (4, 5) => {
            augmented_hand_type = hand_type + 1;
        }
        (2, 2) => {
            augmented_hand_type = hand_type + 3;
        }
        (_, _) => {
            augmented_hand_type = hand_type + 2;
        }
    }
    augmented_hand_type
}

fn count_unique_characters(s: &str) -> Vec<u8> {
    let mut counts: HashMap<char, u8> = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let mut counts = counts.values().cloned().collect::<Vec<u8>>();
    counts.sort();
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        let example_answer_part1 = "6440";
        let result = run1(&File::open("src/day7/example_input.txt").unwrap());
        assert_eq!(result.unwrap(), example_answer_part1.to_string());
    }

    #[test]
    fn test_run2() {
        let example_answer_part2 = "5905";
        let result = run2(&File::open("src/day7/example_input.txt").unwrap());
        assert_eq!(result.unwrap(), example_answer_part2.to_string());
    }

    #[test]
    fn test_augmented_hand_type() {
        let test_cases = vec![
            ("2345J", 0, 1),
            ("2344J", 1, 3),
            ("2233J", 2, 4),
            ("2333J", 3, 5),
            ("2222J", 5, 6),
            ("234JJ", 1, 3),
            ("233JJ", 2, 5),
            ("222JJ", 4, 6),
            ("23JJJ", 3, 5),
            ("22JJJ", 4, 6),
            ("2JJJJ", 5, 6),
        ];

        for (hand, expected_hand_type, expected_augmented_hand_type) in test_cases {
            let result = hand_type(&hand);
            assert_eq!(result, expected_hand_type);
            let result = get_augmented_hand_type(&hand);
            assert_eq!(result, expected_augmented_hand_type);
        }
    }

    #[test]
    fn test_hand_as_int_1() {
        // Lock the mutex
        let card_ordering_1 = CARD_ORDERING_1.lock().unwrap();
        let test_cases = vec![("KKKK2", "QQQQ2"), ("QQQQ2", "JKKK2"), ("JKKK2", "99992")];
        for (larger, smaller) in test_cases {
            let small = hand_as_int(&smaller.to_string(), &card_ordering_1);
            let large = hand_as_int(&larger.to_string(), &card_ordering_1);
            assert!(small < large);
        }
    }

    #[test]
    fn test_hand_as_int_2() {
        // Lock the mutex
        let card_ordering_2 = CARD_ORDERING_2.lock().unwrap();
        let test_cases = vec![("KKKK2", "QQQQ2"), ("QQQQ2", "JKKK2"), ("99992", "JKKK2")];
        for (larger, smaller) in test_cases {
            let small = hand_as_int(&smaller.to_string(), &card_ordering_2);
            let large = hand_as_int(&larger.to_string(), &card_ordering_2);
            assert!(small < large);
        }
    }
}
