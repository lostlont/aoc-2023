use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction
{
	North,
	West,
	South,
	East,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Map
{
	data: Vec<char>,
	width: usize,
	height: usize,
	direction: Direction,
}

impl Map
{
	pub fn new(
		data: &str,
		width: usize,
		height: usize,
		direction: Direction) -> Self
	{
		assert_eq!(data.len(), width * height);

		Self
		{
			data: data.chars().collect_vec(),
			width,
			height,
			direction,
		}
	}

	pub fn at(&self, x: usize, y: usize) -> Option<char>
	{
		if x < self.width && y < self.height
		{
			Some(self.data[y * self.width + x])
		}
		else
		{
			None
		}
	}

	pub fn set_at(&mut self, x: usize, y: usize, value: char)
	{
		if x < self.width && y < self.height
		{
			self.data[y * self.width + x] = value;
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

	pub fn direction(&self) -> Direction
	{
		self.direction
	}

	pub fn set_direction(&mut self, direction: Direction)
	{
		self.direction = direction;
	}

	pub fn tick(&mut self)
	{
		match self.direction
		{
			Direction::North => self.move_north(),
			Direction::West => self.move_west(),
			Direction::South => self.move_south(),
			Direction::East => self.move_east(),
		}
	}

	fn move_north(&mut self)
	{
		for x in 0..self.width
		{
			let mut free_y = 0;
			for y in 0..self.height
			{
				match self.at(x, y).unwrap()
				{
					'.' => {},
					'O' =>
					{
						self.move_object(x, y, x, free_y);
						free_y += 1;
					}
					'#' => free_y = y + 1,
					_ => panic!(),
				}
			}
		}
	}

	fn move_south(&mut self)
	{
		for x in 0..self.width
		{
			let mut free_y = self.height - 1;
			for y in (0..self.height).rev()
			{
				match self.at(x, y).unwrap()
				{
					'.' => {},
					'O' =>
					{
						self.move_object(x, y, x, free_y);
						free_y = free_y.max(1) - 1;
					}
					'#' => free_y = y.max(1) - 1,
					_ => panic!(),
				}
			}
		}
	}

	fn move_west(&mut self)
	{
		for y in 0..self.height
		{
			let mut free_x = 0;
			for x in 0..self.width
			{
				match self.at(x, y).unwrap()
				{
					'.' => {},
					'O' =>
					{
						self.move_object(x, y, free_x, y);
						free_x += 1;
					}
					'#' => free_x = x + 1,
					_ => panic!(),
				}
			}
		}
	}

	fn move_east(&mut self)
	{
		for y in 0..self.height
		{
			let mut free_x = self.width - 1;
			for x in (0..self.width).rev()
			{
				match self.at(x, y).unwrap()
				{
					'.' => {},
					'O' =>
					{
						self.move_object(x, y, free_x, y);
						free_x = free_x.max(1) - 1;
					}
					'#' => free_x = x.max(1) - 1,
					_ => panic!(),
				}
			}
		}
	}

	fn move_object(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize)
	{
		if (from_x != to_x) || (from_y != to_y)
		{
			self.set_at(from_x, from_y, '.');
			self.set_at(to_x, to_y, 'O');
		}
	}
}

impl From<&[&str]> for Map
{
	fn from(value: &[&str]) -> Self
	{
		assert!(value.iter().skip(1).all(|line| line.len() == value[0].len()));

		let data = value
			.iter()
			.flat_map(|line| line.chars())
			.collect_vec();

		Self
		{
			data,
			width: value[0].len(),
			height: value.len(),
			direction: Direction::North,
		}
	}
}

pub fn solution(input: &Vec<&str>) -> u32
{
	let mut map = Map::from(input.as_slice());

	map.tick();

	(0..map.height())
		.flat_map(|y| (0..map.width())
			.map(move |x| (x, y)))
		.filter(|&(x, y)| map.at(x as usize, y as usize) == Some('O'))
		.map(|(_, y)| map.height() as u32 - y as u32)
		.sum()
}
