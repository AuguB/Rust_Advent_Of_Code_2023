#!/bin/bash

# Create the main directory
mkdir advent_of_code
cd advent_of_code

# Create the Cargo.toml file
echo '[package]
name = "advent_of_code"
version = "0.1.0"
edition = "2018"

[dependencies]' > Cargo.toml

# Create the src directory and the lib.rs file
mkdir src
echo "" > src/lib.rs

# Create the tests directory
mkdir tests

# Create the files for each day
for i in {1..25}
do
   echo "mod day$i;" >> src/lib.rs
   echo "pub fn run() {
    // Your code here
}" > src/day$i.rs
   echo "use advent_of_code::day$i;

#[test]
fn test_run() {
    day$i::run();
    // Add assertions here
}" > tests/day${i}_tests.rs
done