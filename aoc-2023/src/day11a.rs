use itertools::Itertools;

pub type Num = u64;
pub type Coord = i64;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Position
{
	x: Coord,
	y: Coord,
}

impl Position
{
	pub fn new(x: Coord, y: Coord) -> Self
	{
		Self
		{
			x,
			y,
		}
	}
}

impl From<(Coord, Coord)> for Position
{
	fn from(value: (Coord, Coord)) -> Self
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
			.map(move |(x, _)| Position::new(x as Coord, y as Coord)))
}

pub fn expand_map<'a>(map: impl Iterator<Item = Position> + Clone + 'a, by: Coord) -> impl Iterator<Item = Position> + 'a
{
	map
		.clone()
		.map(move |p| Position::new(
			p.x * by - count_x_before(map.clone(), p.x) * (by - 1),
			p.y * by - count_y_before(map.clone(), p.y) * (by - 1)))
}

fn count_x_before(map: impl Iterator<Item = Position>, x: Coord) -> Coord
{
	map
		.map(|p| p.x)
		.filter(|&px| px < x)
		.unique()
		.count() as Coord
}

fn count_y_before(map: impl Iterator<Item = Position>, y: Coord) -> Coord
{
	map
		.map(|p| p.y)
		.filter(|&py| py < y)
		.unique()
		.count() as Coord
}

pub fn distances(map: impl Iterator<Item = Position> + Clone) -> impl Iterator<Item = Coord>
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

pub fn solution(input: &Vec<&str>, expansion: Coord) -> Num
{
	let map = parse_map(input.iter().cloned())
		.collect_vec();

	let expanded_map = expand_map(map.iter().cloned(), expansion)
		.collect_vec();

	distances(expanded_map.iter().cloned())
		.sum::<Coord>() as Num
}
