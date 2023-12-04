use std::collections::VecDeque;
use super::day04a::find_winning_numbers;
use super::day04a::parse_card;

pub fn solution(input: &Vec<&str>) -> u32
{
	input
		.iter()
		.map(|l| parse_card(l))
		.map(|c| find_winning_numbers(&c).count() as u32)
		.scan(VecDeque::new(), |extras : &mut VecDeque<u32>, n|
			{
				let card_count = 1 + extras
					.pop_front()
					.unwrap_or(0);

				let extras_length = extras.len().max(n as usize);
				let new_extras = (0..extras_length)
					.map(|i| extras
						.get(i)
						.unwrap_or(&0)
						+ if (i as u32) < n { card_count } else { 0 })
					.collect::<Vec<_>>();
				*extras = VecDeque::from(new_extras);
				Some(card_count)
			})
		.sum()
}
