use std::io::Cursor;
use std::path::Path;
use rstest::*;
use aoc_2023::solution_from_reader;
use aoc_2023::common::map::Map;
use aoc_2023::day16a::{Cell, Field, Flow, Mirror, Splitter, solution};

#[test]
fn parse_field_works()
{
	let input =
	[
		r".|.\",
		r"..-.",
		r"/...",
	];

	let actual = Field::parse(&input);

	let cells = Map::new(
		[
			Cell::Empty, Cell::Splitter(Splitter::Vertical), Cell::Empty, Cell::Mirror(Mirror::LeftToDown),
			Cell::Empty, Cell::Empty, Cell::Splitter(Splitter::Horizontal), Cell::Empty,
			Cell::Mirror(Mirror::LeftToUp), Cell::Empty, Cell::Empty, Cell::Empty,
		],
		4,
		3);
	let flow_map = Field::empty_flow_of(&cells);
	let expected = Field::new(cells, flow_map);
	assert_eq!(actual, expected);
}

#[rstest]
#[case(
	create_cells(&[
		"..",
		"..",
	]),
	create_flow_map(&[
		">.",
		"..",
	]),
	&[(0, 0)],
	create_flow_map(&[
		">>",
		"..",
	]),
	vec![(1, 0)],
)]
#[case(
	create_cells(&[
		"..",
		"..",
	]),
	create_flow_map(&[
		">>",
		"..",
	]),
	&[(1, 0)],
	create_flow_map(&[
		">>",
		"..",
	]),
	vec![],
)]
#[case(
	create_cells(&[
		"..",
		"..",
	]),
	create_flow_map(&[
		">v",
		"..",
	]),
	&[(0, 0)],
	Map::new(
		[
			Flow::FROM_LEFT, (Flow::FROM_LEFT | Flow::FROM_TOP),
			Flow::EMPTY, Flow::EMPTY,
		],
		2,
		2),
	vec![(1, 0)],
)]
#[case(
	create_cells(&[
		"....",
		"....",
		"....",
		"....",
	]),
	create_flow_map(&[
		"..v.",
		">...",
		"...<",
		".^..",
	]),
	&[(0, 1), (2, 0), (3, 2), (1, 3)],
	create_flow_map(&[
		"..v.",
		">>v.",
		".^<<",
		".^..",
	]),
	vec![(1, 1), (2, 1), (2, 2), (1, 2)],
)]
#[case(
	create_cells(&[
		"../.",
		"/...",
		".../",
		"./..",
	]),
	create_flow_map(&[
		"..v.",
		">...",
		"...<",
		".^..",
	]),
	&[(0, 1), (2, 0), (3, 2), (1, 3)],
	create_flow_map(&[
		"^<v.",
		">...",
		"...<",
		".^>v",
	]),
	vec![(0, 0), (1, 0), (3, 3), (2, 3)],
)]
#[case(
	create_cells(&[
		r"..\.",
		r"\...",
		r"...\",
		r".\..",
	]),
	create_flow_map(&[
		"..v.",
		">...",
		"...<",
		".^..",
	]),
	&[(0, 1), (2, 0), (3, 2), (1, 3)],
	create_flow_map(&[
		"..v>",
		">..^",
		"v..<",
		"<^..",
	]),
	vec![(0, 2), (3, 0), (3, 1), (0, 3)],
)]
#[case(
	create_cells(&[
		"..-.",
		"-...",
		"...-",
		".-..",
	]),
	create_flow_map(&[
		"..v.",
		">...",
		"...<",
		".^..",
	]),
	&[(0, 1), (2, 0), (3, 2), (1, 3)],
	create_flow_map(&[
		".<v>",
		">>..",
		"..<<",
		"<^>.",
	]),
	vec![(1, 1), (1, 0), (3, 0), (2, 2), (0, 3), (2, 3)],
)]
#[case(
	create_cells(&[
		"..|.",
		"|...",
		"...|",
		".|..",
	]),
	create_flow_map(&[
		"..v.",
		">...",
		"...<",
		".^..",
	]),
	&[(0, 1), (2, 0), (3, 2), (1, 3)],
	create_flow_map(&[
		"^.v.",
		">.v^",
		"v^.<",
		".^.v",
	]),
	vec![(0, 0), (0, 2), (2, 1), (3, 1), (3, 3), (1, 2)],
)]
#[case(
	create_cells(&[
		"...",
		"...",
		"...",
	]),
	create_flow_map(&[
		".v.",
		">.<",
		".^.",
	]),
	&[(0, 1), (1, 0), (2, 1), (1, 2)],
	Map::new(
		[
			Flow::EMPTY, Flow::FROM_TOP, Flow::EMPTY,
			Flow::FROM_LEFT, Flow::new(true, true, true, true), Flow::FROM_RIGHT,
			Flow::EMPTY, Flow::FROM_BOTTOM, Flow::EMPTY,
		],
		3,
		3),
	vec![(1, 1)],
)]
#[case(
	create_cells(&[
		"..",
		"..",
	]),
	create_flow_map(&[
		">>",
		"..",
	]),
	&[(0, 0)],
	create_flow_map(&[
		">>",
		"..",
	]),
	vec![],
)]
fn field_flow_works(
	#[case] cells: Map<Cell>,
	#[case] flow_map: Map<Flow>,
	#[case] changed_cells: &[(usize, usize)],
	#[case] expected_flow_map: Map<Flow>,
	#[case] mut expected_result: Vec<(usize, usize)>)
{
	let mut subject = Field::new(cells, flow_map);

	let mut actual_result = subject.flow(changed_cells);
	actual_result.sort();
	let actual_flow_map = subject.flow_map();

	assert_eq!(actual_flow_map, &expected_flow_map);

	expected_result.sort();
	assert_eq!(actual_result, expected_result);
}

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

	assert_eq!(actual, 46);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-16");
	let actual = solution_from_reader(&path, solution);

	assert_eq!(actual, 7242);
}

fn create_cells(values: &[&str]) -> Map::<Cell>
{
	let parser = |c| match c
	{
		'.' => Cell::Empty,
		'/' => Cell::Mirror(Mirror::LeftToUp),
		'\\' => Cell::Mirror(Mirror::LeftToDown),
		'-' => Cell::Splitter(Splitter::Horizontal),
		'|' => Cell::Splitter(Splitter::Vertical),
		_ => panic!("Character '{c}' can not be parsed as a cell!"),
	};

	create_map(values, parser)
}

fn create_flow_map(values: &[&str]) -> Map::<Flow>
{
	let parser = |c| match c
	{
		'.' => Flow::EMPTY,
		'>' => Flow::FROM_LEFT,
		'v' => Flow::FROM_TOP,
		'<' => Flow::FROM_RIGHT,
		'^' => Flow::FROM_BOTTOM,
		_ => panic!("Character '{c}' can not be parsed as flow!"),
	};

	create_map(values, parser)
}

fn create_map<T>(values: &[&str], parser: impl Fn(char) -> T) -> Map::<T>
{
	let width = values[0].len();
	let height = values.len();

	let values = values
		.iter()
		.flat_map(|line| line.chars())
		.map(parser);

	Map::new(values, width, height)
}
