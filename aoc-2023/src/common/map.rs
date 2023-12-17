use std::io::BufRead;
use itertools::Itertools;

#[derive(Clone, Debug, PartialEq)]
pub struct Map<TValue>
{
	values: Vec<TValue>,
	width: usize,
	height: usize,
}

impl<TValue> Map<TValue>
{
	pub fn new(values: impl IntoIterator<Item = TValue>, width: usize, height: usize) -> Self
	{
		let values = values
			.into_iter()
			.collect_vec();

		assert_eq!(values.len(), width * height);

		Self
		{
			values,
			width,
			height,
		}
	}

	pub fn from_str_parser(value: &[&str], parser: impl Fn(char) -> TValue) -> Self
	{
		assert!(value.iter().skip(1).all(|line| line.len() == value[0].len()));

		let values = value
			.iter()
			.flat_map(|line| line.chars())
			.map(parser)
			.collect_vec();

		Self
		{
			values,
			width: value[0].len(),
			height: value.len(),
		}
	}

	pub fn from_read(input: impl BufRead) -> Self
	where
		TValue: From<char>,
	{
		let input = input
			.lines()
			.map(|line| line.expect("Couldn't read input line!"))
			.collect_vec();
		let values = input
			.iter()
			.map(|line| &line[..])
			.collect_vec();
		let values = values.as_slice();
		Self::from(values)
	}

	pub fn from_read_parser(input: impl BufRead, parser: impl Fn(char) -> TValue) -> Self
	where
		TValue: From<char>,
	{
		let input = input
			.lines()
			.map(|line| line.expect("Couldn't read input line!"))
			.collect_vec();
		let values = input
			.iter()
			.map(|line| &line[..])
			.collect_vec();
		let values = values.as_slice();
		Self::from_str_parser(values, parser)
	}

	pub fn at(&self, x: usize, y: usize) -> Option<&TValue>
	{
		if x < self.width && y < self.height
		{
			Some(&self.values[y * self.width + x])
		}
		else
		{
			None
		}
	}

	pub fn set_at(&mut self, x: usize, y: usize, value: TValue)
	{
		if x < self.width && y < self.height
		{
			self.values[y * self.width + x] = value;
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
}

impl<TValue> From<&[&str]> for Map<TValue>
where
	TValue: From<char>,
{
	fn from(value: &[&str]) -> Self
	{
		assert!(value.iter().skip(1).all(|line| line.len() == value[0].len()));

		let values = value
			.iter()
			.flat_map(|line| line.chars())
			.map(TValue::from)
			.collect_vec();

		Self
		{
			values,
			width: value[0].len(),
			height: value.len(),
		}
	}
}
