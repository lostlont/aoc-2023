use std::collections::HashSet;
use std::io::Cursor;
use itertools::Itertools;
use maplit::hashset;
use rstest::*;
use aoc_2023::day17a::{Direction, Route, Step, get_direction, solution};

#[rstest]
#[case(
	Step::new(3, 2),
	Step::new(4, 2),
	Some(Direction::Right),
)]
#[case(
	Step::new(4, 2),
	Step::new(3, 2),
	Some(Direction::Left),
)]
#[case(
	Step::new(3, 2),
	Step::new(5, 2),
	None,
)]
#[case(
	Step::new(3, 1),
	Step::new(3, 2),
	Some(Direction::Down),
)]
#[case(
	Step::new(3, 2),
	Step::new(3, 1),
	Some(Direction::Up),
)]
#[case(
	Step::new(3, 1),
	Step::new(3, 3),
	None,
)]
fn get_direction_works(
	#[case] from_step: Step,
	#[case] to_step: Step,
	#[case] expected: Option<Direction>)
{
	let actual = get_direction(&from_step, &to_step);

	assert_eq!(actual, expected);
}

#[rstest]
#[case(&[], &[])]
#[case(&[Step::new(0, 0)], &[])]
#[case(
	&[
		Step::new(3, 2),
		Step::new(4, 2),
		Step::new(4, 3),
		Step::new(4, 4),
		Step::new(3, 4),
		Step::new(3, 3),
		Step::new(2, 3),
	],
	&[
		Direction::Left,
		Direction::Up,
		Direction::Left,
		Direction::Down,
		Direction::Down,
		Direction::Right,
	],
)]
fn route_get_last_directions_works(#[case] steps: &[Step], #[case] expected: &[Direction])
{
	let subject = Route::new(0, 0, 0, steps.into_iter().copied());

	let actual = subject
		.get_last_directions()
		.collect_vec();

	assert_eq!(actual, expected);
}

#[rstest]
#[case(
	&[],
	hashset!{ Direction::Left, Direction::Up, Direction::Right, Direction::Down },
)]
#[case(
	&[Step::new(0, 0)],
	hashset!{ Direction::Left, Direction::Up, Direction::Right, Direction::Down },
)]
#[case(
	&[
		Step::new(0, 0),
		Step::new(1, 0),
	],
	hashset!{ Direction::Up, Direction::Right, Direction::Down },
)]
#[case(
	&[
		Step::new(0, 0),
		Step::new(1, 0),
		Step::new(2, 0),
	],
	hashset!{ Direction::Up, Direction::Right, Direction::Down },
)]
#[case(
	&[
		Step::new(0, 0),
		Step::new(1, 0),
		Step::new(2, 0),
		Step::new(3, 0),
	],
	hashset!{ Direction::Up, Direction::Down },
)]
#[case(
	&[
		Step::new(0, 0),
		Step::new(1, 0),
		Step::new(2, 0),
		Step::new(2, 1),
		Step::new(2, 2),
		Step::new(2, 3),
	],
	hashset!{ Direction::Left, Direction::Right },
)]
#[case(
	&[
		Step::new(0, 0),
		Step::new(1, 0),
		Step::new(2, 0),
		Step::new(2, 1),
		Step::new(2, 2),
		Step::new(3, 2),
	],
	hashset!{ Direction::Up, Direction::Right, Direction::Down },
)]
fn route_get_next_directions_works(#[case] steps: &[Step], #[case] expected: HashSet<Direction>)
{
	let subject = Route::new(0, 0, 0, steps.into_iter().copied());

	let actual = subject.get_next_directions();

	assert_eq!(actual, expected);
}

#[test]
fn example_is_correct()
{
	let input = Cursor::new(
r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533");

	let actual = solution(input);

	assert_eq!(actual, 102);
}
