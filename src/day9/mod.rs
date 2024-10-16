use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code)]
#[allow(unused_imports)]
pub fn run1(file: &File) -> Result<String, Box<dyn std::error::Error>> {
    let _reader = io::BufReader::new(file);
    let mut acc = 0;
    for line in _reader.lines() {
        let mut derivatives: Vec<Vec<i32>> = vec![line.unwrap().split(" ").map(|x| x.parse::<i32>().unwrap()).collect()];
        while !derivatives.last().unwrap().iter().all(|x| *x == 0){
            let last_derivative = derivatives.last().unwrap();
            let new_derivative: Vec<i32> = last_derivative.windows(2).map(|window| window[1] - window[0]).collect();
            derivatives.push(new_derivative);
        }
        acc += derivatives.iter().map(|l| l.last().unwrap()).sum::<i32>();
    }
    Ok(acc.to_string())
}

pub fn run2(file: &File) -> Result<String, Box<dyn std::error::Error>> {
    let _reader = io::BufReader::new(file);
    let mut acc = 0;
    for line in _reader.lines() {
        let mut derivatives: Vec<Vec<i32>> = vec![line.unwrap().split(" ").map(|x| x.parse::<i32>().unwrap()).collect()];
        while !derivatives.last().unwrap().iter().all(|x| *x == 0){
            let last_derivative = derivatives.last().unwrap();
            let new_derivative: Vec<i32> = last_derivative.windows(2).map(|window| window[1] - window[0]).collect();
            derivatives.push(new_derivative);
        }
        acc += derivatives.iter().map(|l: &Vec<i32>| l.first().unwrap()).rev().fold(0, |u: i32, v: &i32| (v-u));
    }
    Ok(acc.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        let example_answer_part1 = "8";
        let result = run1(&File::open("src/day9/example_input.txt").unwrap());
        assert_eq!(result.unwrap(), example_answer_part1.to_string());
    }

    #[test]
    fn test_run2() {
        let example_answer_part2 = "";
        let result = run2(&File::open("src/day9/example_input.txt").unwrap());
        assert_eq!(result.unwrap(), example_answer_part2.to_string());
    }
}

