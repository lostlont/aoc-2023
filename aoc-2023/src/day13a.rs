use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub struct Image
{
	data: String,
	width: usize,
	height: usize,
}

impl Image
{
	pub fn new(data: &str, width: usize, height: usize) -> Self
	{
		assert_eq!(data.len(), width * height);

		Self
		{
			data: data.to_string(),
			width,
			height,
		}
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
}

impl From<&[&str]> for Image
{
	fn from(value: &[&str]) -> Self
	{
		assert!(value.iter().skip(1).all(|line| line.len() == value[0].len()));

		Self
		{
			data: value.join(""),
			width: value[0].len(),
			height: value.len(),
		}
	}
}

pub fn is_mirrored_by_horizontal_line_at(image: &Image, at: usize) -> bool
{
	let at = at as i32;
	let repeated_height = i32::min(at, image.height as i32 - at);

	if repeated_height <= 0
	{
		return false;
	}

	for y in 0..repeated_height
	{
		for x in 0..image.width
		{
			let top_y = (at - y - 1) as usize;
			let bottom_y = (at + y) as usize;
			if image.at(x, top_y) != image.at(x, bottom_y)
			{
				return false;
			}
		}
	}

	return true;
}

pub fn is_mirrored_by_vertical_line_at(image: &Image, at: usize) -> bool
{
	let at = at as i32;
	let repeated_width = i32::min(at, image.width as i32 - at);

	if repeated_width <= 0
	{
		return false;
	}

	for y in 0..image.height
	{
		for x in 0..repeated_width
		{
			let left_x = (at - x - 1) as usize;
			let right_x = (at + x) as usize;
			if image.at(left_x, y) != image.at(right_x, y)
			{
				return false;
			}
		}
	}

	return true;
}

pub fn score(image: &Image) -> u32
{
	for y in 1..=image.height-1
	{
		if is_mirrored_by_horizontal_line_at(image, y)
		{
			return 100 * y as u32;
		}
	}

	for x in 1..=image.width-1
	{
		if is_mirrored_by_vertical_line_at(image, x)
		{
			return x as u32;
		}
	}

	panic!("Invalid image!");
}

pub fn solution(input: &Vec<&str>) -> u32
{
	let split_indices = input
		.iter()
		.enumerate()
		.filter(|(_, line)| line.is_empty())
		.map(|(index, _)| index as i32);

	let split_indices = std::iter
		::once(-1)
		.chain(split_indices)
		.chain(
			std::iter::once(input.len() as i32));

	let split_index_pairs = split_indices.tuple_windows();

	split_index_pairs
		.map(|(a, b)| Image::from(&input[((a+1) as usize)..(b as usize)]))
		.map(|image| score(&image))
		.sum()
}
