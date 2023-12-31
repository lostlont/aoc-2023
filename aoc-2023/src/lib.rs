use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub mod common;
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
pub mod day07b;
pub mod day08a;
pub mod day09a;
pub mod day09b;
pub mod day10a;
pub mod day11a;
pub mod day12a;
pub mod day13a;
pub mod day14a;
pub mod day14b;
pub mod day15a;
pub mod day15b;
pub mod day16a;
pub mod day16b;

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

pub fn solution_from_reader<TSolution, TResult>(path: &Path, solution: TSolution) -> TResult
where
	TSolution: Fn(BufReader<File>) -> TResult,
{
	let file = File::open(&path)
		.expect(
			&format!(
				"Couldn't open file {}!",
				path.display()));
	let reader = BufReader::new(file);

	solution(reader)
}
