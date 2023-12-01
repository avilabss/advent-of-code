use std::{env, fs, str::Split};

use regex::Regex;

fn get_digits(puzzle_line: &str) -> Vec<&str> {
    let re = Regex::new(r"\d").unwrap();

    let results = re.find_iter(puzzle_line).map(|m| m.as_str()).collect();
    results
}

fn convert_to_digits(puzzle_line: &str) -> String {
    let parsed_string = puzzle_line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    parsed_string
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

fn solve_part_2(puzzle_lines: Split<'_, char>) -> u32 {
    let mut total_sum = 0;

    for line in puzzle_lines {
        let patched_line = convert_to_digits(line);
        let digits = get_digits(patched_line.as_str());
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

        let solution_part_1 = solve_part_1(puzzle_lines.clone());
        let solution_part_2 = solve_part_2(puzzle_lines);

        println!("Part-1 Solution: {solution_part_1}");
        println!("Part-2 Solution: {solution_part_2}");
    }
}
