use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pipe
{
	Vertical,
	Horizontal,
	BendNorthEast,
	BendNorthWest,
	BendSouthWest,
	BendSouthEast,
	Ground,
	StartingPosition,
}

impl From<char> for Pipe
{
	fn from(value: char) -> Self
	{
		match value
		{
			'|' => Pipe::Vertical,
			'-' => Pipe::Horizontal,
			'L' => Pipe::BendNorthEast,
			'J' => Pipe::BendNorthWest,
			'7' => Pipe::BendSouthWest,
			'F' => Pipe::BendSouthEast,
			'.' => Pipe::Ground,
			'S' => Pipe::StartingPosition,
			_ => panic!("Value '{value}' can not be parsed as a pipe!"),
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Position
{
	pub x: i32,
	pub y: i32,
}

const WEST: Position = Position{ x: -1, y: 0 };
const EAST: Position = Position{ x: 1, y: 0 };
const NORTH: Position = Position{ x: 0, y: -1 };
const SOUTH: Position = Position{ x: 0, y: 1 };

#[derive(Debug, PartialEq)]
pub struct Map
{
	values: Vec<Pipe>,
	width: usize,
	height: usize,
	starting_position: Position,
}

impl Map
{
	pub fn new(values: impl Iterator<Item = Pipe>, width: usize, height: usize, starting_position: Position) -> Self
	{
		let values = values.collect_vec();
		assert_eq!(values.len(), width * height);
		assert!(0 <= starting_position.x);
		assert!(starting_position.x < width as i32);
		assert!(0 <= starting_position.y);
		assert!(starting_position.y < height as i32);

		Self
		{
			values,
			width,
			height,
			starting_position,
		}
	}

	pub fn width(&self) -> usize
	{
		self.width
	}

	pub fn height(&self) -> usize
	{
		self.height
	}

	pub fn starting_position(&self) -> Position
	{
		self.starting_position
	}

	pub fn at(&self, position: Position) -> Option<Pipe>
	{
		match self.is_valid(position)
		{
			true => Some(self.values[(position.y as usize) * self.width + (position.x as usize)]),
			false => None,
		}
	}

	fn is_valid(&self, position: Position) -> bool
	{
		0 <= position.x &&
		position.x < self.width as i32 &&
		0 <= position.y &&
		position.y < self.height as i32
	}
}

impl From<&Vec<&str>> for Map
{
	fn from(value: &Vec<&str>) -> Self
	{
		let width = value[0].len();
		let height = value.len();

		let values = value
			.iter()
			.flat_map(|line| line.chars().map(|c| c.into()))
			.collect_vec();

		let starting_position = values
			.iter()
			.cloned()
			.enumerate()
			.find(|&(_, pipe)| pipe == Pipe::StartingPosition)
			.map(|(index, _)| Position{ x: (index % width) as i32, y: (index / height) as i32 })
			.expect("Map should have a starting position!");

		Self
		{
			values,
			width,
			height,
			starting_position,
		}
	}
}


pub fn is_pipe_connected_to(pipe: Pipe, to: Position) -> bool
{
	match pipe
	{
		Pipe::Vertical => [NORTH, SOUTH].contains(&to),
		Pipe::Horizontal => [WEST, EAST].contains(&to),
		Pipe::BendNorthEast => [NORTH, EAST].contains(&to),
		Pipe::BendNorthWest => [NORTH, WEST].contains(&to),
		Pipe::BendSouthWest => [SOUTH, WEST].contains(&to),
		Pipe::BendSouthEast => [SOUTH, EAST].contains(&to),
		Pipe::Ground => false,
		Pipe::StartingPosition => true,
	}
}

pub fn are_pipes_connected(map: &Map, position1: Position, position2: Position) -> bool
{
	match (map.at(position1), map.at(position2))
	{
		(Some(pipe1), Some(pipe2)) =>
			is_pipe_connected_to(pipe1, Position{ x: position2.x - position1.x, y: position2.y - position1.y }) &&
			is_pipe_connected_to(pipe2, Position{ x: position1.x - position2.x, y: position1.y - position2.y }),

		_ => false,
	}
}

pub fn out_direction(pipe: Pipe, in_direction: Position) -> Position
{
	match (pipe, in_direction)
	{
		(Pipe::Vertical, NORTH) => NORTH,
		(Pipe::Vertical, SOUTH) => SOUTH,
		(Pipe::Horizontal, WEST) => WEST,
		(Pipe::Horizontal, EAST) => EAST,
		(Pipe::BendNorthEast, SOUTH) => EAST,
		(Pipe::BendNorthEast, WEST) => NORTH,
		(Pipe::BendNorthWest, SOUTH) => WEST,
		(Pipe::BendNorthWest, EAST) => NORTH,
		(Pipe::BendSouthWest, NORTH) => WEST,
		(Pipe::BendSouthWest, EAST) => SOUTH,
		(Pipe::BendSouthEast, NORTH) => EAST,
		(Pipe::BendSouthEast, WEST) => SOUTH,
		_ => panic!("Invalid out direction for pipe {pipe:?} in direction {in_direction:?}!"),
	}
}

pub fn traverse(map: &Map, mut from: Position, mut direction: Position) -> Option<Vec<Position>>
{
	let mut to = Position{ x: from.x + direction.x, y: from.y + direction.y };
	if !are_pipes_connected(map, from, to)
	{
		return None;
	}

	let mut result = vec![];
	let mut run = true;
	while run
	{
		result.push(from);
		from = Position{ x: from.x + direction.x, y: from.y + direction.y };
		let from_pipe = map.at(from);
		if from_pipe == None
		{
			run = false;
		}
		if from_pipe == Some(Pipe::StartingPosition)
		{
			run = false;
		}
		else
		{
			direction = out_direction(from_pipe.unwrap(), direction);

			to = Position{ x: from.x + direction.x, y: from.y + direction.y };
			if !are_pipes_connected(map, from, to)
			{
				return None;
			}
		}
	}

	Some(result)
}

pub fn find_route(map: &Map) -> Vec<Position>
{
	[WEST, EAST, NORTH, SOUTH]
		.iter()
		.cloned()
		.find_map(|d| traverse(&map, map.starting_position(), d))
		.expect("Map should be traversable!")
}

pub fn solution(input: &Vec<&str>) -> u32
{
	let map = Map::from(input);
	let length = find_route(&map)
		.len();

	length as u32 / 2
}
