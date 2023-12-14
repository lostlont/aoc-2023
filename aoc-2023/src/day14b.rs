use maplit::hashmap;
use crate::day14a::Direction;
use crate::day14a::Map;

pub fn solution(input: &Vec<&str>) -> u32
{
	let mut map = Map::from(input.as_slice());

	let mut index = 0;
	let mut traversed_maps = hashmap!{};
	let mut cycle: Option<(u32, u32)> = None;

	const STEP_COUNT: u32 = 1000000000;
	while index < STEP_COUNT
	{
		if let Some(cycle) = cycle
		{
			if (index % cycle.1) == (STEP_COUNT % cycle.1)
			{
				break;
			}
		}
		else
		{
			if let Some(&old_index) = traversed_maps.get(&map)
			{
				let cycle_start = old_index;
				let cycle_length = index - old_index;
				cycle = Some((cycle_start, cycle_length));
			}
			else
			{
				traversed_maps.insert(map.clone(), index);
			}
		}

		for direction in [Direction::North, Direction::West, Direction::South, Direction::East]
		{
			map.set_direction(direction);
			map.tick();
		}

		index += 1;
	}

	(0..map.height())
		.flat_map(|y| (0..map.width())
			.map(move |x| (x, y)))
		.filter(|&(x, y)| map.at(x as usize, y as usize) == Some('O'))
		.map(|(_, y)| map.height() as u32 - y as u32)
		.sum()
}
