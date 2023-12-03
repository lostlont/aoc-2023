use super::day03a::sample_numbers_around;

pub fn get_gear_ratios_around_gears(input: &Vec<&str>) -> Vec<u32>
{
	let mut result = vec![];

	for row_index in 0..input.len()
	{
		let row = input[row_index];
		for column_index in 0..row.len()
		{
			let ch = row.chars().nth(column_index).unwrap();
			if ch == '*'
			{
				let numbers_around = sample_numbers_around(input, row_index, column_index);
				if numbers_around.len() == 2
				{
					let gear_ratio = numbers_around[0].number * numbers_around[1].number;
					result.push(gear_ratio);
				}
			}
		}
	}

	result
		.into_iter()
		.collect()
}

pub fn solution(input: &Vec<&str>) -> u32
{
	get_gear_ratios_around_gears(input)
		.iter()
		.sum()
}

