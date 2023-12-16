use std::io::Cursor;
use std::path::Path;
use aoc_2023::solution_from_reader;
use aoc_2023::day16b::solution;

#[test]
fn example_is_correct()
{
	let input = Cursor::new(
r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....");

	let actual = solution(input);

	assert_eq!(actual, 51);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-16");
	let actual = solution_from_reader(&path, solution);

	assert_eq!(actual, 7572);
}
