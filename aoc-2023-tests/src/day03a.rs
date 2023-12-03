use std::path::Path;
use rstest::rstest;
use aoc_2023::solution_from;
use aoc_2023::day03a::find_first_digit_around;
use aoc_2023::day03a::find_last_digit_around;
use aoc_2023::day03a::sample_number_at;
use aoc_2023::day03a::sample_numbers_around;
use aoc_2023::day03a::sample_numbers_around_symbols;
use aoc_2023::day03a::solution;
use aoc_2023::day03a::FoundNumber;

#[rstest]
#[case(".", 0, None)]
#[case("2", 0, Some(0))]
#[case(".2", 0, None)]
#[case(".2", 1, Some(1))]
#[case("..12..34..56..", 2, Some(2))]
#[case("..12..34..56..", 3, Some(2))]
#[case("..12..34..56..", 4, None)]
#[case("..12..34..56..", 5, None)]
#[case("..12..34..56..", 6, Some(6))]
fn find_first_digit_around_works(#[case] input: &str, #[case] index: usize, #[case] expected: Option<usize>)
{
	let actual = find_first_digit_around(input, index);

	assert_eq!(actual, expected);
}

#[rstest]
#[case(".", 0, None)]
#[case("2", 0, Some(0))]
#[case(".2", 0, None)]
#[case(".2", 1, Some(1))]
#[case("..12..34..56..", 2, Some(3))]
#[case("..12..34..56..", 3, Some(3))]
#[case("..12..34..56..", 4, None)]
#[case("..12..34..56..", 5, None)]
#[case("..12..34..56..", 6, Some(7))]
fn find_last_digit_around_works(#[case] input: &str, #[case] index: usize, #[case] expected: Option<usize>)
{
	let actual = find_last_digit_around(input, index);

	assert_eq!(actual, expected);
}

#[rstest]
#[case(".", 0, None)]
#[case("2", 0, Some(FoundNumber{ row: 99, column: 0, number: 2 }))]
#[case("9", 0, Some(FoundNumber{ row: 99, column: 0, number: 9 }))]
#[case("123", 2, Some(FoundNumber{ row: 99, column: 0, number: 123 }))]
#[case("..12..34..56..", 6, Some(FoundNumber{ row: 99, column: 6, number: 34 }))]
fn sample_number_at_works(#[case] input: &str, #[case] index: usize, #[case] expected: Option<FoundNumber>)
{
	let actual = sample_number_at(input, 99, index);

	assert_eq!(actual, expected);
}

#[rstest]
#[case(
	vec![
		"..12",
		"345.",
	],
	0,
	0,
	vec![
		FoundNumber{ row: 1, column: 0, number: 345 },
	])]
#[case(
	vec![
		"..12",
		"345.",
	],
	0,
	2,
	vec![
		FoundNumber{ row: 0, column: 2, number: 12 },
		FoundNumber{ row: 1, column: 0, number: 345 },
	])]
#[case(
	vec![
		"123",
		"4.6",
		"789",
	],
	1,
	1,
	vec![
		FoundNumber{ row: 0, column: 0, number: 123 },
		FoundNumber{ row: 1, column: 0, number: 4 },
		FoundNumber{ row: 1, column: 2, number: 6 },
		FoundNumber{ row: 2, column: 0, number: 789 },
	])]
fn sample_numbers_around_works(
	#[case] input: Vec<&str>,
	#[case] row: usize,
	#[case] column: usize,
	#[case] expected: Vec<FoundNumber>)
{
	let mut actual = sample_numbers_around(&input, row, column);
	actual.sort();

	let mut expected = expected;
	expected.sort();
	assert_eq!(actual, expected);
}

#[rstest]
#[case(
	vec![
		".",
	],
	vec![])]
#[case(
	vec![
		"$1",
	],
	vec![
		FoundNumber{ row: 0, column: 1, number: 1 },
	])]
#[case(
	vec![
		".1",
	],
	vec![])]
#[case(
	vec![
		".12",
		"12*",
	],
	vec![
		FoundNumber{ row: 0, column: 1, number: 12 },
		FoundNumber{ row: 1, column: 0, number: 12 },
	])]
fn sample_numbers_around_symbols_works(
	#[case] input: Vec<&str>,
	#[case] expected: Vec<FoundNumber>)
{
	let mut actual = sample_numbers_around_symbols(&input);
	actual.sort();

	let mut expected = expected;
	expected.sort();
	assert_eq!(actual, expected);
}

#[test]
fn example_is_correct()
{
	let input = vec![
		"467..114..",
		"...*......",
		"..35..633.",
		"......#...",
		"617*......",
		".....+.58.",
		"..592.....",
		"......755.",
		"...$.*....",
		".664.598..",
	];

	let actual = solution(&input);
	assert_eq!(actual, 4361);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-03");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 521601);
}
