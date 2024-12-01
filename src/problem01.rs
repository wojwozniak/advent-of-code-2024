use std::fs::File;
use std::io::{self, BufRead};

pub fn problem01() -> Result<(), ()> {
    let path = "inputs/input01.txt";

    let file: File = File::open(&path).unwrap();
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line: String = line.unwrap();
        //println!("{}", line);

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

    vec1.sort_unstable();
    vec2.sort_unstable();

    for (a, b) in vec1.iter().zip(vec2.iter()) {
        println!("({}, {})", a, b);
    }

    Ok(())
}
