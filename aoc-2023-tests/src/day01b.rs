use std::path::Path;
use rstest::*;
use aoc_2023::day01b::filter_number;
use aoc_2023::day01b::filter_numbers;
use aoc_2023::day01b::solution;
use aoc_2023::day01b::solution_from;

#[rstest]
#[case("a", None)]
#[case("1", Some(1))]
#[case("9", Some(9))]
#[case("one", Some(1))]
#[case("two", Some(2))]
#[case("three", Some(3))]
#[case("four", Some(4))]
#[case("five", Some(5))]
#[case("six", Some(6))]
#[case("seven", Some(7))]
#[case("eight", Some(8))]
#[case("nine", Some(9))]
#[case("ninex", Some(9))]
fn filter_number_works(#[case] input: &str, #[case] expected: Option<u32>)
{
	let actual = filter_number(input);

	assert_eq!(actual, expected);
}

#[rstest]
#[case("seven7seven", &vec![7, 7, 7])]
#[case("a1b2c", &vec![1, 2])]
#[case("lot9of8numbers7here", &vec![9, 8, 7])]
#[case("oononettwtwo", &vec![1, 2])]
fn filter_numbers_works(#[case] input: &str, #[case] expected: &Vec<u32>)
{
	let actual = filter_numbers(input);

	assert_eq!(actual, *expected);
}

#[test]
fn example_is_correct()
{
	let input = vec![
		"two1nine",
		"eightwothree",
		"abcone2threexyz",
		"xtwone3four",
		"4nineeightseven2",
		"zoneight234",
		"7pqrstsixteen",
	];

	let actual = solution(&input);
	assert_eq!(actual, 281);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-01");
	let actual = solution_from(&path);

	assert_eq!(actual, 53592);
}
