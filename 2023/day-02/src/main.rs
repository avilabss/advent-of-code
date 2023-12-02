use std::{env, fs, str::Split};

fn parse_cube_count(color_part: &str, replace: &str) -> u32 {
    let cube_count = color_part.replace(replace, "");
    let cube_count = cube_count.trim().parse::<u32>().unwrap();
    cube_count
}

fn solve(puzzle_lines: Split<'_, &str>) -> u32 {
    let mut game_ids: Vec<u32> = vec![];
    let max_red_cubes = 12;
    let max_green_cubes = 13;
    let max_blue_cubes = 14;

    for line in puzzle_lines {
        let mut line_parts = line.split(":");
        let game_id = line_parts.next().unwrap().replace("Game", "");
        let game_sets = line_parts.next().unwrap().split(";");
        let mut is_valid_game = true;

        for game_set in game_sets {
            let mut total_red_cubes_this_game = 0;
            let mut total_green_cubes_this_game = 0;
            let mut total_blue_cubes_this_game = 0;
            let color_parts = game_set.split(",");

            for color_part in color_parts {
                if color_part.contains("red") {
                    let cube_count = parse_cube_count(color_part, "red");
                    total_red_cubes_this_game += cube_count;
                } else if color_part.contains("green") {
                    let cube_count = parse_cube_count(color_part, "green");
                    total_green_cubes_this_game += cube_count;
                } else if color_part.contains("blue") {
                    let cube_count = parse_cube_count(color_part, "blue");
                    total_blue_cubes_this_game += cube_count;
                }
            }

            if total_red_cubes_this_game > max_red_cubes
                || total_green_cubes_this_game > max_green_cubes
                || total_blue_cubes_this_game > max_blue_cubes
            {
                is_valid_game = false;
                break;
            }
        }

        if is_valid_game {
            game_ids.push(game_id.trim().parse::<u32>().unwrap());
        }
    }

    game_ids.iter().sum()
}

fn main() {
    let mut cwd_path_buff = env::current_dir().unwrap();
    cwd_path_buff.push("puzzle_input.txt");

    if let Some(puzzle_input_path) = cwd_path_buff.to_str() {
        let puzzle_input = fs::read_to_string(puzzle_input_path).unwrap();
        let puzzle_lines = puzzle_input.split("\n");

        let solution = solve(puzzle_lines);

        println!("Solution: {}", solution);
    }
}
