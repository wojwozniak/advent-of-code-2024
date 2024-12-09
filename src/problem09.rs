use std::fs::File;
use std::io::BufReader;

/*
   Problem 09 of 2024 Advent of Code
   https://adventofcode.com/2024/day/9
*/
pub fn problem09() -> Result<i64, ()> {
    let path = "../inputs/input03.txt";

    // Open the file
    let file: File = File::open(&path).expect("Failed to open file");
    let reader: BufReader<File> = BufReader::new(file);

    Ok(0)
}
