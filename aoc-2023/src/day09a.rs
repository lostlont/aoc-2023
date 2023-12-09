use itertools::Itertools;

pub type Num = i64;

pub fn derive_once(values: impl AsRef<Vec<Num>>) -> Vec<Num>
{
	let values = values.as_ref();
	let mut result = vec![];
	result.reserve(values.len() - 1);
	for index in 1..values.len()
	{
		let last_value = values[index - 1];
		let next_value = values[index];
		result.push(next_value - last_value);
	}
	result
}

pub fn derive(values: impl AsRef<Vec<Num>>) -> Vec<Num>
{
	let mut values = values.as_ref().clone();
	let mut result = vec![values[values.len() - 1]];
	while values.iter().any(|&v| v != 0)
	{
		values = derive_once(values);
		result.push(values[values.len() - 1]);
	}
	result
}

pub fn extrapolate(values: impl AsRef<Vec<Num>>) -> Num
{
	let derived = derive(values);
	derived
		.iter()
		.cloned()
		.rev()
		.reduce(|acc, v| acc + v)
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
		.map(extrapolate)
		.sum()
}
