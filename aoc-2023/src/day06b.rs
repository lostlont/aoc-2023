pub type Num = u64;

pub fn solution(input: &Vec<&str>) -> Num
{
	let [max_time, record] : [Num; 2] = input
		.iter()
		.filter(|line| !line.is_empty())
		.map(|line|
			{
				let chars = line
					.split_once(':')
					.expect(&format!("Line \"{line}\" can not be split at ':'!"))
					.1
					.chars()
					.filter(|c| c.is_ascii_digit());
				let text = String::from_iter(chars);
				text
					.parse::<Num>()
					.expect(&format!("Value \"{text}\" can not be parsed as number!"))
			})
		.collect::<Vec<_>>()
		.try_into()
		.expect("Input can not be parsed as two lines!");

	(0..max_time)
		.map(|charge_time| (max_time - charge_time) * charge_time)
		.filter(|&distance| distance > record)
		.count() as Num
}
