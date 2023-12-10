use itertools::Itertools;
use super::day10a::find_route;
use super::day10a::Map;
use super::day10a::Pipe;
use super::day10a::Position;

pub fn solution(input: &Vec<&str>) -> u32
{
	let map = Map::from(input);
	let route = find_route(&map);

	let first = route[1];
	let last = route[route.len() - 1];

	let diff = Position{ x: last.x - first.x, y: last.y - first.y };
	let start_pipe = match diff
	{
		Position{ x: 0, y: _ } => Pipe::Vertical,
		Position{ x: _, y: 0 } => Pipe::Horizontal,
		Position{ x: -1, y: -1 } => Pipe::BendNorthEast,
		Position{ x: 1, y: -1 } => Pipe::BendNorthWest,
		Position{ x: -1, y: 1 } => Pipe::BendSouthEast,
		Position{ x: 1, y: 1 } => Pipe::BendSouthWest,
		_ => panic!("Invalid start pipe!"),
	};

	(0..map.height())
		.map(|y| count_inside_row(
			&map,
			y as i32,
			route.iter().cloned(),
			start_pipe))
		.sum::<i32>() as u32
}

fn count_inside_row(map: &Map, y: i32, route: impl IntoIterator<Item = Position>, start_pipe: Pipe) -> i32
{
	let mut positions = route
		.into_iter()
		.filter(|p| p.y == y)
		.collect_vec();
	positions.sort_by_key(|p| p.x);

	let mut result = 0;
	let mut is_inside = false;

	for x in 0..map.width()
	{
		let position = Position{ x: x as i32, y };
		let pipe = match map.at(position).unwrap()
		{
			Pipe::StartingPosition => start_pipe,
			pipe => pipe,
		};

		let is_on_route = positions.contains(&position);
		match pipe
		{
			Pipe::Horizontal => {},

			Pipe::Vertical => if is_on_route { is_inside = !is_inside },
			Pipe::BendNorthEast => if is_on_route && !is_inside { is_inside = true },
			Pipe::BendNorthWest => if is_on_route { is_inside = !is_inside },
			Pipe::BendSouthWest => if is_on_route { is_inside = !is_inside },
			Pipe::BendSouthEast => if is_on_route && !is_inside { is_inside = true },

			Pipe::Ground => if is_inside { result += 1 },

			_ => panic!(),
		}
	}

	result
}
