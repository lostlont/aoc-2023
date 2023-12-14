use itertools::Itertools;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Object
{
	x: i32,
	y: i32,
}

#[derive(Debug, PartialEq)]
pub struct Map
{
	data: String,
	width: usize,
	height: usize,
	objects: Vec<Object>,
}

impl Object
{
	pub fn new(x: i32, y: i32) -> Self
	{
		Self
		{
			x,
			y,
		}
	}
}

impl Map
{
	pub fn new(data: &str, width: usize, height: usize, objects: Vec<Object>) -> Self
	{
		assert_eq!(data.len(), width * height);

		Self
		{
			data: data.to_string(),
			width,
			height,
			objects: Self::sorted_objects(objects),
		}
	}

	fn sorted_objects(mut objects: Vec<Object>) -> Vec<Object>
	{
		objects.sort_by_key(|o| o.y);
		objects.sort_by_key(|o| o.x);
		objects
	}

	pub fn at(&self, x: usize, y: usize) -> Option<char>
	{
		if x < self.width && y < self.height
		{
			self.data.chars().nth(y * self.width + x)
		}
		else
		{
			None
		}
	}

	pub fn get_width(&self) -> usize
	{
		self.width
	}

	pub fn get_height(&self) -> usize
	{
		self.height
	}

	pub fn get_objects(&self) -> impl Iterator<Item = &Object>
	{
		self.objects.iter()
	}

	pub fn tick(&mut self) -> bool
	{
		let mut is_moved = false;

		let movable_indices = self.objects
			.iter()
			.enumerate()
			.filter(|(_, object)| object.y > 0)
			.filter(|(_, object)| self.at(object.x as usize, (object.y as usize) - 1) == Some('.'))
			.filter(|(_, object)| !self.objects.iter().any(|other| (other.x == object.x) && (other.y == object.y - 1)))
			.map(|(index, _)| index)
			.collect_vec();

		for index in movable_indices
		{
			self.objects[index].y -= 1;
			is_moved = true;
		}

		is_moved
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
			.map(|c| match c
			{
				'O' => '.',
				c => c,
			})
			.join("");

		let objects = value
			.iter()
			.enumerate()
			.flat_map(|(y, line)| line
				.char_indices()
				.filter(|(_, c)| *c == 'O')
				.map(move |(x, _)| (x as i32, y as i32)))
			.map(|(x, y)| Object::new(x, y))
			.collect_vec();

		Self
		{
			data,
			width: value[0].len(),
			height: value.len(),
			objects: Self::sorted_objects(objects),
		}
	}
}

pub fn solution(input: &Vec<&str>) -> u32
{
	let mut map = Map::from(input.as_slice());

	while map.tick()
	{
	}

	map
		.get_objects()
		.map(|object| map.height as u32 - object.y as u32)
		.sum()
}
