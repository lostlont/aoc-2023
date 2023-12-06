pub type Num = u32;

pub fn solution(input: &Vec<&str>) -> Num
{
	let [times, records] : [Vec<Num>; 2] = input
		.iter()
		.filter(|line| !line.is_empty())
		.map(|line| line
			.split_once(':')
			.expect(&format!("Line \"{line}\" can not be split at ':'!"))
			.1
			.split_whitespace()
			.map(|t| t
				.parse::<Num>()
				.expect(&format!("Value \"{t}\" can not be parsed as number!")))
			.collect::<Vec<_>>())
		.collect::<Vec<_>>()
		.try_into()
		.expect("Input can not be parsed as two lines!");

	Iterator
		::zip(
			times.iter(),
			records.iter())
		.map(|(&max_time, &record)| (0..max_time)
			.map(|charge_time| (max_time - charge_time) * charge_time)
			.filter(|&distance| distance > record)
			.count() as Num)
		.product()
}
