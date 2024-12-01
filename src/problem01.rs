use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

/*
   Problem 01 of 2024 Advent of Code
   https://adventofcode.com/2024/day/1
*/
pub fn problem01() -> Result<(i32, i32), ()> {
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

    // Calculate the answer to the first part
    let output_first: i32 = difference_sum(vec1.clone(), vec2.clone());
    println!("{}", output_first);

    // Second part of problem uses the same data, we just need
    // to calculate different thing.
    let output_second: i32 = similarity_score(vec1.clone(), vec2.clone());
    println!("{}", output_second);

    Ok((output_first, output_second))
}

// Calculate answer to the first part of the problem
fn difference_sum(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    let mut difference_sum: i32 = 0;

    for (a, b) in vec1.iter().zip(vec2.iter()) {
        let diff = a.abs_diff(*b) as i32;
        difference_sum += diff;
    }

    difference_sum
}

// Calculate answer to the second part of the problem
fn similarity_score(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    // We will use HashMap to make code as efficient as possible
    // Initialize hashMap and calculate frequencies
    let mut freq_map = HashMap::new();
    for &num in &vec2 {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    // Go over first vector and calculate similarity score
    let mut similarity_score = 0;

    for &num in &vec1 {
        if let Some(&freq) = freq_map.get(&num) {
            similarity_score += num * freq;
        }
    }

    similarity_score
}
