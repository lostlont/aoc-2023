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
pub mod day05a;
pub mod day06a;
pub mod day06b;
pub mod day07a;

pub fn solution_from<T>(path: &Path, solution: impl Fn(&Vec<&str>) -> T) -> T
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
