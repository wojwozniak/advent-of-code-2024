use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
   Problem 03 of 2024 Advent of Code
   https://adventofcode.com/2024/day/3
*/

// Function to process the mul() statements and return their sum
fn calculate_mul_sum(lines: Vec<String>, re: &Regex) -> i64 {
    let mut sum_of_mults = 0;

    // Iterate over the subset of lines passed to this function
    for line in lines {
        for cap in re.captures_iter(&line) {
            let n1: i64 = cap[1].parse().unwrap();
            let n2: i64 = cap[2].parse().unwrap();

            // Add their product to the sum
            sum_of_mults += n1 * n2;
        }
    }

    sum_of_mults
}

pub fn problem03() -> Result<i64, ()> {
    let path = "../inputs/input03.txt";

    // Open the file
    let file: File = File::open(&path).expect("Failed to open file");
    let reader: BufReader<File> = BufReader::new(file);

    // Regex pattern to match `mul(n1,n2)` statements
    // with two capture groups to extract n1 and n2
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Failed to compile regex");

    let mut total_sum = 0;
    let mut current_chars = String::new();
    let mut is_do_enabled = true; // do() is enabled by deafult

    // Iterate over lines in the file
    for line in reader.lines() {
        let line = line.map_err(|_| ())?;

        // Iterate over each character in the line
        for ch in line.chars() {
            current_chars.push(ch); // Accumulate characters

            // Check for "don't()" (last 7 chars)
            if current_chars.len() >= 7 && &current_chars[current_chars.len() - 7..] == "don't()" {
                // Check to enable multiple do() or don't() in a row
                if is_do_enabled {
                    // Process the block before don't()
                    total_sum += calculate_mul_sum(vec![current_chars.clone()], &re);
                    current_chars.clear(); // Clear collected characters
                }
                is_do_enabled = false; // Disable summing after don't()
            }

            // Check for "do()" (last 4 chars)
            if current_chars.len() >= 4 && &current_chars[current_chars.len() - 4..] == "do()" {
                if !is_do_enabled {
                    current_chars.clear(); // Clear collected characters when we encounter do()
                }
                is_do_enabled = true; // Enable summing after do()
            }
        }
    }

    // Add chars accumulated at the end
    if is_do_enabled {
        total_sum += calculate_mul_sum(vec![current_chars.clone()], &re);
    }

    // Return the final sum
    println!("{}", total_sum);
    Ok(total_sum)
}
