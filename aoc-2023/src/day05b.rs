use std::ops::Range;
use super::day05a::expect_i64;
use crate::day05a::parse_mapping;
use crate::day05a::Mapping;

pub type Num = i64;

pub fn parse_seeds(input: &str) -> Vec<Range<Num>>
{
	const PREFIX : &str = "seeds:";
	let input = input.strip_prefix(PREFIX)
		.expect(&format!("Line should start with \"{PREFIX}\"!"));

	let mut iter = input
		.split_whitespace()
		.map(expect_i64);

	let mut result = vec![];
	while let (Some(start), Some(length)) = (iter.next(), iter.next())
	{
		result.push(start..start+length);
	}
	result
}

pub fn map_ranges(input: &Vec<Range<Num>>, mapping: &Mapping) -> impl Iterator<Item = Range<Num>> + '_
{
}
/*
pub fn parse_values<'a>(input: &'a Vec<&str>) -> Vec<Num>
{
	let mut ranges = parse_seeds(input[0])
		.iter()
		.cloned()
		.collect::<Vec<_>>();
	let mut category_result = (0..numbers.len())
		.map(|_| None)
		.collect::<Vec<_>>();

	for line in input.iter().skip(1)
	{
		if line.is_empty()
		{
		}
		else if line.ends_with("map:")
		{
			/*
			numbers = category_result
				.iter()
				.zip(numbers)
				.map(|(new, old)| new.unwrap_or(old))
				.collect::<Vec<_>>();
			*/
		}
		else
		{
			/*
			let mapping = parse_mapping(&line);

			category_result = mapping
				.map_values(&numbers)
				.zip(category_result)
				.map(|(new, old)| new.or(old))
				.collect::<Vec<_>>();
			*/
		}
	}

	category_result
		.iter()
		.zip(numbers)
		.map(|(new, old)| new.unwrap_or(old))
		.collect::<_>()
}
*/
/*
pub fn solution(input: &Vec<&str>) -> Num
{
	parse_values(input)
		.iter()
		.cloned()
		.min()
		.expect("No minimum exists!")
}
*/
