use std::path::Path;
use rstest::rstest;
use aoc_2023::solution_from;
use aoc_2023::day09a::Num;
use aoc_2023::day09b::derive_back;
use aoc_2023::day09b::extrapolate_back;
use aoc_2023::day09b::solution;

#[rstest]
#[case(vec![10, 13, 16, 21, 30, 45], vec![10, 3, 0, 2, 0])]
fn derive_back_works(#[case] values: Vec<Num>, #[case] expected: Vec<Num>)
{
	let actual = derive_back(&values);

	assert_eq!(actual, expected);
}

#[rstest]
#[case(vec![0, 3, 6, 9, 12, 15], -3)]
#[case(vec![1, 3, 6, 10, 15, 21], 0)]
#[case(vec![10, 13, 16, 21, 30, 45], 5)]
fn extrapolate_back_works(#[case] values: Vec<Num>, #[case] expected: Num)
{
	let actual = extrapolate_back(&values);

	assert_eq!(actual, expected);
}

#[test]
fn example_is_correct()
{
	let input =
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = solution(&input);

	assert_eq!(actual, 2);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-09");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 942);
}
