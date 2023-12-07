use std::cmp::Ordering;
use itertools::Itertools;
use super::day07a::get_hand_type;
use super::day07a::tuples_from_labels;
use super::day07a::Bid;
use super::day07a::Hand;
use super::day07a::Label;

pub fn hand_from_jokers(labels: &str) -> Hand
{
	let labels = labels
		.chars()
		.map(Label::from)
		.collect_vec();

	let mut tuples = tuples_from_labels(&labels);

	if let Some((best_index, best_label)) = tuples
		.iter()
		.enumerate()
		.flat_map(|(index, labels)| labels.iter().map(move |label| (index, *label)))
		.rev()
		.filter(|(_index, label)| *label != Label::Joker)
		.next()
	{
		let joker_count = labels
			.iter()
			.cloned()
			.filter(|&l| l == Label::Joker)
			.count();

		tuples[best_index].retain(|&l| l != best_label);
		tuples
			.iter_mut()
			.for_each(|labels| labels.retain(|&l| l != Label::Joker));
		tuples[best_index + joker_count].push(best_label);
	}
	else
	{
		tuples = [vec![], vec![], vec![], vec![], vec![Label::Ace]];
	}

	let hand_type = get_hand_type(&tuples);

	Hand
	{
		labels,
		tuples,
		hand_type,
	}
}

pub fn bid_from_jokers(value: &str) -> Bid
{
	let (input_hand, input_bid) = value
		.split_once(' ')
		.expect(&format!("Input value \"{value}\" should have two values!"));

	Bid
	{
		hand: hand_from_jokers(input_hand),
		bid: input_bid.parse().expect(&format!("Input value \"{input_bid}\" can not be parsed as u32!")),
	}
}

pub fn parse_bids(input: &Vec<&str>) -> Vec<Bid>
{
	input
		.iter()
		.cloned()
		.map(bid_from_jokers)
		.collect()
}

pub fn solution(input: &Vec<&str>) -> u32
{
	let mut bids = parse_bids(input);
	bids.sort_by(|a, b|
		{
			match a.hand.hand_type.cmp(&b.hand.hand_type)
			{
				Ordering::Equal => {},
				ord => return ord,
			};

			for labels in a.hand.labels
				.iter()
				.cloned()
				.zip(b.hand.labels
					.iter()
					.cloned())
			{
				if labels.0 == Label::Joker && labels.1 != Label::Joker
				{
					return Ordering::Less;
				}
				else if labels.0 != Label::Joker && labels.1 == Label::Joker
				{
					return Ordering::Greater;
				}
				else
				{
					match labels.0.cmp(&labels.1)
					{
						Ordering::Equal => {},
						ord => return ord,
					}
				}
			}

			Ordering::Equal
		});

	bids
		.iter()
		.enumerate()
		.map(|(index, bid)| bid.bid * (index as u32 + 1))
		.sum()
}
