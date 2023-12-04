use std::collections::HashSet;
use std::path::Path;
use maplit::hashset;
use rstest::rstest;
use aoc_2023::solution_from;
use aoc_2023::day04a::find_winning_numbers;
use aoc_2023::day04a::parse_card;
use aoc_2023::day04a::parse_numbers;
use aoc_2023::day04a::score;
use aoc_2023::day04a::solution;
use aoc_2023::day04a::Card;

#[test]
fn parse_numbers_works()
{
	let actual = parse_numbers(" 3 2 43  5 1 99")
		.collect::<HashSet<_>>();

	assert_eq!(&actual, &hashset!{ 3, 2, 43, 5, 1, 99 });
}

#[test]
fn parse_card_works()
{
	let actual = parse_card("Card 1: 3 2 43 | 5 1 99");

	let expected = Card
	{
		have_numbers: hashset!{ 3, 2, 43 },
		winning_numbers: hashset!{ 5, 1, 99 },
	};
	assert_eq!(&actual, &expected);
}

#[test]
fn find_winning_numbers_works()
{
	let card = Card
	{
		have_numbers: hashset!{ 3, 2, 4, 43 },
		winning_numbers: hashset!{ 11, 43, 5, 2, 7 },
	};

	let actual = find_winning_numbers(&card)
		.collect::<HashSet<_>>();

	let expected = hashset!{ 2, 43 };
	assert_eq!(&actual, &expected);
}

#[rstest]
#[case(0, 0)]
#[case(1, 1)]
#[case(2, 2)]
#[case(3, 4)]
#[case(4, 8)]
fn score_works(#[case] number_count: u32, #[case] expected: u32)
{
	let actual = score(number_count);

	assert_eq!(actual, expected);
}

#[test]
fn example_is_correct()
{
	let input = vec![
		"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
		"Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
		"Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
		"Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
		"Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
		"Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
	];

	let actual = solution(&input);

	assert_eq!(actual, 13);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-04");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 25174);
}
