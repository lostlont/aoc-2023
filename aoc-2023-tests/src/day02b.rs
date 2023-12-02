use std::path::Path;
use rstest::rstest;
use aoc_2023::solution_from;
use aoc_2023::day02a::Set;
use aoc_2023::day02b::minimum_set;
use aoc_2023::day02b::solution;

#[rstest]
#[case(
	vec![
		Set{ r: 1, g: 2, b: 3 },
	],
	Set{ r: 1, g: 2, b: 3 })]
#[case(
	vec![
		Set{ r: 0, g: 5, b: 7 },
		Set{ r: 0, g: 8, b: 3 },
	],
	Set{ r: 0, g: 8, b: 7 })]
fn minimum_set_works(#[case] input: Vec<Set>, #[case] expected: Set)
{
	let actual = minimum_set(&input);

	assert_eq!(actual, expected);
}

#[test]
fn example_is_correct()
{
	let input = vec![
		"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
		"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
		"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
		"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
		"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
	];

	let actual = solution(&input);

	assert_eq!(actual, 2286);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-02");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 77021);
}
