use std::path::Path;
use rstest::rstest;
use aoc_2023::solution_from;
use aoc_2023::day07a::parse_bids;
use aoc_2023::day07a::solution;
use aoc_2023::day07a::Bid;
use aoc_2023::day07a::Hand;
use aoc_2023::day07a::Label;
use aoc_2023::day07a::Type;

#[rstest]
#[case('2', Label::Two)]
#[case('3', Label::Three)]
#[case('4', Label::Four)]
#[case('5', Label::Five)]
#[case('6', Label::Six)]
#[case('7', Label::Seven)]
#[case('8', Label::Eight)]
#[case('9', Label::Nine)]
#[case('T', Label::Ten)]
#[case('J', Label::Joker)]
#[case('Q', Label::Queen)]
#[case('K', Label::King)]
#[case('A', Label::Ace)]
fn label_from_char_works(#[case] input: char, #[case] expected: Label)
{
	let actual = Label::from(input);

	assert_eq!(actual, expected);
}

#[rstest]
#[case(
	"QQQJJ",
	[vec![], vec![Label::Joker], vec![Label::Queen], vec![], vec![]])]
#[case(
	"QQQJK",
	[vec![Label::Joker, Label::King], vec![], vec![Label::Queen], vec![], vec![]])]
#[case(
	"23456",
	[vec![Label::Two, Label::Three, Label::Four, Label::Five, Label::Six], vec![], vec![], vec![], vec![]])]
fn hand_from_str_has_correct_tuples(#[case] input: &str, #[case] expected: [Vec<Label>; 5])
{
	let actual = Hand::from(input).tuples;

	assert_eq!(actual, expected);
}

#[rstest]
#[case("23456", Type::HighCard)]
#[case("23452", Type::OnePair)]
#[case("23432", Type::TwoPair)]
#[case("23252", Type::ThreeOfAKind)]
#[case("23232", Type::FullHouse)]
#[case("23222", Type::FourOfAKind)]
#[case("22222", Type::FiveOfAKind)]
fn hand_from_str_has_correct_type(#[case] input: &str, #[case] expected: Type)
{
	let actual = Hand::from(input).hand_type;

	assert_eq!(actual, expected);
}

#[test]
fn bid_from_str_has_correct_hand()
{
	let actual = Bid::from("32T3K 765").hand;

	let expected = Hand{
		labels: vec![Label::Three, Label::Two, Label::Ten, Label::Three, Label::King],
		tuples: [vec![Label::Two, Label::Ten, Label::King], vec![Label::Three], vec![], vec![], vec![]],
		hand_type: Type::OnePair,
	};
	assert_eq!(actual, expected);
}

#[test]
fn bid_from_str_has_correct_bid()
{
	let actual = Bid::from("32T3K 765").bid;

	assert_eq!(actual, 765);
}

#[test]
fn parse_bids_works()
{
	let input = vec![
		"32T3K 765",
		"T55J5 684",
	];

	let actual = parse_bids(&input);

	let expected = vec![
		Bid
		{
			hand: Hand
			{
				labels: vec![Label::Three, Label::Two, Label::Ten, Label::Three, Label::King],
				tuples: [vec![Label::Two, Label::Ten, Label::King], vec![Label::Three], vec![], vec![], vec![]],
				hand_type: Type::OnePair,
			},
			bid: 765,
		},
		Bid
		{
			hand: Hand
			{
				labels: vec![Label::Ten, Label::Five, Label::Five, Label::Joker, Label::Five],
				tuples: [vec![Label::Ten, Label::Joker], vec![], vec![Label::Five], vec![], vec![]],
				hand_type: Type::ThreeOfAKind,
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

	assert_eq!(actual, 6440);
}

#[test]
fn hand_order_is_correct()
{
	let hand33332 = Hand
	{
		labels: vec![Label::Three, Label::Three, Label::Three, Label::Three, Label::Two],
		hand_type: Type::FourOfAKind,
		tuples: [vec![Label::Two], vec![], vec![], vec![Label::Three], vec![]],
	};

	let hand2aaaa = Hand
	{
		labels: vec![Label::Two, Label::Ace, Label::Ace, Label::Ace, Label::Ace],
		hand_type: Type::FourOfAKind,
		tuples: [vec![Label::Two], vec![], vec![], vec![Label::Three], vec![]],
	};

	let mut actual = vec![
		hand33332.clone(),
		hand2aaaa.clone(),
	];

	actual.sort();

	let expected = vec![
		hand2aaaa,
		hand33332,
	];
	assert_eq!(actual, expected);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-07");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 248396258);
}

