use itertools::Itertools;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Position
{
	x: i32,
	y: i32,
}

impl Position
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

impl From<(i32, i32)> for Position
{
	fn from(value: (i32, i32)) -> Self
	{
		Self
		{
			x: value.0,
			y: value.1,
		}
	}
}

pub fn parse_map<'a>(input: impl IntoIterator<Item = &'a str> + 'a) -> impl Iterator<Item = Position> + 'a
{
	input
		.into_iter()
		.enumerate()
		.flat_map(|(y, line)| line
			.char_indices()
			.filter(|&(_, c)| c == '#')
			.map(move |(x, _)| Position::new(x as i32, y as i32)))
}

pub fn expand_map<'a>(map: impl Iterator<Item = Position> + Clone + 'a) -> impl Iterator<Item = Position> + 'a
{
	map
		.clone()
		.map(move |p| Position::new(
			p.x * 2 - count_x_before(map.clone(), p.x),
			p.y * 2 - count_y_before(map.clone(), p.y)))
}

fn count_x_before(map: impl Iterator<Item = Position>, x: i32) -> i32
{
	map
		.map(|p| p.x)
		.filter(|&px| px < x)
		.unique()
		.count() as i32
}

fn count_y_before(map: impl Iterator<Item = Position>, y: i32) -> i32
{
	map
		.map(|p| p.y)
		.filter(|&py| py < y)
		.unique()
		.count() as i32
}

pub fn distances(map: impl Iterator<Item = Position> + Clone) -> impl Iterator<Item = i32>
{
	pairs(map)
		.map(|(a, b)| (a.x - b.x).abs() + (a.y - b.y).abs())
}

fn pairs(map: impl Iterator<Item = Position> + Clone) -> impl Iterator<Item = (Position, Position)>
{
	map
		.into_iter()
		.tuple_combinations::<(_, _)>()
}

pub fn solution(input: &Vec<&str>) -> u32
{
	let map = parse_map(input.iter().cloned())
		.collect_vec();

	let expanded_map = expand_map(map.iter().cloned())
		.collect_vec();

	distances(expanded_map.iter().cloned())
		.sum::<i32>() as u32
}
