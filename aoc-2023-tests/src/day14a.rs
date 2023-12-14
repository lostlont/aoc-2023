use std::path::Path;
use itertools::Itertools;
use rstest::*;
use aoc_2023::solution_from;
use aoc_2023::day14a::solution;
use aoc_2023::day14a::Map;
use aoc_2023::day14a::Object;

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
		".#...#",
		3,
		2,
		vec![Object::new(0, 1)]);
	assert_eq!(actual, expected);
}

#[rstest]
#[case(0, 0, Some('.'))]
#[case(1, 0, Some('#'))]
#[case(3, 0, None)]
#[case(0, 1, Some('.'))]
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

#[test]
fn map_orders_objects_by_x_then_by_y()
{
	let subject = create_map(&[
		".#O",
		"O.#",
		"#.O",
	]);

	let actual = subject
		.get_objects()
		.cloned()
		.collect_vec();

	let expected = vec!
	[
		Object::new(0, 1),
		Object::new(2, 0),
		Object::new(2, 2),
	];
	assert_eq!(actual, expected);
}

#[rstest]
#[case(
	&[
		".",
	],
	(vec![], false),
)]
#[case(
	&[
		"O",
	],
	(vec![
		Object::new(0, 0),
	], false),
)]
#[case(
	&[
		".O#O",
		"O.OO"
	],
	(vec![
		Object::new(0, 0),
		Object::new(1, 0),
		Object::new(2, 1),
		Object::new(3, 0),
		Object::new(3, 1),
	], true),
)]
#[case(
	&[
		"OO#O",
		"..OO"
	],
	(vec![
		Object::new(0, 0),
		Object::new(1, 0),
		Object::new(2, 1),
		Object::new(3, 0),
		Object::new(3, 1),
	], false),
)]
fn map_tick_moves_objects(#[case] input: &[&str], #[case] expected: (Vec<Object>, bool))
{
	let mut subject = create_map(input);

	let is_moved = subject.tick();
	let objects = subject
		.get_objects()
		.cloned()
		.collect_vec();
	let actual = (objects, is_moved);

	assert_eq!(actual, expected);
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
	let data = values
		.iter()
		.flat_map(|line| line.chars())
		.map(|c| match c
		{
			'O' => '.',
			c => c,
		})
		.join("");

	let objects = values
		.iter()
		.enumerate()
		.flat_map(|(y, line)| line
			.char_indices()
			.filter(|(_, c)| *c == 'O')
			.map(move |(x, _)| (x as i32, y as i32)))
		.map(|(x, y)| Object::new(x, y))
		.collect_vec();

	Map::new(
		&data,
		values[0].len(),
		values.len(),
		objects)
}
