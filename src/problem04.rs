use std::fs::File;
use std::io::{BufRead, BufReader};

/*
   Problem 04 of 2024 Advent of Code
   https://adventofcode.com/2024/day/4
*/
pub fn problem04() -> Result<i64, ()> {
    let path = "../inputs/input04.txt";

    // Read the file and get the line length and table
    let (line_length, table) = read_file(path)?;

    println!("All lines are {} characters long", line_length);

    // Find the number of "XMAS" occurrences in all directions
    let xmas_count = count_xmas(&table);
    println!("Total 'XMAS' occurrences: {}", xmas_count);

    // Find the number of "MAS" squares
    let mas_square_count = count_mas_squares(&table);
    println!("Total 'MAS' square patterns: {}", mas_square_count);

    Ok(xmas_count + mas_square_count)
}

/// Reads the file and returns the line length and table of chars.
/// Returns Err(()) if the file is empty or lines are inconsistent.
fn read_file(path: &str) -> Result<(usize, Vec<Vec<char>>), ()> {
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

    Ok((line_length, table))
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

/// Counts occurrences of "MAS" square patterns in 3x3 subgrids.
fn count_mas_squares(table: &[Vec<char>]) -> i64 {
    let mut count = 0;

    // Iterate over all possible corners of 3x3 squares
    for row in 0..(table.len() - 2) {
        for col in 0..(table[row].len() - 2) {
            if is_mas_square(row, col, table) {
                count += 1;
            }
        }
    }

    count
}

/// Checks if a 3x3 square starting at (row, col) forms a "MAS" pattern.
fn is_mas_square(row: usize, col: usize, table: &[Vec<char>]) -> bool {
    // Check if the center position contains 'A'
    if table[row + 1][col + 1] != 'A' {
        return false;
    }

    let top_left = table[row][col];
    let top_right = table[row][col + 2];
    let bottom_left = table[row + 2][col];
    let bottom_right = table[row + 2][col + 2];

    if top_left == 'M' && bottom_right == 'S' && top_right == 'M' && bottom_left == 'S' {
        return true;
    }
    if top_left == 'M' && bottom_right == 'S' && top_right == 'S' && bottom_left == 'M' {
        return true;
    }
    if top_left == 'S' && bottom_right == 'M' && top_right == 'M' && bottom_left == 'S' {
        return true;
    }
    if top_left == 'S' && bottom_right == 'M' && top_right == 'S' && bottom_left == 'M' {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_mas_squares() {
        let table: Vec<Vec<char>> = vec![
            ".M.S......".chars().collect(),
            "..A..MSMS.".chars().collect(),
            ".M.S.MAA..".chars().collect(),
            "..A.ASMSM.".chars().collect(),
            ".M.S.M....".chars().collect(),
            "..........".chars().collect(),
            "S.S.S.S.S.".chars().collect(),
            ".A.A.A.A..".chars().collect(),
            "M.M.M.M.M.".chars().collect(),
            "..........".chars().collect(),
        ];

        let result = count_mas_squares(&table);

        assert_eq!(result, 9);
    }
}
