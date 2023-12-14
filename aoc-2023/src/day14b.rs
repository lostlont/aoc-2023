use maplit::hashset;
use crate::day14a::Direction;
use crate::day14a::Map;

pub fn solution(input: &Vec<&str>) -> u32
{
	let mut map = Map::from(input.as_slice());

	let mut traversed_maps = hashset!{};

	for _ in 0..1000000000
	{
		for direction in [Direction::North, Direction::West, Direction::South, Direction::East]
		{
			map.set_direction(direction);
			map.tick();
		}

		if traversed_maps.contains(&map)
		{
			break;
		}

		traversed_maps.insert(map.clone());
	}

	(0..map.height())
		.flat_map(|y| (0..map.width())
			.map(move |x| (x, y)))
		.filter(|&(x, y)| map.at(x as usize, y as usize) == Some('O'))
		.map(|(_, y)| map.height() as u32 - y as u32)
		.sum()
}

fn debug_map(map: &Map)
{
	for y in 0..map.height()
	{
		for x in 0..map.width()
		{
			print!("{}", map.at(x, y).unwrap());
		}
		println!();
	}
	println!();
}
