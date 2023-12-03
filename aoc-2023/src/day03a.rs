use maplit::hashset;

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FoundNumber
{
	pub row: usize,
	pub column: usize,
	pub number: u32,
}

pub fn find_first_digit_around(input: &str, index: usize) -> Option<usize>
{
	input[..index+1]
		.char_indices()
		.rev()
		.take_while(|(_i, c)| ('0'..='9').contains(c))
		.map(|(i, _c)| i)
		.last()
}

pub fn find_last_digit_around(input: &str, index: usize) -> Option<usize>
{
	input
		.char_indices()
		.skip(index)
		.take_while(|(_i, c)| ('0'..='9').contains(c))
		.map(|(i, _c)| i)
		.last()
}

pub fn sample_number_at(input: &str, row_index: usize, column_index: usize) -> Option<FoundNumber>
{
	let first_index = find_first_digit_around(input, column_index);
	let last_index = find_last_digit_around(input, column_index);
	if let Some(first_index) = first_index
	{
		if let Some(last_index) = last_index
		{
			let number = input[first_index..last_index+1]
				.parse::<u32>()
				.unwrap();

			return Some(FoundNumber
			{
				row: row_index,
				column: first_index,
				number,
			});
		}
	}

	None
}

pub fn sample_numbers_around(input: &Vec<&str>, row_index: usize, column_index: usize) -> Vec<FoundNumber>
{
	let mut result = hashset!{};

	let row_from = (row_index as i32 - 1).max(0) as usize;
	let row_to = (row_index + 2).min(input.len());
	for row_index in row_from..row_to
	{
		let row = input[row_index];

		let column_from = (column_index as i32 - 1).max(0) as usize;
		let column_to = (column_index + 2).min(row.len());

		for column_index in column_from..column_to
		{
			if let Some(found_number) = sample_number_at(row, row_index, column_index)
			{
				result.insert(found_number);
			}
		}
	}

	result
		.into_iter()
		.collect()
}

pub fn sample_numbers_around_symbols(input: &Vec<&str>) -> Vec<FoundNumber>
{
	let mut result = hashset!{};

	for row_index in 0..input.len()
	{
		let row = input[row_index];
		for column_index in 0..row.len()
		{
			let ch = row.chars().nth(column_index).unwrap();
			let is_digit = ('0'..='9').contains(&ch);

			if !is_digit && (ch != '.')
			{
				result.extend(
					sample_numbers_around(input, row_index, column_index));
			}
		}
	}

	result
		.into_iter()
		.collect()
}

pub fn solution(input: &Vec<&str>) -> u32
{
	sample_numbers_around_symbols(input)
		.iter()
		.map(|n| n.number)
		.sum()
}
