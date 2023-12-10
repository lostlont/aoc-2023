use rstest::rstest;
use crate::day10a::EXAMPLE_1_SIMPLE_INPUT;
use crate::day10a::EXAMPLE_1_COMPLEX_INPUT;
use crate::day10a::EXAMPLE_2_SIMPLE_INPUT;
use crate::day10a::EXAMPLE_2_COMPLEX_INPUT;
use aoc_2023::day10b::solution;

const EXAMPLE_3_INPUT: &str =
"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

#[rstest]
#[case(EXAMPLE_1_SIMPLE_INPUT, 1)]
#[case(EXAMPLE_1_COMPLEX_INPUT, 0)]
#[case(EXAMPLE_2_SIMPLE_INPUT, 1)]
#[case(EXAMPLE_2_COMPLEX_INPUT, 0)]
#[case(EXAMPLE_3_INPUT, 4)]
fn example_is_correct(#[case] input: &str, #[case] expected: u32)
{
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = solution(&input);

	assert_eq!(actual, expected);
}
