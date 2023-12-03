use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
#[allow(dead_code)]
#[allow(unused_imports)]
pub fn run1(file: &File) -> Result<String, Box<dyn std::error::Error>> {
    let _reader = io::BufReader::new(file);
    let mut previous_parts: Vec<(i32, i32, i32, bool)> = Vec::new();
    let mut previous_symbol_locations: Vec<(i32, i32)> = Vec::new();

    let parts_pattern = Regex::new(r"\d+").unwrap();
    let symbol_pattern = Regex::new(r"[^\d\.]").unwrap();

    let mut part_acc = 0;
    for line in _reader.lines() {
        let line = &line?;
        // Capute all the parts in the current line
        let mut parts: Vec<(i32, i32, i32, bool)> = parts_pattern
            .captures_iter(line)
            .map(|f| {
                (
                    f[0].parse::<i32>().unwrap(),
                    f.get(0).unwrap().start() as i32,
                    f.get(0).unwrap().end() as i32 - 1,
                    false,
                )
            })
            .collect();

        // Capture all the symbols in the current line
        let symbol_locations: Vec<(i32, i32)> = symbol_pattern
            .captures_iter(line)
            .map(|f| {
                (
                    f.get(0).unwrap().start() as i32 - 1,
                    f.get(0).unwrap().end() as i32,
                )
            })
            .collect();

        // Add all the previous parts that were not added yet, and that overlap with the current symbols
        for (start, end) in symbol_locations.iter() {
            for (part, part_start, part_end, part_added) in previous_parts.iter_mut() {
                if !*part_added && *part_start <= *end && *part_end >= *start {
                    *part_added = true;
                    part_acc += part.clone();
                }
            }
        }

        // Add the current parts that overlap with the current and previous symbols
        for (start, end) in symbol_locations
            .iter()
            .chain(previous_symbol_locations.iter())
        {
            for (part, part_start, part_end, part_added) in parts.iter_mut() {
                if !*part_added && *part_start <= *end && *part_end >= *start {
                    *part_added = true;
                    part_acc += part.clone();
                }
            }
        }

        // Update previous symbols and previous parts with the new values
        previous_symbol_locations = symbol_locations;
        previous_parts = parts;
    }
    Ok(part_acc.to_string())
}

pub fn run2(file: &File) -> Result<String, Box<dyn std::error::Error>> {
    let _reader = io::BufReader::new(file);
    let mut previous_parts: Vec<(i32, i32, i32)> = Vec::new();
    let mut previous_gears: Vec<(i32, i32, i32, i32)> = Vec::new();

    let parts_pattern = Regex::new(r"\d+").unwrap();
    let gear_pattern = Regex::new(r"\*{1}").unwrap();

    let mut part_acc = 0;
    for line in _reader.lines() {
        let line = &line?;

        // Capute all the parts in the current line
        let mut parts: Vec<(i32, i32, i32)> = parts_pattern
            .captures_iter(line)
            .map(|f| {
                (
                    f[0].parse::<i32>().unwrap(),
                    f.get(0).unwrap().start() as i32,
                    f.get(0).unwrap().end() as i32 - 1,
                )
            })
            .collect();
        // Capture all the symbols in the current line
        let mut gears: Vec<(i32, i32, i32, i32)> = gear_pattern
            .captures_iter(line)
            .map(|f| {
                (
                    f.get(0).unwrap().start() as i32 - 1,
                    f.get(0).unwrap().end() as i32,
                    0,
                    1,
                )
            })
            .collect();

        // For all the previous and current gears, if they overlap with any of the current parts, update the gear
        for (start, end, gear_count, gear_multiplier) in
            previous_gears.iter_mut().chain(gears.iter_mut())
        {
            for (part, part_start, part_end) in parts.iter_mut() {
                if *part_start <= *end && *part_end >= *start {
                    *gear_count += 1;
                    *gear_multiplier *= part.clone();
                }
            }
        }

        // For all of the current gears, if they overlap with any of the previous parts, update the gear
        for (start, end, gear_count, gear_multiplier) in gears.iter_mut() {
            for (part, part_start, part_end) in previous_parts.iter_mut() {
                if *part_start <= *end && *part_end >= *start {
                    *gear_count += 1;
                    *gear_multiplier *= part.clone();
                }
            }
        }

        // For all of the previous gears, if the gear count is exactly two, add the gear multiplier to the accumulator
        for (_, _, gear_count, gear_multiplier) in previous_gears.iter_mut() {
            if *gear_count == 2 {
                part_acc += gear_multiplier.clone()
            }
        }

        // Update previous gears and previous parts with the new values
        previous_gears = gears;
        previous_parts = parts;
    }
    Ok(part_acc.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        let dummy_answer_part1 = "4361";
        let result = run1(&File::open("src/day3/dummy_input.txt").unwrap());
        assert_eq!(result.unwrap(), dummy_answer_part1.to_string());
    }

    #[test]
    fn test_run2() {
        let dummy_answer_part2 = "467835";
        let result = run2(&File::open("src/day3/dummy_input.txt").unwrap());
        assert_eq!(result.unwrap(), dummy_answer_part2.to_string());
    }
}
