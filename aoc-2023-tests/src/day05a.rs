use std::path::Path;
use aoc_2023::solution_from;
use aoc_2023::day05a::parse_mapping;
use aoc_2023::day05a::parse_seeds;
use aoc_2023::day05a::parse_values;
use aoc_2023::day05a::solution;
use aoc_2023::day05a::Mapping;

#[test]
fn parse_seeds_works()
{
	let mut actual = parse_seeds("seeds: 5 3 12")
		.collect::<Vec<_>>();
	actual.sort();

	let expected = vec![3, 5, 12];
	assert_eq!(actual, expected);
}

#[test]
fn parse_mapping_works()
{
	let actual = parse_mapping("5 50 3");

	let expected = Mapping
	{
		source_start: 50,
		destination_start: 5,
		length: 3,
	};
	assert_eq!(actual, expected);
}

#[test]
fn mapping_map_values_works()
{
	let input = vec![5, 3, 12, 6];
	let mapping = Mapping
	{
		source_start: 2,
		destination_start: 22,
		length: 4,
	};
	let mut actual = mapping
		.map_values(&input)
		.collect::<Vec<_>>();
	actual.sort();

	let mut expected = vec![Some(25), Some(23), None, None];
	expected.sort();
	assert_eq!(actual, expected);
}

#[test]
fn parse_values_maps_once()
{
	let input =
"seeds: 5 3 12

one map:
22 2 3
7 6 2
";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = parse_values(&input);

	let expected = vec![5, 23, 12];
	assert_eq!(actual, expected);
}

#[test]
fn parse_values_maps_twice_parallel()
{
	let input =
"seeds: 5 3 12

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

	let expected = vec![5, 23, 13];
	assert_eq!(actual, expected);
}

#[test]
fn parse_values_maps_twice_serial()
{
	let input =
"seeds: 5 3 12

one map:
22 2 3
7 6 2

two map:
11 21 3
";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = parse_values(&input);

	let expected = vec![5, 13, 12];
	assert_eq!(actual, expected);
}

#[test]
fn parse_values_does_not_map_same_category_multiple_times()
{
	let input =
"seeds: 5 3 12

one map:
22 2 3
42 22 3
";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = parse_values(&input);

	let expected = vec![5, 23, 12];
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

	assert_eq!(actual, 35);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-05");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 525792406);
}
