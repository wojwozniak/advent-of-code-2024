use std::fs::File;
use std::io::{self, BufRead};

/*
   Problem 01 of 2024 advent of code
   https://adventofcode.com/2024/day/1
*/
pub fn problem01() -> Result<i32, ()> {
    // Read file
    let path = "../inputs/input01.txt";

    let file: File = File::open(&path).unwrap();
    let reader: io::BufReader<File> = io::BufReader::new(file);

    // Initialize vectors to store read numbers
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    // Read lines, cast into numbers, push to vectors
    for line in reader.lines() {
        let line: String = line.unwrap();

        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if numbers.len() == 2 {
            vec1.push(numbers[0]);
            vec2.push(numbers[1]);
        } else {
            eprintln!("Invalid line: {}", line);
        }
    }

    // Sort vectors
    vec1.sort_unstable();
    vec2.sort_unstable();

    // Calculate the answer
    let mut difference_sum: i32 = 0;

    for (a, b) in vec1.iter().zip(vec2.iter()) {
        let diff = a.abs_diff(*b) as i32;
        difference_sum += diff;
    }

    println!("{}", difference_sum);
    Ok(difference_sum)
}
