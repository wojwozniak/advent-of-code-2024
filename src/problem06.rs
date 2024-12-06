use std::fs::File;
use std::io::{BufRead, BufReader};

/*
   Problem 06 of 2024 Advent of Code
   https://adventofcode.com/2024/day/6
*/
pub fn problem06() -> Result<i64, String> {
    let path = "../inputs/input06.txt";
    let (data, _) = get_data(path)?;

    let (start_r, start_c, start_dir) = find_start(&data);
    let visited_count = calculate_visited_count(&data, start_r, start_c, start_dir);

    println!("{}", visited_count);
    Ok(visited_count)
}

/// Main task of first part -> calculating amount of visited fields
fn calculate_visited_count(
    data: &Vec<Vec<char>>,
    start_r: usize,
    start_c: usize,
    start_dir: char,
) -> i64 {
    let rows = data.len() as i32;
    let cols = data[0].len() as i32;

    let mut r = start_r as i32;
    let mut c = start_c as i32;
    let mut delta = dir_to_delta(start_dir);

    let mut visited_count: i64 = 0;
    let mut visited = vec![vec![false; cols as usize]; rows as usize];

    loop {
        let curr_r = r as usize;
        let curr_c = c as usize;

        if !visited[curr_r][curr_c] {
            visited[curr_r][curr_c] = true;
            visited_count += 1;
        }

        let next_r = r + delta.0;
        let next_c = c + delta.1;

        // Check if next position is out of bounds, end loop
        if next_r < 0 || next_r >= rows || next_c < 0 || next_c >= cols {
            break;
        }

        // Check if next cell is blocked
        if data[next_r as usize][next_c as usize] == '#' {
            delta = rotate_delta(delta);
            continue;
        }

        // Move to next position
        r = next_r;
        c = next_c;
    }

    visited_count
}

/// Auxiliary function to rotate movement delta
fn rotate_delta(delta: (i32, i32)) -> (i32, i32) {
    let (dr, dc) = delta;
    match (dr, dc) {
        (-1, 0) => (0, 1),  // up -> right
        (0, 1) => (1, 0),   // right -> down
        (1, 0) => (0, -1),  // down -> left
        (0, -1) => (-1, 0), // left -> up
        _ => delta,         // invalid or no change
    }
}

/// Function finding starting direction in our grid
fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize, char) {
    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if "^>v<".contains(ch) {
                return (r, c, ch);
            }
        }
    }
    panic!("No starting direction found");
}

/// Get delta of starting direction
fn dir_to_delta(dir: char) -> (i32, i32) {
    match dir {
        '^' => (-1, 0), // up
        '>' => (0, 1),  // right
        'v' => (1, 0),  // down
        '<' => (0, -1), // left
        _ => (0, 0),    // invalid
    }
}

/// Auxiliary function getting and parsing data
fn get_data(path: &str) -> Result<(Vec<Vec<char>>, usize), String> {
    let file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader = BufReader::new(file);

    let lines: Vec<Vec<char>> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    if lines.is_empty() {
        return Err("No lines collected".to_string());
    }

    let line_length = lines[0].len();
    let consistent_length = lines.iter().all(|line| line.len() == line_length);

    if !consistent_length {
        return Err("Inconsistent line lengths".to_string());
    }

    Ok((lines, line_length))
}
