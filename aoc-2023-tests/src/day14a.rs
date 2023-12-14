use std::path::Path;
use rstest::*;
use aoc_2023::solution_from;
use aoc_2023::day14a::solution;
use aoc_2023::day14a::Direction;
use aoc_2023::day14a::Map;

#[test]
fn map_from_vec_str_works()
{
	let input = vec!
	[
		".#.",
		"O.#",
	];

	let actual = Map::from(input.as_slice());

	let expected = Map::new(
		".#.O.#",
		3,
		2,
		Direction::North);
	assert_eq!(actual, expected);
}

#[rstest]
#[case(0, 0, Some('.'))]
#[case(1, 0, Some('#'))]
#[case(3, 0, None)]
#[case(0, 1, Some('O'))]
#[case(1, 1, Some('.'))]
#[case(2, 1, Some('#'))]
#[case(0, 2, None)]
fn map_at_works(#[case] x: usize, #[case] y: usize, #[case] expected: Option<char>)
{
	let subject = create_map(&[
		".#.",
		"O.#",
	]);

	let actual = subject.at(x, y);

	assert_eq!(actual, expected);
}

#[rstest]
#[case(
	&[
		".",
	],
	Direction::North,
	&[
		".",
	],
)]
#[case(
	&[
		"O",
	],
	Direction::North,
	&[
		"O",
	],
)]
#[case(
	&[
		"....",
		"...#",
		".O..",
		"...O",
	],
	Direction::North,
	&[
		".O..",
		"...#",
		"...O",
		"....",
	],
)]
#[case(
	&[
		"....",
		"....",
		".O..",
		".#.O",
	],
	Direction::West,
	&[
		"....",
		"....",
		"O...",
		".#O.",
	],
)]
#[case(
	&[
		".O..",
		"....",
		".#.O",
		"....",
	],
	Direction::South,
	&[
		"....",
		".O..",
		".#..",
		"...O",
	],
)]
#[case(
	&[
		"....",
		"O.#.",
		"....",
		"..O.",
	],
	Direction::East,
	&[
		"....",
		".O#.",
		"....",
		"...O",
	],
)]
fn map_tick_moves_objects(#[case] input: &[&str], #[case] direction: Direction, #[case] expected: &[&str])
{
	let mut subject = create_map(input);
	subject.set_direction(direction);

	subject.tick();

	let mut expected = create_map(expected);
	expected.set_direction(direction);
	assert_eq!(subject, expected);
}

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

	assert_eq!(actual, 136);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-14");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 109385);
}

fn create_map(values: &[&str]) -> Map
{
	Map::new(
		&values.join(""),
		values[0].len(),
		values.len(),
		Direction::North)
}
