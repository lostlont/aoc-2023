use std::cmp::Ordering;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Label
{
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Joker,
	Queen,
	King,
	Ace,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Type
{
	HighCard,
	OnePair,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hand
{
	pub labels: Vec<Label>,
	pub tuples: [Vec<Label>; 5],
	pub hand_type: Type,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Bid
{
	pub hand: Hand,
	pub bid: u32,
}

impl From<char> for Label
{
	fn from(value: char) -> Self
	{
		match value
		{
			'2' => Label::Two,
			'3' => Label::Three,
			'4' => Label::Four,
			'5' => Label::Five,
			'6' => Label::Six,
			'7' => Label::Seven,
			'8' => Label::Eight,
			'9' => Label::Nine,
			'T' => Label::Ten,
			'J' => Label::Joker,
			'Q' => Label::Queen,
			'K' => Label::King,
			'A' => Label::Ace,
			_ => panic!("Invalid label '{value}'!"),
		}
	}
}

impl From<&str> for Hand
{
	fn from(labels: &str) -> Self
	{
		let mut tuples: [Vec<Label>; 5] = (0..5)
			.map(|_| vec![])
			.collect::<Vec<_>>()
			.try_into()
			.expect("Tuples should have five elements!");

		let labels = labels
			.chars()
			.map(Label::from)
			.collect_vec();

		labels
			.iter()
			.cloned()
			.counts()
			.iter()
			.for_each(|(&label, count)| tuples[count - 1].push(label));

		tuples
			.iter_mut()
			.for_each(|labels| labels.sort());

		let counts = tuples
			.iter()
			.map(|t| t.len())
			.collect::<Vec<_>>();
		let hand_type = match counts.as_slice()
		{
			[5, 0, 0, 0, 0] => Type::HighCard,
			[3, 1, 0, 0, 0] => Type::OnePair,
			[1, 2, 0, 0, 0] => Type::TwoPair,
			[2, 0, 1, 0, 0] => Type::ThreeOfAKind,
			[0, 1, 1, 0, 0] => Type::FullHouse,
			[1, 0, 0, 1, 0] => Type::FourOfAKind,
			[0, 0, 0, 0, 1] => Type::FiveOfAKind,
			counts => panic!("Invalid hand with card counts {counts:?}!"),
		};

		Self
		{
			labels,
			tuples,
			hand_type,
		}
	}
}

impl Ord for Hand
{
	fn cmp(&self, other: &Self) -> Ordering
	{
		match self.hand_type.cmp(&other.hand_type)
		{
			Ordering::Equal => {},
			ord => return ord,
		}

		for labels in self.labels
			.iter()
			.zip(other.labels.iter())
		{
			match labels.0.cmp(labels.1)
			{
				Ordering::Equal => {},
				ord => return ord,
			}
		}

		return Ordering::Equal;
	}
}

impl PartialOrd for Hand
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl From<&str> for Bid
{
	fn from(value: &str) -> Self
	{
		let (input_hand, input_bid) = value
			.split_once(' ')
			.expect(&format!("Input value \"{value}\" should have two values!"));

		Self
		{
			hand: input_hand.into(),
			bid: input_bid.parse().expect(&format!("Input value \"{input_bid}\" can not be parsed as u32!")),
		}
	}
}

pub fn parse_bids(input: &Vec<&str>) -> Vec<Bid>
{
	input
		.iter()
		.cloned()
		.map(Bid::from)
		.collect()
}

pub fn solution(input: &Vec<&str>) -> u32
{
	let mut bids = parse_bids(input);
	bids.sort();

	bids
		.iter()
		.enumerate()
		.map(|(index, bid)| bid.bid * (index as u32 + 1))
		.sum()
}
