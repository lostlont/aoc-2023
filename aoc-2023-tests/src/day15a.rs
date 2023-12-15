use std::io::Cursor;
use std::path::Path;
use rstest::*;
use aoc_2023::solution_from_reader;
use aoc_2023::day15a::hash;
use aoc_2023::day15a::solution;


#[rstest]
#[case("rn", 0)]
#[case("cm", 0)]
#[case("qp", 1)]
#[case("pc", 3)]
#[case("ot", 3)]
#[case("ab", 3)]
#[case("rn=1", 30)]
fn hash_works(#[case] input: &str, #[case] expected: u8)
{
	let actual = hash(input);

	assert_eq!(actual, expected);
}

#[test]
fn example_is_correct()
{
	let input = Cursor::new("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");

	let actual = solution(input);

	assert_eq!(actual, 1320);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-15");
	let actual = solution_from_reader(&path, solution);

	assert_eq!(actual, 515495);
}

