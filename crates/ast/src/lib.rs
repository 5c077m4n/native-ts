#![allow(dead_code)]

#[derive(Debug, Default, PartialEq)]
pub struct Node {
	pub raw_value: String,
	pub node_type: String,
	pub file_path: String,
	pub column: usize,
	pub line: usize,
	pub children: Vec<Node>,
}

impl Node {
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
		let mut node: Node = Default::default();
		assert_eq!(node.children.len(), 0);

		let child_node: Node = Default::default();
		node.add(child_node);
		assert_eq!(node.children.len(), 1);
	}
}
