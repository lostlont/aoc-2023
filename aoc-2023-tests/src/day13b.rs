use std::path::Path;
use rstest::*;
use aoc_2023::solution_from;
use aoc_2023::day13a::is_mirrored_by_horizontal_line_at;
use aoc_2023::day13a::is_mirrored_by_vertical_line_at;
use aoc_2023::day13a::solution;
use aoc_2023::day13a::Image;

#[rstest]
#[case(0, 0, false)]
#[case(1, 0, false)]
#[case(2, 0, false)]
#[case(3, 0, false)]
#[case(4, 0, false)]
#[case(0, 1, false)]
#[case(1, 1, false)]
#[case(2, 1, false)]
#[case(3, 1, true)]
#[case(4, 1, false)]
fn is_mirrored_by_horizontal_line_at_finds_smudged(#[case] at: usize, #[case] diff: u32, #[case] expected: bool)
{
	let image = create_image(&[
		"##..",
		"#..#",
		".###",
		".#.#",
		"#..#",
	]);

	let actual = is_mirrored_by_horizontal_line_at(&image, at, diff);

	assert_eq!(actual, expected);
}

fn create_image(values: &[&str]) -> Image
{
	Image::new(
		&values.join(""),
		values[0].len(),
		values.len())
}

#[rstest]
#[case(0, 0, false)]
#[case(1, 0, false)]
#[case(2, 0, false)]
#[case(3, 0, false)]
#[case(4, 0, false)]
#[case(5, 0, false)]
#[case(6, 0, false)]
#[case(0, 1, false)]
#[case(1, 1, false)]
#[case(2, 1, false)]
#[case(3, 1, false)]
#[case(4, 1, true)]
#[case(5, 1, false)]
#[case(6, 1, false)]
fn is_mirrored_by_vertical_line_at_finds_smudged(#[case] at: usize, #[case] diff: u32, #[case] expected: bool)
{
	let image = create_image(&[
		".#.####",
		"..#..#.",
		"#.####.",
	]);

	let actual = is_mirrored_by_vertical_line_at(&image, at, diff);

	assert_eq!(actual, expected);
}

#[test]
fn example_is_correct()
{
	let input =
"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = solution(&input, 1);

	assert_eq!(actual, 400);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-13");
	let actual = solution_from(&path, |input| solution(input, 1));

	assert_eq!(actual, 31603);
}
