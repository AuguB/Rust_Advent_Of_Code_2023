use std::fs::File;
use std::path::Path;

mod day9;
#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let path = Path::new("src/day9/input.txt");
    let file = File::open(&path).unwrap();

    match day9::run1(&file) {
        Ok(result) => println!("Part 1 finished successfully: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }

    let file = File::open(&path).unwrap();
    match day9::run2(&file) {
        Ok(result) => println!("Part 2 finished successfully: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
