use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub fn filter_numbers(text: &str) -> Vec<u32>
{
	text
		.chars()
		.filter_map(|c| c.to_digit(10))
		.collect::<Vec<u32>>()
}

pub fn get_calibration_value(numbers: &Vec<u32>) -> u32
{
	let first = numbers
		.first()
		.expect("No first number!");
	let last = numbers
		.last()
		.expect("No last number!");
	first * 10 + last
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
