use std::path::Path;
use rstest::*;
use aoc_2023::day01a::filter_numbers;
use aoc_2023::day01a::get_calibration_value;
use aoc_2023::day01a::solution;
use aoc_2023::day01a::solution_from;

#[rstest]
#[case("seven7seven", &vec![7])]
#[case("a1b2c", &vec![1, 2])]
#[case("lot9of8numbers7here", &vec![9, 8, 7])]
fn filter_numbers_works(#[case] input: &str, #[case] expected: &Vec<u32>)
{
	let actual = filter_numbers(input);

	assert_eq!(actual, *expected);
}

#[rstest]
#[case(&vec![1, 2], 12)]
#[case(&vec![7], 77)]
#[case(&vec![3, 5, 7, 6], 36)]
fn get_calibration_value_works(#[case] input: &Vec<u32>, #[case] expected: u32)
{
	let actual = get_calibration_value(input);

	assert_eq!(actual, expected);
}

#[test]
fn example_is_correct()
{
	let input = vec![
		"1abc2",
		"pqr3stu8vwx",
		"a1b2c3d4e5f",
		"treb7uchet",
	];

	let actual = solution(&input);
	assert_eq!(actual, 142);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-01");
	let actual = solution_from(&path);

	assert_eq!(actual, 55621);
}
