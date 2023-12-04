use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Card
{
	pub have_numbers: HashSet<u32>,
	pub winning_numbers: HashSet<u32>,
}

pub fn parse_numbers(input: &str) -> impl Iterator<Item = u32> + '_
{
	input
		.split_whitespace()
		.map(|n| n.trim())
		.map(move |n| n.parse::<u32>().expect(&format!("Value \"{n}\" in \"{input}\" can not be parsed as u32!")))
}

pub fn parse_card(input: &str) -> Card
{
	let input_numbers = input
		.split_once(':')
		.expect("Line should have a ':'!")
		.1;

	let (input_have_numbers, input_winning_numbers) = input_numbers
		.split_once('|')
		.expect("Line should have a '|'!");

	let have_numbers = parse_numbers(input_have_numbers)
		.collect();
	let winning_numbers = parse_numbers(input_winning_numbers)
		.collect();
	Card
	{
		have_numbers,
		winning_numbers,
	}
}

pub fn find_winning_numbers(card: &Card) -> impl Iterator<Item = u32> + '_
{
	card.have_numbers
		.iter()
		.cloned()
		.filter(|n| card.winning_numbers.contains(n))
}

pub fn score(number_count: u32) -> u32
{
	if number_count > 0
	{
		2_u32.pow(number_count - 1)
	}
	else
	{
		0
	}
}

pub fn solution(input: &Vec<&str>) -> u32
{
	input
		.iter()
		.map(|l| parse_card(l))
		.map(|c| find_winning_numbers(&c).count() as u32)
		.map(score)
		.sum()
}
