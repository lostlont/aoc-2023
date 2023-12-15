use std::collections::VecDeque;
use std::path::Path;
use std::io::Cursor;
use maplit::hashmap;
use rstest::*;
use aoc_2023::solution_from_reader;
use aoc_2023::day15b::{Boxes, Lens, Operation, solution};

#[rstest]
#[case("rn=1", Operation::Add{ label: "rn".into(), focal_length: 1 })]
#[case("qp=3", Operation::Add{ label: "qp".into(), focal_length: 3 })]
#[case("cm-", Operation::Remove{ label: "cm".into() })]
fn operation_from_str_works(#[case] value: &str, #[case] expected: Operation)
{
	let actual = Operation::from(value);

	assert_eq!(actual, expected);
}

#[rstest]
#[case(
	Boxes::new(hashmap!
		{
			0 => VecDeque::from(vec![
				Lens::new("cm", 1),
			]),
		}),
	Operation::Add{ label: "rn".to_owned(), focal_length: 2 },
	Boxes::new(hashmap!
		{
			0 => VecDeque::from(vec![
				Lens::new("cm", 1),
				Lens::new("rn", 2),
			]),
		}),
)]
#[case(
	Boxes::new(hashmap!
		{
			0 => VecDeque::from(vec![
				Lens::new("cm", 1),
				Lens::new("rn", 2),
			]),
		}),
	Operation::Add{ label: "cm".to_owned(), focal_length: 3 },
	Boxes::new(hashmap!
		{
			0 => VecDeque::from(vec![
				Lens::new("cm", 3),
				Lens::new("rn", 2),
			]),
		}),
)]
#[case(
	Boxes::new(hashmap!
		{
			0 => VecDeque::from(vec![
				Lens::new("rn", 2),
			]),
		}),
	Operation::Remove{ label: "cm".to_owned() },
	Boxes::new(hashmap!
		{
			0 => VecDeque::from(vec![
				Lens::new("rn", 2),
			]),
		}),
)]
#[case(
	Boxes::new(hashmap!
		{
			0 => VecDeque::from(vec![
				Lens::new("cm", 1),
				Lens::new("rn", 2),
			]),
		}),
	Operation::Remove{ label: "cm".to_owned() },
	Boxes::new(hashmap!
		{
			0 => VecDeque::from(vec![
				Lens::new("rn", 2),
			]),
		}),
)]
fn boxes_apply_works(#[case] mut subject: Boxes, #[case] operation: Operation, #[case] expected: Boxes)
{
	subject.apply(operation);

	assert_eq!(subject, expected);
}

#[test]
fn example_is_correct()
{
	let input = Cursor::new("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");

	let actual = solution(input);

	assert_eq!(actual, 145);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-15");
	let actual = solution_from_reader(&path, solution);

	assert_eq!(actual, 229349);
}
