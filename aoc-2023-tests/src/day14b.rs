use std::path::Path;
use aoc_2023::solution_from;
use aoc_2023::day14b::solution;

#[test]
fn example_is_correct()
{
	let input =
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = solution(&input);

	assert_eq!(actual, 64);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-14");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 93102);
}
