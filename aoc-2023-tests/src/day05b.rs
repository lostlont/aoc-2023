use std::path::Path;
use aoc_2023::solution_from;
use aoc_2023::day05b::map_ranges;
use aoc_2023::day05b::parse_seeds;
use aoc_2023::day05b::parse_values;
//use aoc_2023::day05b::solution;

#[test]
fn parse_seeds_works()
{
	let actual = parse_seeds("seeds: 5 3 12 4");

	let expected = vec![5..5+3, 12..12+4];
	assert_eq!(actual, expected);
}

#[test]
fn parse_values_maps_once()
{
	let input =
"seeds: 5 3 12 4

one map:
22 2 3
7 6 2
";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = parse_values(&input);

	let expected = vec![5, 7, 8, 12, 13, 14, 15];
	assert_eq!(actual, expected);
}

#[test]
fn map_ranges_works()
{
	let input = vec![(5..8), (12..17)];
	let mapping = Mapping
	{
		source_start: 7,
		destination_start: 17,
		length: 3,
	};
	let mut actual = map_ranges(&input, &mapping)
		.collect::<Vec<_>>();
	actual.sort();

	let mut expected = vec![(5..7), (12..18)]; // TODO
	expected.sort();
	assert_eq!(actual, expected);
}

/*
#[test]
fn parse_values_maps_twice_parallel()
{
	let input =
"seeds: 5 3 12 4

one map:
22 2 3
7 6 2

two map:
13 12 1
";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = parse_values(&input);

	let expected = vec![5, 7, 8, 13, 13, 14, 15];
	assert_eq!(actual, expected);
}

#[test]
fn parse_values_maps_twice_serial()
{
	let input =
"seeds: 5 3 12 4

one map:
22 2 3
7 6 2

two map:
37 7 1
";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = parse_values(&input);

	let expected = vec![5, 37, 8, 12, 13, 14, 15];
	assert_eq!(actual, expected);
}

#[test]
fn parse_values_does_not_map_same_category_multiple_times()
{
	let input =
"seeds: 5 3 12 4

one map:
26 6 2
46 26 2
";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = parse_values(&input);

	let expected = vec![5, 26, 27, 12, 13, 14, 15];
	assert_eq!(actual, expected);
}

#[test]
fn example_is_correct()
{
	let input =
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = solution(&input);

	assert_eq!(actual, 46);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-05");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 1);
}
*/
