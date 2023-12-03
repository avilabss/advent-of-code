use std::{env, fs, process};

pub fn read_puzzle(file_name: &str) -> String {
    let mut cwd_path_buff;
    let puzzle_input_path;
    let puzzle_content;

    match env::current_dir() {
        Ok(path) => cwd_path_buff = path,
        Err(err) => {
            println!("Error: {err}");
            process::exit(1);
        }
    }

    cwd_path_buff.push(file_name);
    puzzle_input_path = cwd_path_buff.as_path();

    match fs::read_to_string(puzzle_input_path) {
        Ok(content) => puzzle_content = content,
        Err(err) => {
            println!("Error: {err}");
            process::exit(1);
        }
    }

    puzzle_content
}
