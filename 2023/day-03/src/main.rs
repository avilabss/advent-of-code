mod utils;

fn main() {
    let puzzle = utils::read_puzzle("puzzle_input.txt");
    let mut puzzle_line_iter = puzzle.split("\n").into_iter();

    loop {
        let line_1 = puzzle_line_iter.next();
        let line_2 = puzzle_line_iter.next();

        if line_1.is_none() || line_2.is_none() {
            break;
        }
    }
}
