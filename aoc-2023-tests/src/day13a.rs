use std::path::Path;
use rstest::*;
use aoc_2023::solution_from;
use aoc_2023::day13a::is_mirrored_by_horizontal_line_at;
use aoc_2023::day13a::is_mirrored_by_vertical_line_at;
use aoc_2023::day13a::score;
use aoc_2023::day13a::solution;
use aoc_2023::day13a::Image;

#[test]
fn image_from_vec_str_works_with_one_image()
{
	let input = vec!
	[
		".#.",
		"..#",
	];

	let actual = Image::from(input.as_slice());

	let expected = Image::new(".#...#", 3, 2);
	assert_eq!(actual, expected);
}

#[test]
fn image_from_vec_str_works_with_more_images()
{
	let input = vec!
	[
		"..#",
		"...",
		"",
		".#.",
		"..#",
		"",
		"#.#",
		"###",
	];

	let actual = Image::from(&input[3..5]);

	let expected = Image::new(".#...#", 3, 2);
	assert_eq!(actual, expected);
}

#[rstest]
#[case(0, 0, Some('.'))]
#[case(1, 0, Some('#'))]
#[case(3, 0, None)]
#[case(0, 1, Some('.'))]
#[case(1, 1, Some('.'))]
#[case(2, 1, Some('#'))]
#[case(0, 2, None)]
fn image_at_works(#[case] x: usize, #[case] y: usize, #[case] expected: Option<char>)
{
	let subject = create_image(&[
		".#.",
		"..#",
	]);

	let actual = subject.at(x, y);

	assert_eq!(actual, expected);
}

#[rstest]
#[case(0, false)]
#[case(1, false)]
#[case(2, true)]
#[case(3, false)]
fn is_mirrored_by_horizontal_line_at_finds_normal_mirrored(#[case] at: usize, #[case] expected: bool)
{
	let image = create_image(&[
		"#..#",
		".###",
		".###",
		"#..#",
	]);

	let actual = is_mirrored_by_horizontal_line_at(&image, at, 0);

	assert_eq!(actual, expected);
}

#[rstest]
#[case(0, false)]
#[case(1, false)]
#[case(2, false)]
#[case(3, true)]
#[case(4, false)]
fn is_mirrored_by_horizontal_line_at_finds_offseted_mirrored(#[case] at: usize, #[case] expected: bool)
{
	let image = create_image(&[
		"##..",
		"#..#",
		".###",
		".###",
		"#..#",
	]);

	let actual = is_mirrored_by_horizontal_line_at(&image, at, 0);

	assert_eq!(actual, expected);
}

#[rstest]
#[case(0, false)]
#[case(1, false)]
#[case(2, true)]
#[case(3, false)]
#[case(4, false)]
fn is_mirrored_by_vertical_line_at_finds_normal_mirrored(#[case] at: usize, #[case] expected: bool)
{
	let image = create_image(&[
		".##.",
		"#..#",
	]);

	let actual = is_mirrored_by_vertical_line_at(&image, at, 0);

	assert_eq!(actual, expected);
}

#[rstest]
#[case(0, false)]
#[case(1, false)]
#[case(2, false)]
#[case(3, false)]
#[case(4, true)]
#[case(5, false)]
#[case(6, false)]
fn is_mirrored_by_vertical_line_at_finds_offseted_mirrored(#[case] at: usize, #[case] expected: bool)
{
	let image = create_image(&[
		".#.##.#",
		"..#..#.",
	]);

	let actual = is_mirrored_by_vertical_line_at(&image, at, 0);

	assert_eq!(actual, expected);
}

#[test]
fn score_finds_horizontal_mirror()
{
	let image = create_image(&[
		"##..",
		"#..#",
		".###",
		".###",
		"#..#",
	]);

	let actual = score(&image, 0);

	assert_eq!(actual, 300);
}

#[test]
fn score_finds_vertical_mirror()
{
	let image = create_image(&[
		".#.##.#",
		"..#..#.",
	]);

	let actual = score(&image, 0);

	assert_eq!(actual, 4);
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

	let actual = solution(&input, 0);

	assert_eq!(actual, 405);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-13");
	let actual = solution_from(&path, |input| solution(input, 0));

	assert_eq!(actual, 28895);
}

fn create_image(values: &[&str]) -> Image
{
	Image::new(
		&values.join(""),
		values[0].len(),
		values.len())
}
