use std::path::Path;
use aoc_2023::solution_from;
use aoc_2023::day12b::solution;

#[test]
fn example_is_correct()
{
	let input =
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = solution(&input);

	assert_eq!(actual, 525152);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-12");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 1);
}
