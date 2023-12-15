use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code)]
#[allow(unused_imports)]
pub fn run1(file: &File) -> Result<String, Box<dyn std::error::Error>> {
    let _reader = io::BufReader::new(file);
    let mut lines_iter = _reader.lines();

    // take only the first line of the reader
    let mut seeds = lines_iter
        .next()
        .unwrap()?
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|c| c.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // keep track of which seeds have been mapped
    let mut seed_mapped: Vec<bool> = vec![false; seeds.len()];

    // iterate over the rest of the lines
    for line in lines_iter {
        // unwrap the line
        let line = &line?;
        //  if the line is empty or ends with a colon, reset the seed_mapped vector
        if (line == "") || (line.ends_with(":")) {
            seed_mapped = vec![false; seeds.len()];
        } else {
            // otherwise, parse the line and map the seeds
            let current_map_def = line
                .split_whitespace()
                .map(|c| c.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            let dest = current_map_def[0];
            let source = current_map_def[1];
            let range = current_map_def[2];
            for (i, seed) in seeds.iter_mut().enumerate() {
                // if the seed is unmapped and within the range, map it
                if !seed_mapped[i] && (source <= *seed) && (*seed <= (source + range - 1)) {
                    *seed = dest + (*seed - source);
                    seed_mapped[i] = true;
                }
            }
        }
    }
    Ok(seeds.iter().min().unwrap().to_string())
}

pub fn run2(file: &File) -> Result<String, Box<dyn std::error::Error>> {
    let _reader = io::BufReader::new(file);
    let mut lines_iter = _reader.lines();

    // take only the first line of the reader
    let seeds = lines_iter
        .next()
        .unwrap()?
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|c| c.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut current_phase: Vec<(u64, u64)> = seeds
        .chunks(2)
        .map(|c| (c[0], c[0] + c[1] - 1))
        .collect::<Vec<(u64, u64)>>();

    let mut next_phase: Vec<(u64, u64)> = Vec::new();
    let mut _i = 0;
    let mut _j = 0;
    // iterate over the rest of the lines
    for line in lines_iter {
        // unwrap the line
        let line = &line?;
        //  if the line is empty or ends with a colon, add all the non-mapped seeds to the new_seeds vector, and then replace the seeds vector with the new_seeds vector
        if (line == "") || (line.ends_with(":")) {
            _i += 1;
            _j = 0;
            for seed in current_phase.iter() {
                next_phase.push(seed.clone());
            }
            current_phase = next_phase;
            current_phase = merge_overlapping_ranges(&mut current_phase);
            next_phase = Vec::new();
        } else {
            _j += 1;
            // otherwise, parse the line and map the seeds
            let current_map_def = line
                .split_whitespace()
                .map(|c| c.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            let map_dest = current_map_def[0];
            let map_source = current_map_def[1];
            let map_length = current_map_def[2];
            let mut new_current_phase = Vec::new();
            for (seeds_start, seeds_end) in current_phase.iter_mut() {
                // The ranges do not overlap if the end of one range is less than the start of the other
                let overlaps =
                    !((map_source + map_length - 1 < *seeds_start) || (map_source > *seeds_end));

                if overlaps {
                    let overlap_start = if map_source > *seeds_start {
                        map_source
                    } else {
                        *seeds_start
                    };
                    let overlap_end = if map_source + map_length - 1 < *seeds_end {
                        map_source + map_length - 1
                    } else {
                        *seeds_end
                    };
                    next_phase.push((
                        map_dest + (overlap_start - map_source),
                        map_dest + (overlap_end - map_source),
                    ));
                    // Add the non-overlapping parts to the new_current_phase vector
                    if map_source > *seeds_start {
                        new_current_phase.push((*seeds_start, map_source - 1));
                    }
                    if map_source + map_length - 1 < *seeds_end {
                        new_current_phase.push((map_source + map_length, *seeds_end));
                    }
                } else {
                    new_current_phase.push((seeds_start.clone(), seeds_end.clone()));
                }
            }
            current_phase = new_current_phase;
            current_phase = merge_overlapping_ranges(&mut current_phase);
        }
    }
    // find the smallest first element in the current_phase vector
    Ok(current_phase.iter().min().unwrap().0.to_string())
}

fn merge_overlapping_ranges(ranges: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    // Check if the ranges are empty
    if ranges.is_empty() {
        return Vec::new();
    }
    // Sort the ranges by the start of each range
    ranges.sort_by_key(|range| range.0);

    let mut merged_ranges = Vec::new();

    // Start with the first range
    let mut current_range = ranges[0];

    for &(start, end) in &ranges[1..] {
        // If the current range overlaps with or is adjacent to the next one
        if current_range.1 >= start {
            // Merge the ranges
            current_range.1 = current_range.1.max(end);
        } else {
            // Push the current range to the merged_ranges vector
            merged_ranges.push(current_range);
            // Move to the next range
            current_range = (start, end);
        }
    }

    // Push the last range
    merged_ranges.push(current_range);

    merged_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        let example_answer_part1 = "35";
        let result = run1(&File::open("src/day5/example_input.txt").unwrap());
        assert_eq!(result.unwrap(), example_answer_part1.to_string());
    }

    #[test]
    fn test_run2() {
        let example_answer_part2 = "46";
        let result = run2(&File::open("src/day5/example_input.txt").unwrap());
        assert_eq!(result.unwrap(), example_answer_part2.to_string());
    }

    #[test]
    fn test_merge_overlapping_ranges() {
        let mut ranges = vec![(1, 3), (2, 4), (5, 7), (6, 8), (10, 12), (11, 13)];
        let new_ranges = merge_overlapping_ranges(&mut ranges);
        assert_eq!(new_ranges, vec![(1, 4), (5, 8), (10, 13)]);

        let mut ranges = vec![(1, 3), (2, 4)];
        let new_ranges = merge_overlapping_ranges(&mut ranges);
        assert_eq!(new_ranges, vec![(1, 4)]);

        let mut ranges = vec![(1, 2), (2, 4)];
        let new_ranges = merge_overlapping_ranges(&mut ranges);
        assert_eq!(new_ranges, vec![(1, 4)]);

        let mut ranges = vec![(1, 2), (3, 4)];
        let new_ranges = merge_overlapping_ranges(&mut ranges);
        assert_eq!(new_ranges, vec![(1, 2), (3, 4)]);

        let mut ranges = vec![(1, 4), (2, 3)];
        let new_ranges = merge_overlapping_ranges(&mut ranges);
        assert_eq!(new_ranges, vec![(1, 4)]);

        let mut ranges = vec![
            (1, 2),
            (1, 2),
            (1, 2),
            (1, 2),
            (1, 2),
            (1, 2),
            (1, 2),
            (1, 2),
        ];
        let new_ranges = merge_overlapping_ranges(&mut ranges);
        assert_eq!(new_ranges, vec![(1, 2)]);

        let mut ranges = vec![(2, 3), (1, 4)];
        let new_ranges = merge_overlapping_ranges(&mut ranges);
        assert_eq!(new_ranges, vec![(1, 4)]);

        let mut ranges = Vec::new();
        let new_ranges = merge_overlapping_ranges(&mut ranges);
        assert_eq!(new_ranges, Vec::new());
    }
}
