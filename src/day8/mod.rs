use std::arch::aarch64::int16x8x4_t;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use num::integer::lcm;

#[allow(dead_code)]
#[allow(unused_imports)]
pub fn run1(file: &File) -> Result<String, Box<dyn std::error::Error>> {
    // construct a reader
    let _reader: io::BufReader<&File> = io::BufReader::new(file);
    // Construct an iterator over the lines
    let mut lines = _reader.lines();
    // Get the first line (the instructions) and convert it to a vector of characters
    let instructions: Vec<char> = lines.next().unwrap()?.chars().collect();
    // Skip the second line (it is empty)
    lines.next(); // skip the second line
                  // Create an empty hashmap to store the nodes, and their left and right neighbors
    let mut node_map: HashMap<String, (String, String)> = HashMap::new();
    // Iterate over the remaining lines and populate the hashmap
    for line in lines {
        // Unwrap the line (it might be an error)
        let line_content = line?;
        // Split the line into the node and its neighbors
        let split1: Vec<&str> = line_content.split(" = ").collect::<Vec<&str>>();
        // Get the node (it is the first element of the vector)
        let node: &str = split1[0];
        // Strip the brackets of the rest, and split it along the comma to get the left and right neighbors
        let split2: Vec<&str> = split1[1][1..split1[1].len() - 1]
            .split(", ")
            .collect::<Vec<&str>>();
        // Insert the node and its neigbors (convert them to String type first)
        node_map.insert(
            node.to_string(),
            (split2[0].to_string(), split2[1].to_string()),
        );
    }
    // Main algorithm
    // Set the current node to AAA
    let mut current_node: &str = "AAA";
    // Set the current instruction to none
    let mut current_instruction: char;
    // Set the current step to 0
    let mut step: usize = 0;
    // Define terminal condition
    while current_node != "ZZZ" {
        // Get the neighbors of the current node from the map
        let (left, right) = node_map.get(current_node).unwrap();
        // Get the current instruction from the list
        current_instruction = instructions[step % instructions.len()];
        // Move to the neighbor according to the instruction
        current_node = match current_instruction {
            'L' => left,
            'R' => right,
            _ => panic!("Invalid instruction"),
        };
        // Increment step counter
        step += 1;
    }
    // Return result
    Ok(step.to_string())
}

pub fn run2(file: &File) -> Result<String, Box<dyn std::error::Error>> {
    // construct a reader
    let _reader: io::BufReader<&File> = io::BufReader::new(file);
    // Construct an iterator over the lines
    let mut lines = _reader.lines();
    // Get the first line (the instructions) and convert it to a vector of characters
    let instructions: Vec<char> = lines.next().unwrap()?.chars().collect();
    // Skip the second line (it is empty)
    lines.next(); // skip the second line
                  // Create an empty hashmap to store the nodes, and their left and right neighbors
    let mut node_map: HashMap<String, (String, String)> = HashMap::new();
    // Create an empty vector to store the nodes at which the ghosts currently are
    let mut ghosts: Vec<String> = Vec::new();
    // Iterate over the remaining lines and populate the hashmap
    for line in lines {
        // Unwrap the line (it might be an error)
        let line_content = line?;
        // Split the line into the node and its neighbors
        let split1: Vec<&str> = line_content.split(" = ").collect::<Vec<&str>>();
        // Get the node (it is the first element of the vector)
        let node: &str = split1[0];
        // If the node ends with A, add it to the ghosts list
        if node.ends_with('A') {
            ghosts.push(node.to_string());
        }
        // Strip the brackets of the rest, and split it along the comma to get the left and right neighbors
        let split2: Vec<&str> = split1[1][1..split1[1].len() - 1]
            .split(", ")
            .collect::<Vec<&str>>();
        // Insert the node and its neigbors (convert them to String type first)
        node_map.insert(
            node.to_string(),
            (split2[0].to_string(), split2[1].to_string()),
        );
    }
    // Main algorithm
    // Create an empty list with the cycle length for each ghost
    let mut cycle_lengths: Vec<usize> = vec![];
    for ghost in ghosts.iter(){
                // Set the current step to 0
        let mut step: usize = 0;
        let mut this_ghost = ghost.clone();
        while !this_ghost.ends_with('Z'){
            this_ghost = get_correct_neighbor(instructions[step%instructions.len()], node_map.get(&this_ghost).unwrap()).to_string();
            step+=1;
        }
        cycle_lengths.push(step);
    }
    let lcm: usize = cycle_lengths.iter().cloned().reduce(|u, v| lcm(u, v)).unwrap();
    return Ok(lcm.to_string());

}

fn get_correct_neighbor<'a>(instruction:char, neighbors:&'a(String, String)) -> &'a str{
    match instruction {
        'L' => &neighbors.0,
        'R' => &neighbors.1,
        _ => panic!("Invalid instruction"),
    }
}


fn all_ghosts_are_done(ghosts: &Vec<String>) -> bool {
    ghosts.iter().all(|x| x.ends_with('Z'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        let example_answer_part1: &str = "8";
        let result: Result<String, Box<dyn Error>> =
            run1(&File::open("src/day8/example_input.txt").unwrap());
        assert_eq!(result.unwrap(), example_answer_part1.to_string());
    }

    #[test]
    fn test_run2() {
        let example_answer_part2: &str = "";
        let result: Result<String, Box<dyn Error>> =
            run2(&File::open("src/day8/example_input.txt").unwrap());
        assert_eq!(result.unwrap(), example_answer_part2.to_string());
    }
}
