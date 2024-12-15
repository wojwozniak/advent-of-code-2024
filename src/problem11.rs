use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

/*
   Problem 11 of 2024 Advent of Code
   https://adventofcode.com/2024/day/11

   First part -> just replace 75 with 25
   The main idea here is that we use a HashMap instead of an array. Order
   does not really matter, and using HashMap (pair of number, occurences) speeds
   up the function massively (not having to do 236 trillion operations in last loop
   is quite helpful). HashMap is best here because we still do a lot of operations
   so we take advantage of O(1) read/write
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

    let output = n_times_runner(numbers, 75);

    println!("{}", output);
    Ok(output as i64)
}

/// Function to run step of main n times -> reworked so it uses
/// a hashmap instead of vector -> this way we speed it up massively
fn n_times_runner(numbers: Vec<i64>, n: i32) -> i64 {
    let mut counts: HashMap<i64, i64> = HashMap::new();

    for num in numbers.iter() {
        *counts.entry(*num).or_insert(0) += 1;
    }

    for _ in 0..n {
        let mut new_counts: HashMap<i64, i64> = HashMap::new();

        for (&key, &value) in counts.iter() {
            if key == 0 {
                *new_counts.entry(1).or_insert(0) += value;
            } else if has_even_digits(key) {
                let (key1, key2) = split_number(key);
                *new_counts.entry(key1).or_insert(0) += value;
                *new_counts.entry(key2).or_insert(0) += value;
            } else {
                *new_counts.entry(key * 2024).or_insert(0) += value;
            }
        }

        counts = new_counts;
    }

    counts.values().sum()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probem11_01() {
        let numbers = vec![125, 17];
        let result = n_times_runner(numbers, 6);

        assert_eq!(result, 22);
    }

    #[test]
    fn test_probem11_02() {
        let numbers = vec![125, 17];
        let result = n_times_runner(numbers, 25);

        assert_eq!(result, 55312);
    }
}
