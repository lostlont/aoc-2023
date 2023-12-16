use rstest::*;
use aoc_2023::common::map::Map;

#[test]
fn map_from_vec_str_works_with_char()
{
	let input = vec!
	[
		".#.",
		"O.#",
	];

	let actual: Map<char> = Map::from(input.as_slice());

	let expected = Map::new(
		".#.O.#".chars(),
		3,
		2);
	assert_eq!(actual, expected);
}

#[derive(Debug, PartialEq)]
enum EnumValue{ Left, Right }

impl From<char> for EnumValue
{
	fn from(value: char) -> Self
	{
		match value
		{
			'L' => EnumValue::Left,
			'R' => EnumValue::Right,
			_ => panic!("Character '{value}' can not be parsed as EnumValue!"),
		}
	}
}

#[test]
fn map_from_vec_str_works_with_enum()
{
	let input = vec!
	[
		"RLL",
		"LLR",
	];

	let actual: Map<EnumValue> = Map::from(input.as_slice());

	let expected = Map::new(
		[
			EnumValue::Right, EnumValue::Left, EnumValue::Left,
			EnumValue::Left, EnumValue::Left, EnumValue::Right,
		],
		3,
		2);
	assert_eq!(actual, expected);
}

#[test]
fn map_from_str_parser_works_with_option_enum()
{
	let input = vec!
	[
		"RL.",
		"L.R",
	];

	let parser = |char| match char
	{
		'L' => Some(EnumValue::Left),
		'R' => Some(EnumValue::Right),
		'.' => None,
		_ => panic!("Character '{char}' can not be parsed as EnumValue!"),
	};
	let actual: Map<Option<EnumValue>> = Map::from_str_parser(input.as_slice(), parser);

	let expected = Map::new(
		[
			Some(EnumValue::Right), Some(EnumValue::Left), None,
			Some(EnumValue::Left), None, Some(EnumValue::Right),
		],
		3,
		2);
	assert_eq!(actual, expected);
}

#[rstest]
#[case(0, 0, Some('.'))]
#[case(1, 0, Some('#'))]
#[case(3, 0, None)]
#[case(0, 1, Some('O'))]
#[case(1, 1, Some('.'))]
#[case(2, 1, Some('#'))]
#[case(0, 2, None)]
fn map_at_works_with_char(#[case] x: usize, #[case] y: usize, #[case] expected: Option<char>)
{
	let subject: Map<char> = Map::new(
		[
			".#.",
			"O.#",
		].join("").chars(),
		3,
		2);

	let actual = subject
		.at(x, y)
		.cloned();

	assert_eq!(actual, expected);
}

#[rstest]
#[case(0, 0, Some(EnumValue::Right))]
#[case(1, 0, Some(EnumValue::Left))]
#[case(3, 0, None)]
#[case(0, 1, Some(EnumValue::Left))]
#[case(1, 1, Some(EnumValue::Left))]
#[case(2, 1, Some(EnumValue::Right))]
#[case(0, 2, None)]
fn map_at_works_with_enum(#[case] x: usize, #[case] y: usize, #[case] expected: Option<EnumValue>)
{
	let subject: Map<EnumValue> = Map::new(
		[
			EnumValue::Right, EnumValue::Left, EnumValue::Left,
			EnumValue::Left, EnumValue::Left, EnumValue::Right,
		],
		3,
		2);

	let actual = subject.at(x, y);

	let expected = expected.as_ref();
	assert_eq!(actual, expected);
}
