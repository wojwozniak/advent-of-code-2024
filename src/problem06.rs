use std::fs::File;
use std::io::{BufRead, BufReader};

/*
   Problem 06 of 2024 Advent of Code
   https://adventofcode.com/2024/day/6
*/
pub fn problem06() -> Result<i64, ()> {
    let path = "../inputs/input06.txt";

    // Open the file
    let file: File = File::open(&path).expect("Failed to open file");
    let reader: BufReader<File> = BufReader::new(file);

    Ok(0)
}
