use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use itertools::Itertools;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Node
{
	name: String,
}

#[derive(Debug, PartialEq)]
pub struct Graph
{
	nodes: HashSet<Rc<Node>>,
	edges: HashMap<Rc<Node>, (Rc<Node>, Rc<Node>)>,
}

impl Node
{
	pub fn new(name: &str) -> Self
	{
		Self
		{
			name: name.to_string(),
		}
	}

	pub fn get_name(&self) -> &str
	{
		&self.name
	}
}

impl Graph
{
	pub fn get_nodes(&self) -> &HashSet<Rc<Node>>
	{
		&self.nodes
	}

	pub fn get_edges(&self) -> &HashMap<Rc<Node>, (Rc<Node>, Rc<Node>)>
	{
		&self.edges
	}

	pub fn step(&self, from: &Node, direction: char) -> Rc<Node>
	{
		let (left, right) = self.edges[from].clone();
		match direction
		{
			'L' => left.clone(),
			'R' => right.clone(),
			_ => panic!("Invalid direction {direction}!"),
		}
	}
}

pub fn parse_edge(value: &str) -> (&str, &str, &str)
{
	let (from, to) = value
		.split_once('=')
		.expect(&format!("Line \"{value}\" could not be split at '='!"));

	let (left, right) = to
		.trim()
		.strip_prefix('(')
		.expect(&format!("To part \"{to}\" does not contain '(' at start!"))
		.strip_suffix(')')
		.expect(&format!("To part \"{to}\" does not contain ')' at end!"))
		.split_once(',')
		.expect(&format!("To part \"{to}\" does not contain ','!"));

	let from = from.trim();
	let left = left.trim();
	let right = right.trim();
	(from, left, right)
}

pub fn find_node(nodes: &HashSet<Rc<Node>>, name: &str) -> Rc<Node>
{
	nodes
		.iter()
		.find(|n| n.get_name() == name)
		.expect(&format!("Node with name \"{name}\" can not be found!"))
		.clone()
}

impl From<&Vec<&str>> for Graph
{
	fn from(value: &Vec<&str>) -> Self
	{
		let values = value
			.iter()
			.cloned()
			.skip(2)
			.map(parse_edge)
			.collect_vec();
		let nodes = values
			.iter()
			.map(|v| v.0)
			.map(|n| Rc::new(Node::new(n)))
			.collect::<HashSet<_>>();
		let edges = values
			.iter()
			.map(|v| (
				find_node(&nodes, v.0),
				(
					find_node(&nodes, v.1),
					find_node(&nodes, v.2)
				)))
			.collect::<HashMap<_, _>>();
		Self
		{
			nodes,
			edges,
		}
	}
}

pub fn navigate(graph: &Graph, from: Rc<Node>, instructions: &str) -> Rc<Node>
{
	let mut node = from;
	for direction in instructions.chars()
	{
		node = graph.step(&*node, direction);
	}
	node
}

pub fn solution(input: &Vec<&str>) -> u32
{
	let instructions = input[0];
	let graph = Graph::from(input);

	let mut node = find_node(&graph.nodes, "AAA");
	let mut count = 0;
	while node.get_name() != "ZZZ"
	{
		node = navigate(&graph, node, instructions);
		count += 1;
	}
	instructions.len() as u32 * count
}
