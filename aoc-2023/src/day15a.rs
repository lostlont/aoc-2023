use std::io::BufRead;

pub fn hash(value: &str) -> u8
{
	value
		.chars()
		.map(|c| u32::try_from(c).unwrap())
		.fold(0, |acc: u32, b| ((acc + b as u32) * 17) % 256)
		as u8
}

pub fn solution(input: impl BufRead) -> u32
{
	input
		.lines()
		.next()
		.expect("Input should contain a line!")
		.iter()
		.flat_map(|line| line.split(','))
		.map(|part| hash(part) as u32)
		.sum()
}
