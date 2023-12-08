use std::rc::Rc;
use itertools::Itertools;
use super::day08a::navigate;
use super::day08a::Graph;
use super::day08a::Node;

pub fn measure_cycle(graph: &Graph, instructions: &str, from: Rc<Node>) -> (u32, u32)
{
	let mut node = from;
	let mut constant_length = 0;
	while !node.get_name().ends_with('Z')
	{
		node = navigate(graph, node, instructions);
		constant_length += 1;
	}

	let mut cycle_length = 0;
	let mut run = true;
	while run
	{
		node = navigate(graph, node, instructions);
		cycle_length += 1;
		run = !node.get_name().ends_with('Z');
	}

	(constant_length, cycle_length)
}

pub fn solution(input: &Vec<&str>) -> u32
{
	let instructions = input[0];
	let graph = Graph::from(input);

	let nodes = graph.get_nodes()
		.iter()
		.cloned()
		.filter(|n| n
			.get_name()
			.ends_with('A'))
		.collect_vec();

	for node in nodes
	{
		let (constant_length, cycle_length) = measure_cycle(&graph, instructions, node);
		println!("::PL {constant_length} {cycle_length}");
	}

	0
}
