use std::fs::File;
use std::io::{self, BufRead};

/*
   Problem 11 of 2024 Advent of Code
   https://adventofcode.com/2024/day/11
*/
pub fn problem11() -> Result<i64, ()> {
    let path = "../inputs/input11.txt";

    let file: File = File::open(&path).unwrap();
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut numbers: Vec<i64> = Vec::new();

    for line in reader.lines() {
        let line: String = line.unwrap();

        numbers = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
    }

    let output = n_times_runner(numbers, 5);

    println!("{}", output);
    Ok(output as i64)
}

/// Function to run step of main n timess
fn n_times_runner(mut numbers: Vec<i64>, n: i32) -> i64 {
    for _ in 0..n {
        //println!("{:?}", numbers);
        numbers = process_numbers(numbers);
        println!("{:?}", numbers);
    }
    numbers.len() as i64
}

/// Single step of first part of task
fn process_numbers(mut numbers: Vec<i64>) -> Vec<i64> {
    let mut i = 0;
    while i < numbers.len() {
        if numbers[i] == 0 {
            numbers[i] = 1;
            i += 1;
        } else if has_even_digits(numbers[i]) {
            let (first, second) = split_number(numbers[i]);

            numbers.remove(i);
            numbers.insert(i, second);
            numbers.insert(i, first);

            i += 2;
        } else {
            numbers[i] *= 2024;
            i += 1;
        }
    }
    numbers
}

/// Auxilliary spliting function
fn split_number(num: i64) -> (i64, i64) {
    let num_str = num.to_string();
    let len = num_str.len();
    let mid = len / 2;

    let first_half = &num_str[..mid];
    let second_half = &num_str[mid..];

    let left = first_half.parse::<i64>().unwrap_or(0);
    let right = second_half.parse::<i64>().unwrap_or(0);

    (left, right)
}

/// Auxiliary function checking if given number has even number of digits
fn has_even_digits(num: i64) -> bool {
    let digit_count = (num.abs() as f64).log10().floor() as u32 + 1;
    digit_count % 2 == 0
}
