use std::path::Path;
use rstest::*;
use aoc_2023::solution_from;
use aoc_2023::day12a::next;
use aoc_2023::day12a::parse_spring;
use aoc_2023::day12a::process;
use aoc_2023::day12a::solution;
use aoc_2023::day12a::Spring;
use aoc_2023::day12a::State;
use aoc_2023::day12a::WorkEntry;

#[rstest]
#[case(
	State::new(Spring::Working, vec![3, 4]),
	Spring::Working,
	Some(State::new(Spring::Working, vec![3, 4])))]
#[case(
	State::new(Spring::Working, vec![3, 4]),
	Spring::Damaged,
	Some(State::new(Spring::Damaged, vec![2, 4])))]
#[case(
	State::new(Spring::Working, vec![]),
	Spring::Damaged,
	None)]
#[case(
	State::new(Spring::Damaged, vec![1, 4]),
	Spring::Working,
	None)]
#[case(
	State::new(Spring::Damaged, vec![0, 4]),
	Spring::Working,
	Some(State::new(Spring::Working, vec![4])))]
#[case(
	State::new(Spring::Damaged, vec![]),
	Spring::Working,
	None)]
#[case(
	State::new(Spring::Damaged, vec![3, 4]),
	Spring::Damaged,
	Some(State::new(Spring::Damaged, vec![2, 4])))]
#[case(
	State::new(Spring::Damaged, vec![0, 4]),
	Spring::Damaged,
	None)]
fn state_machine_next_works(
	#[case] subject: State,
	#[case] next_spring: Spring,
	#[case] expected: Option<State>)
{
	let actual = subject.next(next_spring);

	assert_eq!(actual, expected);
}

#[rstest]
#[case(
	WorkEntry::new(
		State::new(
			Spring::Working,
			vec![3, 4]),
		vec![],
		create_unknown_springs("."),
	),
	vec!
	[
		WorkEntry::new(
			State::new(
				Spring::Working,
				vec![3, 4]),
			create_known_springs("."),
			vec![]
		),
	]
)]
#[case(
	WorkEntry::new(
		State::new(
			Spring::Working,
			vec![3, 4]),
		vec![],
		create_unknown_springs("#"),
	),
	vec!
	[
		WorkEntry::new(
			State::new(
				Spring::Damaged,
				vec![2, 4]),
			create_known_springs("#"),
			vec![],
		),
	]
)]
#[case(
	WorkEntry::new(
		State::new(
			Spring::Working,
			vec![3, 4]),
		vec![],
		create_unknown_springs("?"),
	),
	vec!
	[
		WorkEntry::new(
			State::new(
				Spring::Working,
				vec![3, 4]),
			create_known_springs("."),
			vec![],
		),
		WorkEntry::new(
			State::new(
				Spring::Damaged,
				vec![2, 4]),
			create_known_springs("#"),
			vec![],
		),
	]
)]
fn next_works(#[case] work_entry: WorkEntry, #[case] expected: Vec<WorkEntry>)
{
	let actual = next(&work_entry);

	assert_eq!(actual, expected);
}

#[rstest]
#[case(
	create_unknown_springs("#.#.###"),
	vec![1, 1, 3],
	vec!
	[
		create_known_springs("#.#.###"),
	],
)]
#[case(
	create_unknown_springs("#.#.###"),
	vec![1, 1, 3],
	vec!
	[
		create_known_springs("#.#.###"),
	],
)]
fn process_works(#[case] springs: Vec<Option<Spring>>, #[case] damaged_springs: Vec<u32>, #[case] mut expected: Vec<Vec<Spring>>)
{
	let mut actual = vec![];
	process(springs, damaged_springs, |v| actual.push(v.clone()));
	actual.sort();

	expected.sort();
	assert_eq!(actual, expected);
}

#[test]
fn example_is_correct()
{
	let input =
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = solution(&input);

	assert_eq!(actual, 21);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-12");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 6981);
}

fn create_known_springs(input: &str) -> Vec<Spring>
{
	input
		.chars()
		.map(|c| parse_spring(c).unwrap())
		.collect::<Vec<_>>()
}

fn create_unknown_springs(input: &str) -> Vec<Option<Spring>>
{
	input
		.chars()
		.map(|c| parse_spring(c))
		.collect::<Vec<_>>()
}
