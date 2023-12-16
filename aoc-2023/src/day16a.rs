use std::io::BufRead;
use std::iter;
use std::ops::BitOr;
use itertools::Itertools;
use maplit::{hashmap, hashset};
use crate::common::map::Map;

#[derive(Clone, Debug, PartialEq)]
pub enum Mirror
{
	LeftToUp,
	LeftToDown,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Splitter
{
	Horizontal,
	Vertical,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Cell
{
	Empty,
	Mirror(Mirror),
	Splitter(Splitter),
}

impl From<char> for Cell
{
	fn from(value: char) -> Cell
	{
		match value
		{
			'.' => Cell::Empty,
			'/' => Cell::Mirror(Mirror::LeftToUp),
			'\\' => Cell::Mirror(Mirror::LeftToDown),
			'-' => Cell::Splitter(Splitter::Horizontal),
			'|' => Cell::Splitter(Splitter::Vertical),
			_ => panic!("Character '{value}' can not be parsed as Cell!"),
		}
	}
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Flow
{
	from_left: bool,
	from_top: bool,
	from_right: bool,
	from_bottom: bool,
}

impl Flow
{
	pub fn new(from_left: bool, from_top: bool, from_right: bool, from_bottom: bool) -> Self
	{
		Self
		{
			from_left,
			from_top,
			from_right,
			from_bottom,
		}
	}

	pub const EMPTY: Self = Self
	{
		from_left: false,
		from_top: false,
		from_right: false,
		from_bottom: false,
	};

	pub const FROM_LEFT: Self = Self
	{
		from_left: true,
		from_top: false,
		from_right: false,
		from_bottom: false,
	};

	pub const FROM_TOP: Self = Self
	{
		from_left: false,
		from_top: true,
		from_right: false,
		from_bottom: false,
	};

	pub const FROM_RIGHT: Self = Self
	{
		from_left: false,
		from_top: false,
		from_right: true,
		from_bottom: false,
	};

	pub const FROM_BOTTOM: Self = Self
	{
		from_left: false,
		from_top: false,
		from_right: false,
		from_bottom: true,
	};
}

impl BitOr for Flow
{
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self::Output
	{
		Self
		{
			from_left: self.from_left | rhs.from_left,
			from_top: self.from_top | rhs.from_top,
			from_right: self.from_right | rhs.from_right,
			from_bottom: self.from_bottom | rhs.from_bottom,
		}
	}
}

#[derive(Debug, PartialEq)]
pub struct Field
{
	cells: Map<Cell>,
	flow_map: Map<Flow>,
}

impl Field
{
	pub fn new(cells: Map<Cell>, flow_map: Map<Flow>) -> Self
	{
		Self
		{
			cells,
			flow_map,
		}
	}

	pub fn parse(input: &[&str]) -> Self
	{
		let cells = Map::from(input);
		let flow_map = Self::empty_flow_of(&cells);
		Self
		{
			cells,
			flow_map
		}
	}

	pub fn empty_flow_of(cells: &Map<Cell>) -> Map<Flow>
	{
		let empty_flow = iter
			::repeat(Flow::default())
			.take(cells.width() * cells.height());

		Map::<Flow>::new(empty_flow, cells.width(), cells.height())
	}

	pub fn cells(&self) -> &Map<Cell>
	{
		&self.cells
	}

	pub fn flow_map(&self) -> &Map<Flow>
	{
		&self.flow_map
	}

	pub fn set_flow_at(&mut self, x: usize, y: usize, value: Flow)
	{
		self.flow_map.set_at(x, y, value);
	}

	pub fn flow(&mut self, changed_cells: &[(usize, usize)]) -> Vec<(usize, usize)>
	{
		let mut new_changed_cells = hashset!{};

		for &(in_x, in_y) in changed_cells
		{
			let x = in_x as i32;
			let y = in_y as i32;
			let possible_out_flows = match self.cells.at(in_x, in_y).unwrap()
			{
				Cell::Empty => hashmap!
				{
					Flow::FROM_LEFT => vec![(x + 1, y, Flow::FROM_LEFT)],
					Flow::FROM_TOP => vec![(x, y + 1, Flow::FROM_TOP)],
					Flow::FROM_RIGHT => vec![(x - 1, y, Flow::FROM_RIGHT)],
					Flow::FROM_BOTTOM => vec![(x, y - 1, Flow::FROM_BOTTOM)],
				},
				Cell::Mirror(Mirror::LeftToUp) => hashmap!
				{
					Flow::FROM_LEFT => vec![(x, y - 1, Flow::FROM_BOTTOM)],
					Flow::FROM_TOP => vec![(x - 1, y, Flow::FROM_RIGHT)],
					Flow::FROM_RIGHT => vec![(x, y + 1, Flow::FROM_TOP)],
					Flow::FROM_BOTTOM => vec![(x + 1, y, Flow::FROM_LEFT)],
				},
				Cell::Mirror(Mirror::LeftToDown) => hashmap!
				{
					Flow::FROM_LEFT => vec![(x, y + 1, Flow::FROM_TOP)],
					Flow::FROM_TOP => vec![(x + 1, y, Flow::FROM_LEFT)],
					Flow::FROM_RIGHT => vec![(x, y - 1, Flow::FROM_BOTTOM)],
					Flow::FROM_BOTTOM => vec![(x - 1, y, Flow::FROM_RIGHT)],
				},
				Cell::Splitter(Splitter::Horizontal) => hashmap!
				{
					Flow::FROM_LEFT => vec![(x + 1, y, Flow::FROM_LEFT)],
					Flow::FROM_TOP => vec!
					[
						(x - 1, y, Flow::FROM_RIGHT),
						(x + 1, y, Flow::FROM_LEFT),
					],
					Flow::FROM_RIGHT => vec![(x - 1, y, Flow::FROM_RIGHT)],
					Flow::FROM_BOTTOM => vec!
					[
						(x - 1, y, Flow::FROM_RIGHT),
						(x + 1, y, Flow::FROM_LEFT),
					],
				},
				Cell::Splitter(Splitter::Vertical) => hashmap!
				{
					Flow::FROM_LEFT => vec!
					[
						(x, y - 1, Flow::FROM_BOTTOM),
						(x, y + 1, Flow::FROM_TOP),
					],
					Flow::FROM_TOP => vec![(x, y + 1, Flow::FROM_TOP)],
					Flow::FROM_RIGHT => vec!
					[
						(x, y - 1, Flow::FROM_BOTTOM),
						(x, y + 1, Flow::FROM_TOP),
					],
					Flow::FROM_BOTTOM => vec![(x, y - 1, Flow::FROM_BOTTOM)],
				},
			};

			for (in_flow, out_flows) in possible_out_flows
			{
				let current_in_flow = self.flow_map
					.at(in_x, in_y)
					.unwrap()
					.clone();
				if (current_in_flow | in_flow) == current_in_flow
				{
					for (out_x, out_y, out_flow) in out_flows
					{
						if (0 <= out_x) && (0 <= out_y)
						{
							let out_x = out_x as usize;
							let out_y = out_y as usize;
							if let Some(flow) = self.flow_map.at(out_x, out_y)
							{
								let new_flow = out_flow | *flow;
								if new_flow != *flow
								{
									self.flow_map.set_at(out_x, out_y, new_flow);
									new_changed_cells.insert((out_x, out_y));
								}
							}
						}
					}
				}
			}
		}

		new_changed_cells
			.iter()
			.cloned()
			.collect_vec()
	}
}

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

	let mut field = Field::parse(&input);
	field.set_flow_at(0, 0, Flow::FROM_LEFT);

	let mut changed_cells = vec![(0, 0)];
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
