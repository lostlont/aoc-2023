use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct Record
{
	pub id: u32,
	pub sets: Vec<Set>,
}

#[derive(Debug, PartialEq)]
pub struct Set
{
	pub r: u32,
	pub g: u32,
	pub b: u32,
}

impl Set
{
	pub fn empty() -> Set
	{
		Set
		{
			r: 0,
			g: 0,
			b: 0,
		}
	}
}

pub fn parse_set(input: &str) -> Set
{
	let mut result = Set::empty();

	for amount_color in input.split(',').map(|s| s.trim())
	{
		let (amount, color) = amount_color
			.split_once(' ')
			.expect(&format!("Draw \"{amount_color}\" in line can not be split to amount and color!"));

		let amount = amount
			.parse::<u32>()
			.expect(&format!("Amount \"{amount}\" in set can not be parsed as u32!"));

		let target = match color
		{
			"red" => &mut result.r,
			"green" => &mut result.g,
			"blue" => &mut result.b,
			_ => panic!("Unknown color \"{color}\" in set!"),
		};

		*target += amount;
	}

	result
}

pub fn parse_record(input: &str) -> Record
{
	let input = input
		.strip_prefix("Game ")
		.expect("Line should start with \"Game\"!");

	let parts = input
		.split_once(": ")
		.expect("Line should be splittable by \": \"!");

	let part0 = parts.0;
	let id = part0
		.parse::<u32>()
		.expect(&format!("Record first part \"{part0}\" can not be parsed as a number!"));

	let input = parts.1;

	let sets = input
		.split(';')
		.map(|s| parse_set(s))
		.collect::<Vec<_>>();

	Record
	{
		id,
		sets,
	}
}

pub fn solution(input: &Vec<&str>) -> u32
{
	input
		.iter()
		.cloned()
		.map(parse_record)
		.filter(|r| r.sets
			.iter()
			.all(|s| (s.r <= 12) && (s.g <= 13) && (s.b <= 14)))
		.map(|r| r.id)
		.sum()
}

pub fn solution_from(path: &Path) -> u32
{
	let file = File::open(&path)
		.expect(
			&format!(
				"Couldn't open file {}!",
				path.display()));
	let reader = BufReader::new(file);
	let input = reader
		.lines()
		.map(
			|line| line.expect("Couldn't read line!"))
		.collect::<Vec<_>>();

	let input = input
		.iter()
		.map(|line| line.as_ref())
		.collect();

	solution(&input)
}
