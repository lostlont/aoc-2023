use std::path::Path;
use rstest::rstest;
use aoc_2023::solution_from;
use aoc_2023::day03b::get_gear_ratios_around_gears;
use aoc_2023::day03b::solution;

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
	vec![])]
#[case(
	vec![
		"*1",
	],
	vec![])]
#[case(
	vec![
		".12",
		"12$",
		"...",
	],
	vec![])]
#[case(
	vec![
		".12",
		"12*",
		"...",
	],
	vec![12 * 12])]
#[case(
	vec![
		".12",
		"12*",
		".34",
	],
	vec![])]
#[case(
	vec![
		"12..",
		"*12*",
		"..12",
	],
	vec![12 * 12, 12 * 12])]
fn get_gear_ratios_around_gears_works(
	#[case] input: Vec<&str>,
	#[case] expected: Vec<u32>)
{
	let mut actual = get_gear_ratios_around_gears(&input);
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
	assert_eq!(actual, 467835);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-03");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 80694070);
}
