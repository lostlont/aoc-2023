use std::path::Path;
use rstest::rstest;
use aoc_2023::solution_from;
use aoc_2023::day02a::parse_record;
use aoc_2023::day02a::parse_set;
use aoc_2023::day02a::solution;
use aoc_2023::day02a::Record;
use aoc_2023::day02a::Set;

#[rstest]
#[case("3 blue, 1 red, 2 green", Set{ r: 1, g: 2, b: 3 })]
#[case("5 green", Set{ r: 0, g: 5, b: 0 })]
fn parse_set_works(#[case] input: &str, #[case] expected: Set)
{
	let actual = parse_set(input);

	assert_eq!(actual, expected);
}

#[rstest]
#[case(
	"Game 5: 3 blue, 1 red, 2 green",
	Record
	{
		id: 5,
		sets: vec![
			Set{ r: 1, g: 2, b: 3 },
		],
	})]
#[case(
	"Game 11: 12 green, 13 red, 14 blue; 15 blue, 16 green, 17 red",
	Record
	{
		id: 11,
		sets: vec![
			Set{ r: 13, g: 12, b: 14 },
			Set{ r: 17, g: 16, b: 15 },
		],
	})]
fn parse_record_works(#[case] input: &str, #[case] expected: Record)
{
	let actual = parse_record(input);

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
	assert_eq!(actual, 8);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-02");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 2285);
}
