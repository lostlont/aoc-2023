use std::path::Path;
use rstest::rstest;
use aoc_2023::solution_from;
use aoc_2023::day07a::Bid;
use aoc_2023::day07a::Hand;
use aoc_2023::day07a::Label;
use aoc_2023::day07a::Type;
use aoc_2023::day07b::bid_from_jokers;
use aoc_2023::day07b::hand_from_jokers;
use aoc_2023::day07b::parse_bids;
use aoc_2023::day07b::solution;

#[rstest]
#[case(
	"23456",
	[vec![Label::Two, Label::Three, Label::Four, Label::Five, Label::Six], vec![], vec![], vec![], vec![]])]
#[case(
	"QQQJK",
	[vec![Label::King], vec![], vec![], vec![Label::Queen], vec![]])]
#[case(
	"QQQJJ",
	[vec![], vec![], vec![], vec![], vec![Label::Queen]])]
#[case(
	"3J4J2",
	[vec![Label::Two, Label::Three], vec![], vec![Label::Four], vec![], vec![]])]
#[case(
	"JJJJJ",
	[vec![], vec![], vec![], vec![], vec![Label::Ace]])]
fn hand_from_jokers_has_correct_tuples(#[case] input: &str, #[case] expected: [Vec<Label>; 5])
{
	let actual = hand_from_jokers(input).tuples;

	assert_eq!(actual, expected);
}

#[rstest]
#[case("23456", Type::HighCard)]
#[case("QQQJK", Type::FourOfAKind)]
#[case("QQQJJ", Type::FiveOfAKind)]
#[case("3J4J2", Type::ThreeOfAKind)]
#[case("JJJJJ", Type::FiveOfAKind)]
fn hand_from_jokers_has_correct_type(#[case] input: &str, #[case] expected: Type)
{
	let actual = hand_from_jokers(input).hand_type;

	assert_eq!(actual, expected);
}


#[test]
fn bid_from_jokers_has_correct_hand()
{
	let actual = bid_from_jokers("32J3K 765").hand;

	let expected = Hand
	{
		labels: vec![Label::Three, Label::Two, Label::Joker, Label::Three, Label::King],
		tuples: [vec![Label::Two, Label::King], vec![], vec![Label::Three], vec![], vec![]],
		hand_type: Type::ThreeOfAKind,
	};
	assert_eq!(actual, expected);
}

#[test]
fn bid_from_jokers_has_correct_bid()
{
	let actual = bid_from_jokers("32J3K 765").bid;

	assert_eq!(actual, 765);
}

#[test]
fn parse_bids_works()
{
	let input = vec![
		"32J3K 765",
		"T55J5 684",
	];

	let actual = parse_bids(&input);

	let expected = vec![
		Bid
		{
			hand: Hand
			{
				labels: vec![Label::Three, Label::Two, Label::Joker, Label::Three, Label::King],
				tuples: [vec![Label::Two, Label::King], vec![], vec![Label::Three], vec![], vec![]],
				hand_type: Type::ThreeOfAKind,
			},
			bid: 765,
		},
		Bid
		{
			hand: Hand
			{
				labels: vec![Label::Ten, Label::Five, Label::Five, Label::Joker, Label::Five],
				tuples: [vec![Label::Ten], vec![], vec![], vec![Label::Five], vec![]],
				hand_type: Type::FourOfAKind,
			},
			bid: 684,
		},
	];
	assert_eq!(actual, expected);
}

#[test]
fn example_is_correct()
{
	let input =
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.collect::<Vec<_>>();

	let actual = solution(&input);

	assert_eq!(actual, 5905);
}

#[test]
fn solution_without_jokers_uses_correct_ordering()
{
	let input =
"33332 10
2AAAA 20
";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.collect::<Vec<_>>();

	let actual = solution(&input);

	assert_eq!(actual, 1*20 + 2*10);
}

#[test]
fn solution_with_jokers_uses_correct_ordering_for_type()
{
	let input =
"333J2 10
2JJJJ 20
";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.collect::<Vec<_>>();

	let actual = solution(&input);

	assert_eq!(actual, 1*10 + 2*20);
}

#[test]
fn solution_with_jokers_uses_correct_ordering_for_jokers()
{
	let input =
"33333 10
JJJJJ 20
";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
		.collect::<Vec<_>>();

	let actual = solution(&input);

	assert_eq!(actual, 1*20 + 2*10);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-07");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 246436046);
}
