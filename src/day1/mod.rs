use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code)]

pub fn run1(file: File) -> Result<String, Box<dyn std::error::Error>> {
    let reader = io::BufReader::new(file);

    let mut acc = 0;
    for line in reader.lines() {
        let calibration_value = extract_calibration_value(&line?, false)?;
        acc += calibration_value;
    }
    Ok(acc.to_string())
}

pub fn run2(file: File) -> Result<(), Box<dyn std::error::Error>> {
    let reader = io::BufReader::new(file);

    let mut acc = 0;
    for line in reader.lines() {
        let calibration_value = extract_calibration_value(&line?, true)?;
        acc += calibration_value;
    }
    println!("{}", acc.to_string());
    Ok(())
}

pub fn extract_calibration_value(
    line: &str,
    parse_written_out_digits: bool,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut new_line = line.to_string();
    if parse_written_out_digits {
        new_line = convert_substrings_to_numbers(&line)?;
    }

    let nums: Vec<char> = Regex::new(r"[a-z]")
        .unwrap()
        .replace_all(&new_line, "")
        .chars()
        .collect();
    format!("{}{}", nums[0], &nums.last().unwrap())
        .parse::<i32>()
        .map_err(|e| e.into())
}

pub fn convert_substrings_to_numbers(s: &str) -> Result<String, Box<dyn std::error::Error>> {
    let list_of_numerical_substrings = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut new_string = s.to_string();

    for (i, string_number) in list_of_numerical_substrings.iter().enumerate() {
        new_string = new_string.replacen(
            string_number,
            &format!("{}{}{}", string_number, i + 1, string_number),
            10,
        );
    }
    Ok(new_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_run() {
        let result = run1(File::open("src/day1/example_input.txt").unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_substrings_to_numbers() {
        let result = convert_substrings_to_numbers("two1nine");
        assert_eq!(result.unwrap(), "219");

        let result = convert_substrings_to_numbers("eightwothree");
        assert_eq!(result.unwrap(), "8wo3");

        let result = convert_substrings_to_numbers("xtwone3four");
        assert_eq!(result.unwrap(), "x2ne34");

        // let result = convert_substrings_to_numbers("foo");
        // assert_eq!(result.unwrap(), "foo");
    }

    #[test]
    fn test_extract_calibration_value() {
        let result = extract_calibration_value("two1nine", true);
        assert_eq!(result.unwrap(), 29);

        let result = extract_calibration_value("eightwothree", true);
        assert_eq!(result.unwrap(), 83);

        let result = extract_calibration_value("xtwone3four", true);
        assert_eq!(result.unwrap(), 24);

        let result = extract_calibration_value("xtwone3four", true);
        assert_eq!(result.unwrap(), 24);

        let result = extract_calibration_value("4nineeightseven2", true);
        assert_eq!(result.unwrap(), 42);

        let result = extract_calibration_value("zoneight234", true);
        assert_eq!(result.unwrap(), 14);

        let result = extract_calibration_value("7pqrstsixteen", true);
        assert_eq!(result.unwrap(), 76);

        let result = extract_calibration_value("eightwo", true);
        assert_eq!(result.unwrap(), 82);
    }
}
