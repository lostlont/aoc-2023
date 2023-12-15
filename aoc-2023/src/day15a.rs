use std::io::BufRead;

pub fn solution(input: impl BufRead) -> u32
{
	input
		.lines()
		.next()
		.expect("Input should contain a line!")
		.iter()
		.flat_map(|line| line.split(','))
		.map(|part| part
			.chars()
			.map(u32::from)
			.fold(0, |acc, b| ((acc + b) * 17) % 256))
		.sum()
}
