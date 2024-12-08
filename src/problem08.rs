use std::fs::File;
use std::io::{self, BufRead};

/*
   Problem 08 of 2024 Advent of Code
   https://adventofcode.com/2024/day/8
*/
pub fn problem08() -> Result<i64, ()> {
    let path = "../inputs/input08.txt";

    let file: File = File::open(&path).unwrap();
    let reader: io::BufReader<File> = io::BufReader::new(file);

    for line in reader.lines() {
        let line: String = line.unwrap();
    }

    println!("{}", 0);
    Ok(0)
}
