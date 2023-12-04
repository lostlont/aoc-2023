use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub mod day01a;
pub mod day01b;
pub mod day02a;
pub mod day02b;
pub mod day03a;
pub mod day03b;
pub mod day04a;
pub mod day04b;

pub fn solution_from(path: &Path, solution: impl Fn(&Vec<&str>) -> u32) -> u32
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
