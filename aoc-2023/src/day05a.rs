pub type Num = i64;

#[derive(Debug, PartialEq)]
pub struct Mapping
{
	pub source_start: Num,
	pub destination_start: Num,
	pub length: Num,
}

impl Mapping
{
	pub fn map_values<'a, TValues>(&'a self, values: TValues) -> impl Iterator<Item = Option<Num>> + 'a
	where
		TValues: IntoIterator<Item = &'a Num> + 'a,
	{
		values
			.into_iter()
			.cloned()
			.map(|v| self.map_value(v))
	}

	fn map_value(&self, value: Num) -> Option<Num>
	{
		if (self.source_start..self.source_start+self.length).contains(&value)
		{
			let result = value - self.source_start + self.destination_start;
			Some(result)
		}
		else
		{
			None
		}
	}
}

pub fn parse_seeds(input: &str) -> impl Iterator<Item = Num> + '_
{
	const PREFIX : &str = "seeds:";
	let input = input.strip_prefix(PREFIX)
		.expect(&format!("Line should start with \"{PREFIX}\"!"));

	input
		.split_whitespace()
		.map(expect_i64)
}

pub fn parse_mapping(input: &str) -> Mapping
{
	let numbers = input
		.split_whitespace()
		.map(expect_i64)
		.collect::<Vec<_>>();

	match numbers.as_slice()
	{
		&[destination_start, source_start, length] => Mapping
		{
			source_start,
			destination_start,
			length,
		},
		_ => panic!("Line \"{input}\" can not be parsed as 3 numbers of ranges!"),
	}
}

pub fn expect_i64(input: &str) -> Num
{
	input
		.parse::<Num>()
		.expect(&format!("Value \"{input}\" can not be parsed as i64!"))
}

pub fn parse_values<'a>(input: &'a Vec<&str>) -> Vec<Num>
{
	let mut numbers = parse_seeds(input[0])
		.collect::<Vec<Num>>();
	let mut category_result = (0..numbers.len())
		.map(|_| None)
		.collect::<Vec<_>>();

	for line in input.iter().skip(1)
	{
		if line.is_empty()
		{
		}
		else if line.ends_with("map:")
		{
			numbers = category_result
				.iter()
				.zip(numbers)
				.map(|(new, old)| new.unwrap_or(old))
				.collect::<Vec<_>>();
		}
		else
		{
			let mapping = parse_mapping(&line);

			category_result = mapping
				.map_values(&numbers)
				.zip(category_result)
				.map(|(new, old)| new.or(old))
				.collect::<Vec<_>>();
		}
	}

	category_result
		.iter()
		.zip(numbers)
		.map(|(new, old)| new.unwrap_or(old))
		.collect::<_>()
}

pub fn solution(input: &Vec<&str>) -> Num
{
	parse_values(input)
		.iter()
		.cloned()
		.min()
		.expect("No minimum exists!")
}
