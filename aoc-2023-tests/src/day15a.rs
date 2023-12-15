use std::{io::Cursor, path::Path};
use rstest::*;
use aoc_2023::solution_from_reader;
use aoc_2023::day15a::solution;


#[rstest]
#[case("rn=1", 30)]
#[case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", 1320)]
fn example_is_correct(#[case] input: &str, #[case] expected: u32)
{
	let input = Cursor::new(input);

	let actual = solution(input);

	assert_eq!(actual, expected);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-15");
	let actual = solution_from_reader(&path, solution);

	assert_eq!(actual, 515495);
}

