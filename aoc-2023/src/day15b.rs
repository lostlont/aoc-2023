use std::collections::{HashMap, VecDeque};
use std::io::BufRead;
use maplit::hashmap;
use crate::day15a::hash;

#[derive(Debug, PartialEq)]
pub enum Operation
{
	Remove
	{
		label: String,
	},
	Add
	{
		label: String,
		focal_length: u8,
	},
}

#[derive(Debug, PartialEq)]
pub struct Lens
{
	label: String,
	focal_length: u8,
}

#[derive(Debug, PartialEq)]
pub struct Boxes
{
	contents: HashMap<usize, VecDeque<Lens>>,
}

impl From<&str> for Operation
{
	fn from(value: &str) -> Self
	{
		if let Some((label, focal_length)) = value.split_once('=')
		{
			let label = label.to_owned();

			let focal_length = focal_length
				.parse::<u8>()
				.expect(&format!("Couldn't parse {focal_length} as number!"));

			Operation::Add{ label, focal_length }
		}
		else if let Some((label, _)) = value.split_once('-')
		{
			let label = label.to_owned();

			Operation::Remove{ label }
		}
		else
		{
			panic!("Couldn't split value {value} by '=' or '-'!");
		}
	}
}

impl Lens
{
	pub fn new(label: &str, focal_length: u8) -> Self
	{
		Self
		{
			label: label.to_string(),
			focal_length,
		}
	}
}

impl Boxes
{
	pub fn new(contents: HashMap<usize, VecDeque<Lens>>) -> Self
	{
		Self
		{
			contents,
		}
	}

	pub fn new_empty() -> Self
	{
		Self
		{
			contents: hashmap!{},
		}
	}

	pub fn apply(&mut self, operation: Operation)
	{
		match operation
		{
			Operation::Remove { label } =>
			{
				let hash = hash(&label) as usize;

				if let Some(contents) = self.contents.get_mut(&hash)
				{
					if let Some((index, _)) = contents
						.iter()
						.enumerate()
						.find(|(_, lens)| lens.label == label)
					{
						contents.remove(index);
					}
				}
			},

			Operation::Add { label, focal_length } =>
			{
				let hash = hash(&label) as usize;

				if !self.contents.contains_key(&hash)
				{
					self.contents.insert(hash, VecDeque::new());
				}

				let contents = self.contents
					.get_mut(&hash)
					.unwrap();

				let lens = Lens::new(&label, focal_length);
				if let Some((old_index, _)) = contents
					.iter()
					.enumerate()
					.find(|(_, old_lens)| old_lens.label == lens.label)
				{
					contents.remove(old_index);
					contents.insert(old_index, lens);
				}
				else
				{
					contents.push_back(lens);
				}
			},
		}
	}
}

pub fn solution(input: impl BufRead) -> u32
{
	let mut boxes = Boxes::new_empty();

	input
		.lines()
		.next()
		.expect("Input should contain a line!")
		.iter()
		.flat_map(|line| line.split(','))
		.map(Operation::from)
		.for_each(|op| boxes.apply(op));

	boxes.contents
		.iter()
		.flat_map(|(box_index, contents)| contents
			.iter()
			.enumerate()
			.map(|(slot_index, lens)| score(*box_index, slot_index, lens)))
		.sum()
}

fn score(box_index: usize, slot_index: usize, lens: &Lens) -> u32
{
	let box_number = box_index as u32 + 1;
	let slot_number = slot_index as u32 + 1;
	box_number * slot_number * lens.focal_length as u32
}
