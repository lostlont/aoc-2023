use crate::day02a::parse_record;
use crate::day02a::Set;

pub fn minimum_set(sets: &Vec<Set>) -> Set
{
	let mut result = Set::empty();
	for set in sets
	{
		result.r = result.r.max(set.r);
		result.g = result.g.max(set.g);
		result.b = result.b.max(set.b);
	}
	result
}

pub fn solution(input: &Vec<&str>) -> u32
{
	input
		.iter()
		.cloned()
		.map(parse_record)
		.map(|r| minimum_set(&r.sets))
		.map(|s| s.r * s.g * s.b)
		.sum()
}
