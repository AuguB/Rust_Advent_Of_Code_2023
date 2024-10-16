use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code)]
#[allow(unused_imports)]
pub fn run1(file: &File) -> Result<String, Box<dyn std::error::Error>> {
    let _reader = io::BufReader::new(file);
    let mut iterator = _reader.lines();
    let times = iterator
        .next()
        .unwrap()
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    let distances = iterator
        .next()
        .unwrap()
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    let result = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| number_of_possible_held_down_times(*t, *d))
        .product::<i32>();

    Ok(result.to_string())
}

pub fn run2(_file: &File) -> Result<String, Box<dyn std::error::Error>> {
    Ok(number_of_possible_held_down_times(46828479., 347152214061471.).to_string())
}

fn number_of_possible_held_down_times(t: f64, d: f64) -> i32 {
    let (mintime, maxtime) = min_and_max_time_held(t, d).unwrap();
    return (maxtime - mintime + 1.) as i32;
}

fn min_and_max_time_held(t: f64, d: f64) -> Option<(f64, f64)> {
    if let Some((border1, border2)) = abc_formula(1., -t, d) {
        let mintime = border1.min(border2).floor() + 1.;
        let maxtime = border1.max(border2).ceil() - 1.;
        return Some((mintime, maxtime));
    }
    None
}

fn abc_formula(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b.powf(2.) - 4. * a * c;

    if discriminant < 0. {
        // No real solutions
        return None;
    } else {
        let x1 = (-b + discriminant.sqrt()) / (2. * a);
        let x2 = (-b - discriminant.sqrt()) / (2. * a);
        return Some((x1, x2));
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_run1() {
        let example_answer_part1 = "288";
        let result = run1(&File::open("src/day6/example_input.txt").unwrap());
        assert_eq!(result.unwrap(), example_answer_part1.to_string());
    }

    #[test]
    fn test_run2() {
        let example_answer_part2 = "";
        let result = run2(&File::open("src/day6/example_input.txt").unwrap());
        assert_eq!(result.unwrap(), example_answer_part2.to_string());
    }

    #[test]
    fn test_abc_formula() {
        let result = abc_formula(1., 2., 1.);
        assert_eq!(result.unwrap(), (-1., -1.));

        let result = abc_formula(3., 2., 5.);
        assert_eq!(result, None);
    }

    #[test]
    fn test_number_of_possible_held_down_times() {
        let result = number_of_possible_held_down_times(7., 9.);
        assert_eq!(result, 4);

        let result = number_of_possible_held_down_times(15., 40.);
        assert_eq!(result, 8);

        let result = number_of_possible_held_down_times(30., 200.);
        assert_eq!(result, 9);
    }
}
