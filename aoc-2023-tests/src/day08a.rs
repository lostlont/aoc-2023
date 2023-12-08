use std::collections::HashSet;
use std::path::Path;
use std::rc::Rc;
use maplit::hashset;
use rstest::rstest;
use aoc_2023::solution_from;
use aoc_2023::day08a::find_node;
use aoc_2023::day08a::navigate;
use aoc_2023::day08a::parse_edge;
use aoc_2023::day08a::solution;
use aoc_2023::day08a::Graph;
use aoc_2023::day08a::Node;

#[rstest]
#[case("A = (A, A)", ("A", "A", "A"))]
#[case(" ABC = ( CDE , EFG  ) ", ("ABC", "CDE", "EFG"))]
fn parse_edge_works(#[case] value: &str, #[case] expected: (&str, &str, &str))
{
	let actual = parse_edge(value);

	assert_eq!(actual, expected);
}

#[rstest]
#[case(
	hashset!{ Rc::new(Node::new("A")) },
	"A")]
#[case(
	hashset!{ Rc::new(Node::new("B")), Rc::new(Node::new("A")) },
	"A")]
fn find_node_works(#[case] nodes: HashSet<Rc<Node>>, #[case] name: &str)
{
	let actual = find_node(&nodes, name);

	assert_eq!(actual.get_name(), name);
}

#[test]
fn graph_from_str_works_with_empty()
{
	let input = vec![
		"LR",
		"",
	];
	let subject = Graph::from(&input);

	assert!(subject.get_nodes().is_empty());
	assert!(subject.get_edges().is_empty());
}

#[test]
fn graph_from_str_works_with_one_node()
{
	let input = vec![
		"LR",
		"",
		"A = (A, A)",
	];
	let subject = Graph::from(&input);

	let nodes = subject
		.get_nodes()
		.iter()
		.map(|n| n.get_name())
		.collect::<Vec<_>>();

	let edges = subject
		.get_edges()
		.iter()
		.map(|(from, (left, right))| (from.get_name(), (left.get_name(), right.get_name())))
		.collect::<Vec<_>>();

	assert_eq!(nodes, vec!["A"]);
	assert_eq!(edges, vec![("A", ("A", "A"))]);
}

#[test]
fn graph_from_str_works_with_multiple_nodes()
{
	let input = vec![
		"LR",
		"",
		"ABC = (DEF, GHI)",
		"DEF = (ABC, GHI)",
		"GHI = (ABC, DEF)",
	];
	let subject = Graph::from(&input);

	let mut actual_nodes = subject
		.get_nodes()
		.iter()
		.map(|n| n.get_name())
		.collect::<Vec<_>>();
	actual_nodes.sort();

	let mut actual_edges = subject
		.get_edges()
		.iter()
		.map(|(from, (left, right))| (from.get_name(), (left.get_name(), right.get_name())))
		.collect::<Vec<_>>();
	actual_edges.sort();

	let mut expected_nodes = vec!["ABC", "DEF", "GHI"];
	expected_nodes.sort();
	assert_eq!(actual_nodes, expected_nodes);

	let mut expected_edges = vec![
		("ABC", ("DEF", "GHI")),
		("DEF", ("ABC", "GHI")),
		("GHI", ("ABC", "DEF")),
	];
	expected_edges.sort();
	assert_eq!(actual_edges, expected_edges);
}

#[rstest]
#[case("DEF", 'L', "ABC")]
#[case("DEF", 'R', "GHI")]
fn graph_step_works(#[case] from: &str, #[case] direction: char, #[case] expected: &str)
{
	let input = vec![
		"LR",
		"",
		"ABC = (DEF, GHI)",
		"DEF = (ABC, GHI)",
		"GHI = (ABC, DEF)",
	];
	let graph = Graph::from(&input);
	let node = find_node(graph.get_nodes(), from);

	let actual = graph.step(&node, direction);
	let actual = actual.get_name();

	assert_eq!(actual, expected);
}

#[rstest]
#[case("DEF", "LL", "DEF")]
#[case("DEF", "LR", "GHI")]
#[case("DEF", "RL", "ABC")]
#[case("DEF", "RR", "DEF")]
fn navigate_works(#[case] from: &str, #[case] instructions: &str, #[case] expected: &str)
{
	let input = vec![
		"LR",
		"",
		"ABC = (DEF, GHI)",
		"DEF = (ABC, GHI)",
		"GHI = (ABC, DEF)",
	];
	let graph = Graph::from(&input);
	let node = find_node(graph.get_nodes(), from);

	let actual = navigate(&graph, node, instructions);
	let actual = actual.get_name();

	assert_eq!(actual, expected);
}

#[test]
fn example_1_is_correct()
{
	let input =
"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = solution(&input);

	assert_eq!(actual, 2);
}

#[test]
fn example_2_is_correct()
{
	let input =
"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
	let input = input
		.split('\n')
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let actual = solution(&input);

	assert_eq!(actual, 6);
}

#[test]
fn solution_is_correct()
{
	let path = Path::new("../aoc-2023/input-08");
	let actual = solution_from(&path, solution);

	assert_eq!(actual, 16897);
}
