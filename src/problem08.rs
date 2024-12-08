use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

/*
   Problem 08 of 2024 Advent of Code
   https://adventofcode.com/2024/day/8
*/
pub fn problem08() -> Result<(i64, i64), ()> {
    let path = "../inputs/input08.txt";
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut data: Vec<Vec<char>> = Vec::new();
    let mut line_count = 0;
    let mut max_line_length = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        line_count += 1;
        max_line_length = max_line_length.max(line.len());

        data.push(line.chars().collect());
    }

    let antennas = parse_antennas(&data);
    let antinodes = calculate_antinodes(&antennas, max_line_length as i32, line_count as i32);
    let antinodes_limitless =
        calculate_antinodes_limitless(&antennas, max_line_length as i32, line_count as i32);

    println!("{}, {}", antinodes.len(), antinodes_limitless.len());
    Ok((antinodes.len() as i64, antinodes_limitless.len() as i64))
}

/// Function to calculate the number of antinodes with no distance limit
pub fn calculate_antinodes_limitless(
    antennas: &Vec<(i32, i32, char)>,
    width: i32,
    height: i32,
) -> HashSet<(i32, i32)> {
    let mut antinodes = HashSet::new();

    // Group antennas by frequency
    let mut freq_groups: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for &(x, y, freq) in antennas {
        freq_groups.entry(freq).or_default().push((x, y));
    }

    // Iterate over frequency groups and calculate antinodes
    for group in freq_groups.values() {
        for i in 0..group.len() - 1 {
            for j in i + 1..group.len() {
                let (x1, y1) = group[i];
                let (x2, y2) = group[j];

                // Calculate the vector between the two antennas
                let dx = x2 - x1;
                let dy = y2 - y1;

                // Calculate antinode positions starting from each antenna
                // (with ugly fix to allow antinodes on startpoints)
                let mut antinode1 = (x1 + dx, y1 + dy);
                let mut antinode2 = (x2 - dx, y2 - dy);

                // Keep adding antinodes along the vector direction
                loop {
                    antinode1 = (antinode1.0 - dx, antinode1.1 - dy);
                    antinode2 = (antinode2.0 + dx, antinode2.1 + dy);

                    if check_and_add(antinode1.0, antinode1.1, width, height) {
                        antinodes.insert(antinode1);
                    }
                    if check_and_add(antinode2.0, antinode2.1, width, height) {
                        antinodes.insert(antinode2);
                    }

                    // Break the loop if both antinodes are out of bounds
                    if !check_and_add(antinode1.0 - dx, antinode1.1 - dy, width, height)
                        && !check_and_add(antinode2.0 + dx, antinode2.1 + dy, width, height)
                    {
                        break;
                    }
                }
            }
        }
    }

    antinodes
}

/// Function to calculate the number of antinodes
pub fn calculate_antinodes(
    antennas: &Vec<(i32, i32, char)>,
    width: i32,
    height: i32,
) -> HashSet<(i32, i32)> {
    let mut antinodes = HashSet::new();

    // Group antennas by frequency
    let mut freq_groups: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for &(x, y, freq) in antennas {
        freq_groups.entry(freq).or_default().push((x, y));
    }

    // Iterate over frequency groups and calculate antinodes
    for group in freq_groups.values() {
        for i in 0..group.len() - 1 {
            for j in i + 1..group.len() {
                let (x1, y1) = group[i];
                let (x2, y2) = group[j];

                // Calculate the vector between the two antennas
                let dx = x2 - x1;
                let dy = y2 - y1;

                // Calculate the two possible antinode positions
                let antinode1 = (x1 - dx, y1 - dy);
                let antinode2 = (x2 + dx, y2 + dy);

                // Check and add the antinodes if they are within bounds
                if check_and_add(antinode1.0, antinode1.1, width, height) {
                    antinodes.insert(antinode1);
                }
                if check_and_add(antinode2.0, antinode2.1, width, height) {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    antinodes
}

/// Auxiliary function checking correctness of position
fn check_and_add(x: i32, y: i32, width: i32, height: i32) -> bool {
    x >= 0 && x < width && y >= 0 && y < height
}

/// Auxiliary function returning vectors with positions and char of antennas
fn parse_antennas(data: &[Vec<char>]) -> Vec<(i32, i32, char)> {
    let mut antennas = vec![];

    for (y, row) in data.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch != '.' && ch != ' ' {
                antennas.push((x as i32, y as i32, ch));
            }
        }
    }

    antennas
}
