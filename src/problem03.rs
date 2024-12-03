use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
   Problem 03 of 2024 Advent of Code
   https://adventofcode.com/2024/day/3
*/
pub fn problem03() -> Result<i64, ()> {
    let path = "../inputs/input03.txt";

    // Open the file
    let file: File = File::open(&path).expect("Failed to open file");
    let reader: BufReader<File> = BufReader::new(file);

    // Regex pattern to match `mul(n1,n2)` statements
    // With two capture groups () to extract numbers easily
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Failed to compile regex");

    let mut sum_of_mults: i64 = 0;

    // Iterate over lines in the file
    for line in reader.lines() {
        let line = line.map_err(|_| ())?;

        // Find all matches of the pattern in the line
        for cap in re.captures_iter(&line) {
            let n1: i64 = cap[1].parse().map_err(|_| ())?;
            let n2: i64 = cap[2].parse().map_err(|_| ())?;

            // Add their product to the sum
            sum_of_mults += n1 * n2;
        }
    }

    println!("{}", sum_of_mults);
    Ok(sum_of_mults)
}
