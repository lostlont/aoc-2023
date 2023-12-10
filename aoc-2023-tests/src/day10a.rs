use std::path::Path;
use rstest::fixture;
use rstest::rstest;
use aoc_2023::solution_from;
use aoc_2023::day10a::solution;
use aoc_2023::day10a::traverse;
use aoc_2023::day10a::Map;
use aoc_2023::day10a::Pipe;
use aoc_2023::day10a::Position;

const EXAMPLE_1_SIMPLE_INPUT: &str =
".....
.S-7.
.|.|.
.L-J.
.....";

const EXAMPLE_1_COMPLEX_INPUT: &str =
"-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

const EXAMPLE_2_SIMPLE_INPUT: &str =
"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

const EXAMPLE_2_COMPLEX_INPUT: &str =
"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

#[fixture]
fn example_1_simple_map() -> Map
{
	let values = [
		Pipe::Ground, Pipe::Ground, Pipe::Ground, Pipe::Ground, Pipe::Ground,
		Pipe::Ground, Pipe::StartingPosition, Pipe::Horizontal, Pipe::BendSouthWest, Pipe::Ground,
		Pipe::Ground, Pipe::Vertical, Pipe::Ground, Pipe::Vertical, Pipe::Ground,
		Pipe::Ground, Pipe::BendNorthEast, Pipe::Horizontal, Pipe::BendNorthWest, Pipe::Ground,
		Pipe::Ground, Pipe::Ground, Pipe::Ground, Pipe::Ground, Pipe::Ground,
	]
		.iter()
		.cloned();
	Map::new(values, 5, 5, Position{ x: 1, y: 1 })
}

#[fixture]
fn example_1_complex_map() -> Map
{
	let values = [
		Pipe::Horizontal, Pipe::BendNorthEast, Pipe::Vertical, Pipe::BendSouthEast, Pipe::BendSouthWest,
		Pipe::BendSouthWest, Pipe::StartingPosition, Pipe::Horizontal, Pipe::BendSouthWest, Pipe::Vertical,
		Pipe::BendNorthEast, Pipe::Vertical, Pipe::BendSouthWest, Pipe::Vertical, Pipe::Vertical,
		Pipe::Horizontal, Pipe::BendNorthEast, Pipe::Horizontal, Pipe::BendNorthWest, Pipe::Vertical,
		Pipe::BendNorthEast, Pipe::Vertical, Pipe::Horizontal, Pipe::BendNorthWest, Pipe::BendSouthEast,
	]
		.iter()
		.cloned();
	Map::new(values, 5, 5, Position{ x: 1, y: 1 })
}

#[rstest]
#[case('|', Pipe::Vertical)]
#[case('-', Pipe::Horizontal)]
#[case('L', Pipe::BendNorthEast)]
#[case('J', Pipe::BendNorthWest)]
#[case('7', Pipe::BendSouthWest)]
#[case('F', Pipe::BendSouthEast)]
#[case('.', Pipe::Ground)]
#[case('S', Pipe::StartingPosition)]
fn pipe_from_char_works(#[case] value: char, #[case] expected: Pipe)
{
	let actual = Pipe::from(value);

	assert_eq!(actual, expected);
}

#[test]
fn map_from_vec_str_works()
{
	let input = EXAMPLE_1_SIMPLE_INPUT
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = Map::from(&input);

	let expected = example_1_simple_map();
	assert_eq!(actual, expected);
}

#[rstest]
#[case(
	example_1_simple_map(),
	Position{ x: 1, y: 1 },
	Position{ x: -1, y: 0 },
	None)]
#[case(
	example_1_simple_map(),
	Position{ x: 1, y: 1 },
	Position{ x: 0, y: 1 },
	Some(8))]
#[case(
	example_1_simple_map(),
	Position{ x: 1, y: 1 },
	Position{ x: 1, y: 0 },
	Some(8))]
#[case(
	example_1_complex_map(),
	Position{ x: 1, y: 1 },
	Position{ x: -1, y: 0 },
	None)]
#[case(
	example_1_complex_map(),
	Position{ x: 1, y: 1 },
	Position{ x: 0, y: -1 },
	None)]
#[case(
	example_1_complex_map(),
	Position{ x: 1, y: 1 },
	Position{ x: 1, y: 0 },
	Some(8))]
#[case(
	example_1_complex_map(),
	Position{ x: 1, y: 1 },
	Position{ x: 0, y: 1 },
	Some(8))]
fn traverse_works(#[case] map: Map, #[case] from: Position, #[case] direction: Position, #[case] expected: Option<u32>)
{
	let actual = traverse(&map, from, direction);

	assert_eq!(actual, expected);
}

#[rstest]
#[case(EXAMPLE_1_SIMPLE_INPUT, 4)]
#[case(EXAMPLE_1_COMPLEX_INPUT, 4)]
#[case(EXAMPLE_2_SIMPLE_INPUT, 8)]
#[case(EXAMPLE_2_COMPLEX_INPUT, 8)]
fn example_1_simple_is_correct(#[case] input: &str, #[case] expected: u32)
{
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = solution(&input);

	assert_eq!(actual, expected);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-10");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 6927);
}
