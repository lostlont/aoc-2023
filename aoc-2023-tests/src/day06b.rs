use std::path::Path;
use aoc_2023::solution_from;
use aoc_2023::day06b::solution;

#[test]
fn example_is_correct()
{
	let input =
"Time:      7  15   30
Distance:  9  40  200
";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = solution(&input);

	assert_eq!(actual, 71503);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-06");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 35150181);
}
