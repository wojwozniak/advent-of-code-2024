use std::fs::File;
use std::io::{BufRead, BufReader};

/*
   Problem 04 of 2024 Advent of Code
   https://adventofcode.com/2024/day/4
*/
pub fn problem04() -> Result<i64, ()> {
    let path = "../inputs/input04.txt";

    // Read the file and get the line length and table
    let table = read_file(path)?;

    // Find the number of "XMAS" occurrences in all directions
    let count = count_xmas(&table);

    println!("Total 'XMAS' occurrences: {}", count);
    Ok(count)
}

/// Reads the file and returns the line length and table of chars.
/// Returns Err(()) if the file is empty or lines are inconsistent.
fn read_file(path: &str) -> Result<Vec<Vec<char>>, ()> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut table: Vec<Vec<char>> = Vec::new();
    let mut line_length = 0;

    for (i, line_result) in reader.lines().enumerate() {
        let line = line_result.expect("Failed to read line");
        let chars: Vec<char> = line.chars().collect();

        if i == 0 {
            line_length = chars.len(); // Set expected length on the first line
        } else if chars.len() != line_length {
            eprintln!(
                "Inconsistent line length: expected {}, found {}",
                line_length,
                chars.len()
            );
            return Err(()); // Exit if lengths are inconsistent
        }

        table.push(chars);
    }

    if line_length == 0 {
        eprintln!("File is empty");
        return Err(());
    }

    Ok(table)
}

/// Counts occurrences of "XMAS" in all 8 directions.
fn count_xmas(table: &[Vec<char>]) -> i64 {
    let directions = [
        (-1, 0),  // North
        (-1, 1),  // North-East
        (0, 1),   // East
        (1, 1),   // South-East
        (1, 0),   // South
        (1, -1),  // South-West
        (0, -1),  // West
        (-1, -1), // North-West
    ];

    let target = ['X', 'M', 'A', 'S'];
    let mut count = 0;

    for row in 0..table.len() {
        for col in 0..table[row].len() {
            for &(row_offset, col_offset) in &directions {
                if matches_target(row, col, &target, table, row_offset, col_offset) {
                    count += 1;
                }
            }
        }
    }

    count
}

/// Checks if the target string is found starting at (row, col) in the given direction.
fn matches_target(
    start_row: usize,
    start_col: usize,
    target: &[char],
    table: &[Vec<char>],
    row_offset: isize,
    col_offset: isize,
) -> bool {
    let mut r = start_row as isize;
    let mut c = start_col as isize;

    for &ch in target {
        // Ensure within bounds
        if r < 0 || c < 0 || r >= table.len() as isize || c >= table[0].len() as isize {
            return false;
        }

        // Check if the character matches
        if table[r as usize][c as usize] != ch {
            return false;
        }

        // Move to the next position in the direction
        r += row_offset;
        c += col_offset;
    }

    true
}
