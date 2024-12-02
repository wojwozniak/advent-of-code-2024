use std::fs::File;
use std::io::{self, BufRead};

pub fn problem02() -> Result<(i32, i32), ()> {
    let path = "../inputs/input02.txt";

    // Open the file
    let file: File = File::open(&path).expect("Failed to open file");
    let reader: io::BufReader<File> = io::BufReader::new(file);

    // Store amount of safe reports (for first part, no tolerance)
    let mut safe_reports: i32 = 0;

    // Store answer for second part, where at most one error can be tolerated
    let mut safe_reports_dampered: i32 = 0;

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

        safe_reports += if is_strictly_inc_or_dec(&numbers, false) {
            1
        } else {
            0
        };

        safe_reports_dampered += if is_strictly_inc_or_dec(&numbers, true) {
            1
        } else {
            0
        };
    }

    println!("{}", safe_reports);
    println!("{}", safe_reports_dampered);
    Ok((safe_reports, safe_reports_dampered))
}

// Function checking strict increasing or decreasing (by step of 1 - 3)
fn is_strictly_inc_or_dec(numbers: &Vec<i32>, tolerate_error: bool) -> bool {
    if numbers.len() < 2 {
        return true;
    }

    // Determine the trend
    let increasing = numbers[1] > numbers[0];

    for window in numbers.windows(2) {
        let (prev, curr) = (window[0], window[1]);

        // Check difference validity
        let is_valid = if increasing {
            correct_difference(prev, curr)
        } else {
            correct_difference(curr, prev)
        };

        if !is_valid {
            if tolerate_error {
                // Attempt to remove the offending item and call the function recursively
                for i in 0..numbers.len() {
                    let mut new_numbers = numbers.clone();
                    new_numbers.remove(i);
                    if is_strictly_inc_or_dec(&new_numbers, false) {
                        return true; // Return true if the modified array is valid
                    }
                }
                return false;
            } else {
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
