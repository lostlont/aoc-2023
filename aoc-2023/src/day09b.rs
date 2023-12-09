use itertools::Itertools;
use super::day09a::derive_once;
use super::day09a::Num;

pub fn derive_back(values: impl AsRef<Vec<Num>>) -> Vec<Num>
{
	let mut values = values.as_ref().clone();
	let mut result = vec![values[0]];
	while values.iter().any(|&v| v != 0)
	{
		values = derive_once(values);
		result.push(values[0]);
	}
	result
}

pub fn extrapolate_back(values: impl AsRef<Vec<Num>>) -> Num
{
	let derived = derive_back(values);
	derived
		.iter()
		.cloned()
		.rev()
		.reduce(|acc, v| v - acc)
		.unwrap()
}

pub fn solution(input: &Vec<&str>) -> Num
{
	input
		.iter()
		.map(|line| line
			.split_whitespace()
			.map(|value| value
				.parse::<Num>()
				.expect(&format!("Value \"{value}\" can not be parsed as a number!")))
			.collect_vec())
		.map(extrapolate_back)
		.sum()
}
