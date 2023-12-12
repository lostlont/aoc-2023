use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Spring
{
	Working,
	Damaged,
}

#[derive(Clone, Debug, PartialEq)]
pub struct State
{
	last_spring: Spring,
	remaining_damaged: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorkEntry
{
	state: State,
	processed_springs: Vec<Spring>,
	remaining_springs: Vec<Option<Spring>>,
}

pub fn parse_spring(value: char) -> Option<Spring>
{
	match value
	{
		'.' => Some(Spring::Working),
		'#' => Some(Spring::Damaged),
		'?' => None,
		_ => panic!("Value '{value}' can not be parsed as a spring!"),
	}
}

impl State
{
	pub fn new(last_spring: Spring, remaining_damaged: Vec<u32>) -> State
	{
		Self
		{
			last_spring,
			remaining_damaged,
		}
	}

	pub fn next(&self, spring: Spring) -> Option<State>
	{
		let remaining_damaged = if spring == Spring::Working
		{
			if self.last_spring == Spring::Damaged
			{
				if self.remaining_damaged.is_empty()
				{
					return None;
				}

				if self.remaining_damaged[0] == 0
				{
					self.remaining_damaged
						.iter()
						.cloned()
						.skip(1)
						.collect_vec()
				}
				else
				{
					return None;
				}
			}
			else
			{
				self.remaining_damaged.clone()
			}
		}
		else
		{
			if self.remaining_damaged.is_empty()
			{
				return None;
			}

			let first_value = self.remaining_damaged[0];
			if first_value == 0
			{
				return None;
			}

			let first_value = first_value - 1;
			std::iter::once(first_value)
				.chain(self.remaining_damaged
					.iter()
					.skip(1)
					.cloned())
				.collect_vec()
		};
		let result = State::new(spring, remaining_damaged);
		Some(result)
	}
}

impl WorkEntry
{
	pub fn new(state: State, processed_springs: Vec<Spring>, remaining_springs: Vec<Option<Spring>>) -> Self
	{
		Self
		{
			state,
			processed_springs,
			remaining_springs,
		}
	}
}

pub fn next(work_entry: &WorkEntry) -> Vec<WorkEntry>
{
	let mut result = vec![];

	let next_springs = work_entry.remaining_springs
		.first()
		.unwrap()
		.map_or(vec![Spring::Working, Spring::Damaged], |next_spring| vec![next_spring]);

	for next_spring in next_springs
	{
		if let Some(next_state) = work_entry.state.next(next_spring)
		{
			let processed_springs = work_entry.processed_springs
				.iter()
				.cloned()
				.chain(std::iter::once(next_spring))
				.collect::<Vec<_>>();

			let remaining_springs = work_entry.remaining_springs[1..].to_owned();

			let next_work_entry = WorkEntry::new(next_state, processed_springs, remaining_springs);
			result.push(next_work_entry);
		}
	}

	result
}

pub fn process(
	springs: Vec<Option<Spring>>,
	damaged_springs: Vec<u32>,
	mut callback: impl FnMut(&Vec<Spring>) -> ())
{
	let mut stack = vec!
	[
		WorkEntry::new(
			State::new(
				Spring::Working,
				damaged_springs),
			vec![],
			springs
		),
	];

	while let Some(work_entry) = stack.pop()
	{
		let next_work_entries = next(&work_entry);

		next_work_entries
			.iter()
			.filter(|we| we.remaining_springs.is_empty())
			.filter(|we| we.state.remaining_damaged.is_empty() || we.state.remaining_damaged == vec![0])
			.map(|we| we.processed_springs.clone())
			.for_each(|v| callback(&v));

		let next_work_entries = next_work_entries
			.iter()
			.cloned()
			.filter(|we| !we.remaining_springs.is_empty());
		stack.extend(next_work_entries);
	}
}

pub fn solution(input: &Vec<&str>) -> u32
{
	input
		.iter()
		.map(|line|
		{
			let (input_springs, input_damaged_springs) = line
				.split_once(' ')
				.expect(&format!("Line {line} can not be split at space!"));

			let springs = input_springs
				.chars()
				.map(parse_spring)
				.collect_vec();

			let damaged_springs = input_damaged_springs
				.split(',')
				.map(|t| t
					.parse::<u32>()
					.expect(&format!("Value {t} can not be parsed as number!")))
				.collect_vec();

			let mut result = 0;
			process(springs, damaged_springs, |_| result += 1);
			result
		})
		.sum()
}
