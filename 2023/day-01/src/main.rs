use std::{env, fs, str::Split};

use regex::Regex;

fn get_digits(puzzle_line: &str) -> Vec<&str> {
    let re = Regex::new(r"\d").unwrap();

    let results = re.find_iter(puzzle_line).map(|m| m.as_str()).collect();
    results
}

fn solve_part_1(puzzle_lines: Split<'_, char>) -> u32 {
    let mut total_sum = 0;

    for line in puzzle_lines {
        let digits = get_digits(line);
        let digits_length = digits.len();

        if digits_length == 0 {
            continue;
        }

        let first_digit = digits[0];
        let last_digit = digits[digits_length - 1];
        let joinned_digit_str = format!("{first_digit}{last_digit}");
        let joinned_digit = joinned_digit_str.parse::<u32>().unwrap();

        total_sum += joinned_digit;
    }

    total_sum
}

fn main() {
    let mut cwd_path_buff = env::current_dir().unwrap();
    cwd_path_buff.push("puzzle_input.txt");

    if let Some(puzzle_input_path) = cwd_path_buff.to_str() {
        let puzzle_input = fs::read_to_string(puzzle_input_path).unwrap();
        let puzzle_lines = puzzle_input.split('\n');

        let total_sum = solve_part_1(puzzle_lines.clone());

        println!("Part-1 Solution: {total_sum}")
    }
}
