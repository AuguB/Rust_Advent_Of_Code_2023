use std::fs::File;
use std::path::Path;

mod day7;
#[allow(dead_code)]
#[allow(unused_imports)]
fn main() {
    let path = Path::new("src/day7/input.txt");
    let file = File::open(&path).unwrap();

    match day7::run1(&file) {
        Ok(result) => println!("Part 1 finished successfully: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }

    let file = File::open(&path).unwrap();
    match day7::run2(&file) {
        Ok(result) => println!("Part 2 finished successfully: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
