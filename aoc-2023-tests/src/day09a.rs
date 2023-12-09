use std::path::Path;
use rstest::rstest;
use aoc_2023::solution_from;
use aoc_2023::day09a::derive;
use aoc_2023::day09a::derive_once;
use aoc_2023::day09a::extrapolate;
use aoc_2023::day09a::solution;
use aoc_2023::day09a::Num;

#[rstest]
#[case(vec![0, 3, 6, 9, 12, 15], vec![3, 3, 3, 3, 3])]
#[case(vec![3, 3, 3, 3, 3], vec![0, 0, 0, 0])]
#[case(vec![1, 3, 6, 10, 15, 21], vec![2, 3, 4, 5, 6])]
#[case(vec![2, 3, 4, 5, 6], vec![1, 1, 1, 1])]
#[case(vec![1, 1, 1, 1], vec![0, 0, 0])]
fn derive_once_works(#[case] values: Vec<Num>, #[case] expected: Vec<Num>)
{
	let actual = derive_once(&values);

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
fn extrapolate_works(#[case] values: Vec<Num>, #[case] expected: Num)
{
	let actual = extrapolate(&values);

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

	assert_eq!(actual, 2175229206);
}
