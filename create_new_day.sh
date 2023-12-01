#!/bin/bash

cd "/home/stijn/Projects/Rust_Advent_Of_Code_2023/advent_of_code/src"
# Get the day number from the first command line argument
day_number=$1

# Create the directory
mkdir "day$day_number"

# Create the input.txt, instructions.txt, and mod.rs files
touch "day$day_number/input.txt"
touch "day$day_number/instructions.txt"
echo "#[allow(dead_code)]" > "day$day_number/mod.rs"
echo "pub fn run() -> Result<(), Box<dyn std::error::Error>> {" >> "day$day_number/mod.rs"
echo "    Ok(())" >> "day$day_number/mod.rs"
echo "}" >> "day$day_number/mod.rs"
echo "

" >> "day$day_number/mod.rs"
echo "#[cfg(test)]" >> "day$day_number/mod.rs"
echo "mod tests {" >> "day$day_number/mod.rs"
echo "    use super::*;" >> "day$day_number/mod.rs"
echo "    #[test]" >> "day$day_number/mod.rs"
echo "    fn test_run() {" >> "day$day_number/mod.rs"
echo "        let result = run();" >> "day$day_number/mod.rs"
echo "        assert!(result.is_ok());" >> "day$day_number/mod.rs"
echo "    }" >> "day$day_number/mod.rs"
echo "}" >> "day$day_number/mod.rs"

# Add the new day to the lib.rs file
echo "pub mod day$day_number;" >> "lib.rs"