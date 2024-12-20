use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
   Problem 05 of 2024 Advent of Code
   https://adventofcode.com/2024/day/5
*/
pub fn problem05() -> Result<(i32, i32), ()> {
    let (hashmap_with_sorted_arrays, pages_to_check) = get_data(false);
    let (sum, illegal_page_indexes) =
        calculate_sum_of_correct(&hashmap_with_sorted_arrays, &pages_to_check);

    let illegal_pages: Vec<Vec<i32>> = illegal_page_indexes
        .iter()
        .map(|&index| pages_to_check[index].clone())
        .collect();

    let fixed_sum = calculate_sum_of_fixed(&hashmap_with_sorted_arrays, &illegal_pages);

    println!("First sum: {}", sum);
    println!("Fixed sum: {}", fixed_sum);
    Ok((sum, fixed_sum))
}

/// Function calculating sum of rows that were found illegal in the first task by
/// fixing them. I assume there won't be rules that would create unfixable rows
/// (so f.e. 4 can't be before 5, 5 can't be before 4)
fn calculate_sum_of_fixed(
    hashmap_with_sorted_arrays: &HashMap<i32, Vec<i32>>,
    illegal_pages: &Vec<Vec<i32>>,
) -> i32 {
    let mut total_sum = 0;

    for row in illegal_pages {
        let mut array_so_far = Vec::new();

        for &current_value in row {
            if array_so_far.is_empty() {
                array_so_far.push(current_value);
                continue;
            }

            if let Some(forbidden_values) = hashmap_with_sorted_arrays.get(&current_value) {
                let mut insert_index = array_so_far.len();
                for &forbidden in forbidden_values {
                    if let Some(pos) = array_so_far.iter().position(|&x| x == forbidden) {
                        insert_index = insert_index.min(pos);
                    }
                }

                array_so_far.insert(insert_index, current_value);
            } else {
                array_so_far.push(current_value);
            }
        }

        if array_so_far.len() == row.len() {
            let middle_index = array_so_far.len() / 2;
            total_sum += array_so_far[middle_index];
        }
    }

    total_sum
}

/// Main part of the first part - calculating sum of middle values. We also get
/// a small help for task 2 -> we also extract indexes of illegal pages
fn calculate_sum_of_correct(
    hashmap_with_sorted_arrays: &HashMap<i32, Vec<i32>>,
    pages_to_check: &Vec<Vec<i32>>,
) -> (i32, Vec<usize>) {
    let mut sum: i32 = 0;
    let mut illegal_page_indexes = Vec::new();

    for (index, row) in pages_to_check.iter().enumerate() {
        let first_value = row[0];
        let mut checked_so_far = vec![first_value];

        for &value in &row[1..] {
            if let Some(array) = hashmap_with_sorted_arrays.get(&value) {
                if is_correct(&checked_so_far, array) {
                    checked_so_far.push(value);
                } else {
                    illegal_page_indexes.push(index);
                    break;
                }
            } else {
                checked_so_far.push(value);
            }
        }

        if checked_so_far.len() == row.len() {
            let middle_index = row.len() / 2;
            sum += row[middle_index];
        }
    }

    (sum, illegal_page_indexes)
}

/// Is_correct function checks if adding new value is forbidden by rules
fn is_correct(in_front: &[i32], illegal_values: &[i32]) -> bool {
    in_front.iter().all(|&num| !illegal_values.contains(&num))
}

/// Function that gets data for the task and returns it divided into two parts
/// First part is HashMap with vectors -> Values cannot appear in front of the key
/// Second part is vectors of vectors of i32
fn get_data(debug: bool) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let path = "../inputs/input05.txt";
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut first_type_lines: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut second_type_lines: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        if line.contains('|') {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                let key: i32 = parts[0].parse().unwrap_or(0);

                let new_values: Vec<i32> = parts[1]
                    .split(',')
                    .map(|s| s.trim().parse().unwrap_or(0))
                    .collect();

                first_type_lines.entry(key).or_default().extend(new_values);
                first_type_lines.get_mut(&key).unwrap().sort();
            }
        } else if !line.is_empty() {
            let values: Vec<i32> = line
                .split(',')
                .map(|s| s.trim().parse().unwrap_or(0))
                .collect();

            second_type_lines.push(values);
        }

        if debug {
            println!("{}", line);
        }
    }

    if debug {
        for (key, values) in &first_type_lines {
            println!("Key: {}, Values: {:?}", key, values);
        }

        println!("\nSecond Type Lines:");
        for line in &second_type_lines {
            println!("{:?}", line);
        }
    }

    (first_type_lines, second_type_lines)
}
