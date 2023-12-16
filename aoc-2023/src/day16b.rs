use std::io::BufRead;
use itertools::Itertools;
use crate::day16a::{Field, Flow};

pub fn solution(input: impl BufRead) -> u32
{
	let input = input
		.lines()
		.map(|line| line.expect("Couldn't read input line!"))
		.collect_vec();
	let input = input
		.iter()
		.map(|line| &line[..])
		.collect_vec();

	let field = Field::parse(&input);
	let cells = field.cells();
	let width = cells.width();
	let height = cells.height();
	[
		(0..width)
			.map(|x| (x, 0, Flow::FROM_TOP))
			.collect_vec(),
		(0..width)
			.map(|x| (x, height - 1, Flow::FROM_BOTTOM))
			.collect_vec(),
		(0..height)
			.map(|y| (0, y, Flow::FROM_LEFT))
			.collect_vec(),
		(0..height)
			.map(|y| (width - 1, y, Flow::FROM_RIGHT))
			.collect_vec(),
	]
		.iter()
		.flatten()
		.map(|&(x, y, flow)| score(field.clone(), x, y, flow))
		.max()
		.unwrap()
}

fn score(mut field: Field, start_x: usize, start_y: usize, start_flow: Flow) -> u32
{
	field.set_flow_at(start_x, start_y, start_flow);

	let mut changed_cells = vec![(start_x, start_y)];
	while !changed_cells.is_empty()
	{
		changed_cells = field.flow(&changed_cells);
	}

	let cells = field.cells();
	let height = cells.height();
	let width = cells.width();
	(0..height)
		.cartesian_product(0..width)
		.map(|(x, y)| field.flow_map().at(x, y).unwrap())
		.filter(|&&flow| flow != Flow::EMPTY)
		.count() as u32
}
