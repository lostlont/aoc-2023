use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::io::BufRead;
use std::iter;
use itertools::Itertools;
use maplit::hashset;
use crate::common::map::Map;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction
{
	Left,
	Up,
	Right,
	Down,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Step
{
	x: usize,
	y: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Route
{
	x: usize,
	y: usize,
	cost: u32,
	steps: Vec<Step>,
}

impl Step
{
	pub fn new(x: usize, y: usize) -> Self
	{
		Self{ x, y }
	}
}

impl Route
{
	pub fn new(
		x: usize,
		y: usize,
		cost: u32,
		steps: impl IntoIterator<Item = Step>) -> Self
	{
		let steps = steps.into_iter().collect_vec();
		Self
		{
			x,
			y,
			cost,
			steps,
		}
	}

	pub fn get_last_directions(&self) -> impl Iterator<Item = Direction> + '_
	{
		self.steps
			.iter()
			.rev()
			.tuple_windows()
			.filter_map(|(to, from)| get_direction(from, to))
	}

	pub fn get_next_directions(&self) -> HashSet<Direction>
	{
		let mut result = hashset!{ Direction::Left, Direction::Up, Direction::Right, Direction::Down };

		if let Some(last_direction) = self.get_last_directions().next()
		{
			if self
				.get_last_directions()
				.take(3)
				.filter(|&d| d == last_direction)
				.count() == 3
			{
				result.remove(&last_direction);
			}

			let opposite_direction = match last_direction
			{
				Direction::Left => Direction::Right,
				Direction::Up => Direction::Down,
				Direction::Right => Direction::Left,
				Direction::Down => Direction::Up,
			};
			result.remove(&opposite_direction);
		}

		result
	}
}

impl Ord for Route
{
	fn cmp(&self, other: &Self) -> Ordering
	{
		match self.cost.cmp(&other.cost)
		{
			Ordering::Less => return Ordering::Greater,
			Ordering::Greater => return Ordering::Less,
			Ordering::Equal => {},
		};

		let distance = self.x + self.y;
		let other_distance = other.x + other.y;
		match distance.cmp(&other_distance)
		{
			Ordering::Equal => {},
			ord => return ord,
		}

		let length = self.steps.len();
		let other_length = other.steps.len();
		match length.cmp(&other_length)
		{
			Ordering::Less => return Ordering::Greater,
			Ordering::Greater => return Ordering::Less,
			Ordering::Equal => {},
		}

		Ordering::Equal
	}
}

impl PartialOrd for Route
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

pub fn get_direction(from_step: &Step, to_step: &Step) -> Option<Direction>
{
	let from_x = from_step.x as i32;
	let from_y = from_step.y as i32;
	let to_x = to_step.x as i32;
	let to_y = to_step.y as i32;
	match (to_x - from_x, to_y - from_y)
	{
		(1, 0) => Some(Direction::Right),
		(-1, 0) => Some(Direction::Left),
		(0, 1) => Some(Direction::Down),
		(0, -1) => Some(Direction::Up),
		_ => None,
	}
}

pub fn solution(input: impl BufRead) -> u32
{
	let map = Map::<u32>::from_read_parser(input, parse_value);
	//let mut heap = BinaryHeap::from([Route::new(0, 0, 0, [])]);
	let mut heap = BinaryHeap::from([Route::new(0, 0, 0, [Step::new(0, 0)])]);
	let mut best_costs = Map::<Option<u32>>::new(
		iter::repeat(None)
			.take(map.width() * map.height()),
		map.width(),
		map.height());

	//let mut run = 0;
	while let Some(route) = heap.pop()
	{
		/*
		if let Some(best) = best
		{
			if route.cost > best
			{
				//let r = heap.len(); println!("::PL dropping {route:?} {r}");
				continue;
			}
		}
		*/
		//run +=1;
		//println!("::PL running {route:#?}");
		//println!("::PL {heap:#?}");
		//if run > 4
		//{
		//	break;
		//}
		//let debug = route.steps == &[Step::new(0, 0), Step::new(1, 0), Step::new(2, 0), Step::new(2, 1)]; // ::PL
		let debug = route
			.get_last_directions()
			.collect_vec()
			.into_iter()
			.rev()
			.collect_vec() == &[
				Direction::Right,
				Direction::Right,
				Direction::Down,
				Direction::Right,
				Direction::Right,
			]; // ::PL
		if debug { let q = route.get_next_directions().len(); println!("::PL {q} count options"); }
		for direction in route.get_next_directions()
		{
			if debug { println!("::PL considering {direction:?}"); }
			let last = route.steps.last().unwrap();
			if let Some((x, y)) = match direction
			{
				//Direction::Left => match route.x
				Direction::Left => match last.x
				{
					//1.. => Some((route.x - 1, route.y)),
					1.. => Some((last.x - 1, last.y)),
					_ => None,
				},
				//Direction::Up => match route.y
				Direction::Up => match last.y
				{
					//1.. => Some((route.x, route.y - 1)),
					1.. => Some((last.x, last.y - 1)),
					_ => None,
				},
				//Direction::Right => Some((route.x + 1, route.y)),
				Direction::Right => Some((last.x + 1, last.y)),
				//Direction::Down => Some((route.x, route.y + 1)),
				Direction::Down => Some((last.x, last.y + 1)),
			}
			{
				if let Some(cost) = map.at(x, y)
				{
					if debug { println!("::PL map at {x},{y} = {cost}"); }
					let is_new_step = route.steps
						.iter()
						.find(|step| (step.x == x) && (step.y == y))
						.is_none();

					if is_new_step
					{
						let new_cost = route.cost + cost;

						if let Some(&Some(best_cost)) = best_costs.at(x, y)
						{
							if best_cost < new_cost
							{
								continue;
							}
						}
						best_costs.set_at(x, y, Some(new_cost));

						if debug { let c = route.cost; println!("::PL old cost {c}, new cost {new_cost}"); }
						if (x == map.width() - 1) && (y == map.height() - 1)
						{
							let d = route
								.get_last_directions()
								.collect_vec()
								.iter()
								.rev()
								.map(|d| match d { Direction::Left => '<', Direction::Up => '^', Direction::Right => '>', Direction::Down => 'v' })
								.join("");
							//println!("::PL {d}");
							println!("::PL new best {new_cost}");

							//best = match best
							//{
							//	Some(best) => Some(best.min(new_cost)),
							//	None => Some(new_cost),
							//};
						}
						else
						{
							let new_steps = route.steps
								.iter()
								.copied()
								//.chain(iter::once(Step::new(route.x, route.y)))
								.chain(iter::once(Step::new(x, y)))
								.collect_vec();

							let new_route = Route::new(x, y, new_cost, new_steps);

							if debug { let c = heap.len(); println!("::PL pushing {new_route:?} ({c}->)"); }
							heap.push(new_route);
						}
					}
				}
			}
		}
	}

	best_costs
		.at(map.width() - 1, map.height() - 1)
		.unwrap()
		.unwrap()
}

fn parse_value(char: char) -> u32
{
	(char as u32) - ('0' as u32)
}
