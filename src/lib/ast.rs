#![allow(dead_code)]

#[derive(Debug, PartialEq)]
pub struct Node {
	pub node_type: String,
	pub file_path: String,
	pub column: usize,
	pub line: usize,
	pub children: Vec<Node>,
}

impl Node {
	pub fn new() -> Self {
		Node {
			node_type: "Parent".to_owned(),
			file_path: "/".to_owned(),
			column: 0,
			line: 0,
			children: Vec::new(),
		}
	}

	pub fn add(&mut self, child: Node) -> &Self {
		self.children.push(child);
		self
	}
}

#[cfg(test)]
mod ast_tests {
	use super::*;

	#[test]
	fn sanity() {
		let mut node = Node::new();
		assert_eq!(node.children.len(), 0);

		let child_node = Node::new();
		node.add(child_node);
		assert_eq!(node.children.len(), 1);
	}
}
