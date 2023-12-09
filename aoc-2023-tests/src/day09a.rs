use std::path::Path;
use rstest::rstest;
use aoc_2023::solution_from;
use aoc_2023::day09a::derive;
use aoc_2023::day09a::derive_once;
use aoc_2023::day09a::extrapolate;
use aoc_2023::day09a::solution;
use aoc_2023::day09a::Num;

#[rstest]
#[case(12, vec![15], vec![12, 3])]
#[case(9, vec![12, 3], vec![9, 3, 0])]
#[case(21, vec![28], vec![21, 7])]
#[case(15, vec![21, 7], vec![15, 6, 1])]
#[case(10, vec![15, 6, 1], vec![10, 5, 1, 0])]
fn derive_once_works(#[case] last_value: Num, #[case] next_values: Vec<Num>, #[case] expected: Vec<Num>)
{
	let actual = derive_once(last_value, &next_values);

	assert_eq!(actual, expected);
}

#[rstest]
#[case(vec![0, 3, 6, 9, 12, 15], vec![15, 3, 0])]
#[case(vec![1, 3, 6, 10, 15, 21], vec![21, 6, 1, 0])]
#[case(vec![10, 13, 16, 21, 30, 45], vec![45, 15, 6, 2, 0])]
fn derive_works(#[case] values: Vec<Num>, #[case] expected: Vec<Num>)
{
	let actual = derive(&values);

	assert_eq!(actual, expected);
}

#[rstest]
#[case(vec![0, 3, 6, 9, 12, 15], 18)]
#[case(vec![1, 3, 6, 10, 15, 21], 28)]
#[case(vec![10, 13, 16, 21, 30, 45], 68)]
fn extrapolate_works(#[case] input: Vec<Num>, #[case] expected: Num)
{
	let actual = extrapolate(&input);

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

	assert_eq!(actual, 114);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-09");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 1);
}
