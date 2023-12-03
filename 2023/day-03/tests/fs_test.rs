use day_03::utils;

#[test]
fn test_read_puzzle() {
    let expect_str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    let read_str = utils::read_puzzle("example_input.txt");
    assert_eq!(expect_str, read_str.as_str())
}
