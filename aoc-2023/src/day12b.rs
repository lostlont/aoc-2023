use itertools::Itertools;
use super::day12a::solution as solution_a;
use super::day12a::Count;

pub fn solution(input: &Vec<&str>) -> Count
{
	let input = input
		.iter()
		.map(|line|
		{
			let (input_springs, input_damaged_springs) = line
				.split_once(' ')
				.expect(&format!("Line {line} can not be split at space!"));

			let unfolded_springs = std::iter
				::repeat(input_springs)
				.take(5)
				.join("?");

			let unfolded_damaged_springs = std::iter
				::repeat(input_damaged_springs)
				.take(5)
				.join(",");

			format!("{unfolded_springs} {unfolded_damaged_springs}")
		})
		.collect_vec();

	let input = input
		.iter()
		.map(|line| line.as_ref())
		.collect_vec();

	solution_a(&input)
}
