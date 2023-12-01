use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use crate::day01a::get_calibration_value;

pub fn filter_number(text: &str) -> Option<u32>
{
	if let Ok(number) = text[0..1].parse::<u32>()
	{
		return Some(number);
	}

	if text.starts_with("one")
	{
		return Some(1);
	}
	else if text.starts_with("two")
	{
		return Some(2);
	}
	else if text.starts_with("three")
	{
		return Some(3);
	}
	else if text.starts_with("four")
	{
		return Some(4);
	}
	else if text.starts_with("five")
	{
		return Some(5);
	}
	else if text.starts_with("six")
	{
		return Some(6);
	}
	else if text.starts_with("seven")
	{
		return Some(7);
	}
	else if text.starts_with("eight")
	{
		return Some(8);
	}
	else if text.starts_with("nine")
	{
		return Some(9);
	}

	return None
}

pub fn filter_numbers(text: &str) -> Vec<u32>
{
	let mut result = vec![];

	for i in 0..text.len()
	{
		if let Some(number) = filter_number(&text[i..])
		{
			result.push(number);
		}
	}

	result
}

pub fn solution(input: &Vec<&str>) -> u32
{
	input
		.iter()
		.cloned()
		.map(filter_numbers)
		.map(|numbers| get_calibration_value(&numbers))
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
