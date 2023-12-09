use itertools::Itertools;

pub type Num = i64;

pub fn derive_once(mut last_value: Num, next_values: impl AsRef<Vec<Num>>) -> Vec<Num>
{
	let next_values = next_values.as_ref();
	let mut result = vec![last_value];
	for index in 0..next_values.len()
	{
		let derived = next_values[index] - last_value;
		result.push(derived);
		last_value = derived;
	}
	result
}

pub fn derive(values: impl AsRef<Vec<Num>>) -> Vec<Num>
{
	let values = values.as_ref();
	let mut order = 0;
	let next_value = *values.last().unwrap();
	let mut next_values = vec![next_value];
	let mut result = vec![next_value];
	while next_values.last() != Some(&0)
	{
		let last_value = values[values.len() - 2 - order];
		next_values = derive_once(last_value, &next_values);
		let next_value = *next_values.last().unwrap();
		result.push(next_value);
		order += 1;
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
