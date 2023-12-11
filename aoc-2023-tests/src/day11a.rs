use std::path::Path;
use rstest::rstest;
use aoc_2023::solution_from;
use aoc_2023::day11a::distances;
use aoc_2023::day11a::expand_map;
use aoc_2023::day11a::parse_map;
use aoc_2023::day11a::solution;
use aoc_2023::day11a::Position;

#[test]
fn parse_map_works()
{
	let input =
".....
#....
.....
...#.";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let mut actual = parse_map(input)
		.collect::<Vec<_>>();
	actual.sort();

	let mut expected = new_map(&[(0, 1), (3, 3)]);
	expected.sort();
	assert_eq!(actual, expected);
}

#[rstest]
#[case(
	&[(0, 1), (3, 3)],
	&[(0, 2), (5, 5)])]
#[case(
	&[(3, 0), (7, 1), (0, 2), (6, 4), (1, 5), (9, 6), (7, 8), (0, 9), (4, 9)],
	&[(4, 0), (9, 1), (0, 2), (8, 5), (1, 6), (12, 7), (9, 10), (0, 11), (5, 11)])]
fn expand_map_works(#[case] in_positions: &[(i32, i32)], #[case] expected_positions: &[(i32, i32)])
{
	let map = new_map(in_positions);

	let actual = expand_map(map.iter().cloned())
		.collect::<Vec<_>>();

	let expected = new_map(expected_positions);
	assert_eq!(actual, expected);
}

#[test]
fn distances_works()
{
	let map = new_map(&[(0, 0), (1, 0), (0, 1), (1, 1)]);

	let mut actual = distances(map.iter().cloned())
		.collect::<Vec<_>>();
	actual.sort();

	let mut expected = vec![1, 1, 2, 2, 1, 1];
	expected.sort();
	assert_eq!(actual, expected);
}

#[test]
fn example_is_correct()
{
	let input =
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = solution(&input);

	assert_eq!(actual, 374);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-11");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 9329143);
}

fn new_map(positions: &[(i32, i32)]) -> Vec<Position>
{
	positions
		.iter()
		.cloned()
		.map(Position::from)
		.collect::<Vec<_>>()
}
