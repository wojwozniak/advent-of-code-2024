use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
pub struct CellData {
    cell: i32,
    content: i32,
}

impl CellData {
    pub fn new(cell: i32, content: i32) -> Self {
        Self { cell, content }
    }
}

/*
   Problem 09 of 2024 Advent of Code
   https://adventofcode.com/2024/day/9
*/
pub fn problem09() -> Result<i64, ()> {
    let path = "../inputs/input09.txt";

    // Vec<CellData> where it has cell:i32, content:i32
    let vector = get_cell_data(path);
    let optimized_vector = optimize_storage(vector);
    let checksum: i64 = calculate_checksum(optimized_vector);

    println!("{}", checksum);
    Ok(checksum)
}

/// Calculating checksum for optimized file
fn calculate_checksum(vector: Vec<CellData>) -> i64 {
    // We know checksum from first file is 0, so i start with position 1
    let mut position: i64 = 0;
    let mut sum: i64 = 0;

    for cell in vector.iter() {
        // If content is -1, immediately return current sum
        if cell.content == -1 {
            return sum;
        }
        sum += cell.content as i64 * position;
        position += 1;
    }

    sum
}

/// Reads data from the specified path and returns a vector of `CellData`.
pub fn get_cell_data(path: &str) -> Vec<CellData> {
    let file = File::open(path).unwrap();
    let mut reader = io::BufReader::new(file);

    // Read only line
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    // Store result using the auxiliary class
    let mut result: Vec<CellData> = Vec::new();

    // Auxiliary variables
    let mut line_index = 1;
    let mut line_type = 0;
    let mut cell = 0;

    for chunk in line.trim().chars() {
        if let Some(digit) = chunk.to_digit(10) {
            // Empty lines
            if line_index % 2 == 0 {
                for _ in 0..digit {
                    result.push(CellData::new(cell, -1));
                    cell += 1;
                }
            } else {
                for _ in 0..digit {
                    result.push(CellData::new(cell, line_type));
                    cell += 1;
                }
                line_type += 1;
            }
            line_index += 1;
        }
    }

    result
}

/// Two-pointer optimize_storage function
/// We change cell organization too (to keep more data for possible use in part 2)
fn optimize_storage(mut vector: Vec<CellData>) -> Vec<CellData> {
    if vector.is_empty() {
        return vector;
    }

    let mut left = 0;
    let mut right = vector.len() - 1;

    while left < right {
        // Find first zero content cell from the left
        while left < right && vector[left].content != -1 {
            left += 1;
        }

        // Find last non-zero content cell from the right
        while left < right && vector[right].content == -1 {
            right -= 1;
        }

        // If we found a zero content cell and a non-zero content cell, swap them
        if left < right {
            vector.swap(left, right);
        }
    }

    vector
}
