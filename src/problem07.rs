use std::fs::File;
use std::io::{self, BufRead};

/*
   Problem 07 of 2024 Advent of Code
   https://adventofcode.com/2024/day/7
*/
pub fn problem07() -> Result<i64, ()> {
    let path = "../inputs/input07.txt";

    let file: File = File::open(&path).unwrap();
    let reader: io::BufReader<File> = io::BufReader::new(file);

    for line in reader.lines() {
        let line: String = line.unwrap();
    }

    Ok(0)
}
