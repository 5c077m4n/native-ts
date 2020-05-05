#![allow(dead_code)]

#[derive(Debug, PartialEq)]
pub struct Node<'n> {
	pub node_type: &'n str,
	pub file_path: &'n str,
	pub column: usize,
	pub line: usize,
	pub children: Vec<&'n Node<'n>>,
}

impl<'n> Node<'n> {
	pub fn new() -> Self {
		Node {
			node_type: "Parent",
			file_path: ".",
			column: 0,
			line: 0,
			children: Vec::new(),
		}
	}

	fn add(&mut self, child: &'n Node<'n>) -> &Self {
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
		node.add(&child_node);
		assert_eq!(node.children.len(), 1);
	}
}
