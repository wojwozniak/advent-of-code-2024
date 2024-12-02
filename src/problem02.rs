use std::fs::File;
use std::io::{self, BufRead};

pub fn problem02() -> Result<i32, ()> {
    let path = "../inputs/input02.txt";

    // Open the file
    let file: File = File::open(&path).expect("Failed to open file");
    let reader: io::BufReader<File> = io::BufReader::new(file);

    // Store amount of safe reports
    let mut safe_reports: i32 = 0;

    // Process each line
    for line in reader.lines() {
        let line: String = line.expect("Failed to read line");

        // Parse numbers from the line
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        // Print input values
        // println!("Numbers in line: {:?}", numbers);

        safe_reports += if is_strictly_inc_or_dec(numbers) {
            1
        } else {
            0
        };
    }

    println!("{}", safe_reports);
    Ok(safe_reports)
}

// Function checking task requirement on every line
fn is_strictly_inc_or_dec(numbers: Vec<i32>) -> bool {
    if numbers.len() < 2 {
        return true;
    }

    // We determine if we got a increasing or decreasing trend.
    // Then check for correct difference on every 2 item window
    let (first, second) = (numbers[0], numbers[1]);
    if second > first {
        for window in numbers.windows(2) {
            let (prev, curr) = (window[0], window[1]);
            if !correct_difference(prev, curr) {
                return false;
            }
        }
    } else {
        for window in numbers.windows(2) {
            let (prev, curr) = (window[0], window[1]);
            if !correct_difference(curr, prev) {
                return false;
            }
        }
    }

    true
}

// Function checks for difference of 1, 2, 3 on given pair. We don't check for
// negative values, just provide values in correct order
fn correct_difference(smaller: i32, bigger: i32) -> bool {
    let diff = bigger - smaller;
    if diff > 0 && diff < 4 {
        return true;
    }
    false
}
