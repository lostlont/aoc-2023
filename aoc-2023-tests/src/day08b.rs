use std::path::Path;
use aoc_2023::day08a::find_node;
use aoc_2023::solution_from;
use aoc_2023::day08a::Graph;
use aoc_2023::day08b::measure_cycle;
use aoc_2023::day08b::solution;

#[test]
fn measure_cycle_works()
{
	let input =
"LR

A = (B, X)
B = (X, C)
C = (D, X)
D = (X, Z)
Z = (B, X)
X = (X, X)";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let instructions = input[0];
	let graph = Graph::from(&input);
	let node = find_node(graph.get_nodes(), "A");

	let actual = measure_cycle(&graph, instructions, node);

	let expected = (2, 2);
	assert_eq!(actual, expected);
}
/*
#[test]
fn example_is_correct()
{
	let input =
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = solution(&input);

	assert_eq!(actual, 6);
}
*/
/*
#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-08");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 1);
}
*/
