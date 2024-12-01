use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Path to the input file
    let path = "src/input.txt";

    // Open the file and unwrap the Result
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line?; // Unwrap each line
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        left_list.push(numbers[0]);
        right_list.push(numbers[1]);
    }

    // Sort both lists
    left_list.sort();
    right_list.sort();

    // Calculate the total distance
    let total_distance: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    println!("The total distance is: {}", total_distance);
    Ok(())
}

