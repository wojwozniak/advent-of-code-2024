use std::fs::File;
use std::io::{self, BufRead};

/*
   Problem 07 of 2024 Advent of Code
   https://adventofcode.com/2024/day/7
*/
pub fn problem07() -> Result<(i64, i64), ()> {
    let path = "../inputs/input07.txt";

    let file: File = File::open(&path).unwrap();
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut sum: i64 = 0;
    let mut sum_expanded: i64 = 0;

    for line in reader.lines() {
        let line: String = line.unwrap();
        if let Some((left, right)) = line.split_once(":") {
            let target: i64 = left.trim().parse().unwrap();
            let numbers: Vec<i64> = right
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();

            // Part 1 of task
            if check_if_possible(target, numbers.clone(), false) {
                sum += target;
            }

            // Part 2 of task
            if check_if_possible(target, numbers, true) {
                sum_expanded += target;
            }
        }
    }

    println!("{} {}", sum, sum_expanded);
    Ok((sum, sum_expanded))
}

/// Check if it is possible to create target with numbers using + and *
/// using depth first search
fn check_if_possible(target: i64, numbers: Vec<i64>, expanded: bool) -> bool {
    fn dfs(index: usize, current_value: i64, numbers: &[i64], target: i64, expanded: bool) -> bool {
        if index == numbers.len() {
            return current_value == target;
        }

        let next_number = numbers[index];

        if dfs(
            index + 1,
            current_value + next_number,
            numbers,
            target,
            expanded,
        ) {
            return true;
        }

        if dfs(
            index + 1,
            current_value * next_number,
            numbers,
            target,
            expanded,
        ) {
            return true;
        }

        // Part 2 of the task, checking || operator
        if expanded {
            if dfs(
                index + 1,
                merge_numbers_as_strings(current_value, next_number),
                numbers,
                target,
                expanded,
            ) {
                return true;
            }
        }

        false
    }

    if numbers.is_empty() {
        return false;
    }

    dfs(1, numbers[0], &numbers, target, expanded)
}

/// Auxiliary function to add || operator
fn merge_numbers_as_strings(num1: i64, num2: i64) -> i64 {
    let merged_string = format!("{}{}", num1, num2);
    merged_string
        .parse::<i64>()
        .expect("Merged number is out of i64 bounds")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration_result() {
        let input_data = vec![
            ("190", vec![10, 19]),
            ("3267", vec![81, 40, 27]),
            ("83", vec![17, 5]),
            ("156", vec![15, 6]),
            ("7290", vec![6, 8, 6, 15]),
            ("161011", vec![16, 10, 13]),
            ("192", vec![17, 8, 14]),
            ("21037", vec![9, 7, 18, 13]),
            ("292", vec![11, 6, 16, 20]),
        ];

        let expected_total = 3749;

        // Compute the actual total
        let mut actual_total = 0;
        for (target, numbers) in input_data {
            let target_value: i64 = target.parse().unwrap();
            if check_if_possible(target_value, numbers, false) {
                actual_total += target_value;
            }
        }

        assert_eq!(
            actual_total, expected_total,
            "Calibration result does not match!"
        );
    }
}
