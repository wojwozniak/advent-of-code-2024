use std::fs::File;
use std::io::{self, BufRead};

/*
   Problem 07 of 2024 Advent of Code
   https://adventofcode.com/2024/day/7
*/
pub fn problem07() -> Result<i64, ()> {
    let path = "../inputs/input07.txt";

    let file: File = File::open(&path).unwrap();
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut sum: i64 = 0;

    for line in reader.lines() {
        let line: String = line.unwrap();
        if let Some((left, right)) = line.split_once(":") {
            let target: i64 = left.trim().parse().unwrap();
            let numbers: Vec<i64> = right
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();

            if check_if_possible(target, numbers) {
                sum += target;
            }
        }
    }

    println!("{}", sum);
    Ok(sum)
}

/// Check if it is possible to create target with numbers using + and *
/// using depth first search
fn check_if_possible(target: i64, numbers: Vec<i64>) -> bool {
    fn dfs(index: usize, current_value: i64, numbers: &[i64], target: i64) -> bool {
        if index == numbers.len() {
            return current_value == target;
        }

        let next_number = numbers[index];

        if dfs(index + 1, current_value + next_number, numbers, target) {
            return true;
        }

        if dfs(index + 1, current_value * next_number, numbers, target) {
            return true;
        }

        false
    }

    if numbers.is_empty() {
        return false;
    }

    dfs(1, numbers[0], &numbers, target)
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
            if check_if_possible(target_value, numbers) {
                actual_total += target_value;
            }
        }

        assert_eq!(
            actual_total, expected_total,
            "Calibration result does not match!"
        );
    }
}
