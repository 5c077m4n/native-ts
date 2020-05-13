#[derive(Debug, Default, PartialEq)]
pub struct Node {
	pub raw_value: String,
	pub node_type: String,
	pub file_path: String,
	pub column: usize,
	pub line: usize,
	pub children: Vec<Box<Node>>,
}

impl Node {
	pub fn add(&mut self, child: Box<Node>) -> &Self {
		self.children.push(child);
		self
	}

	pub fn new() -> Box<Self> {
		let new_node = Default::default();
		Box::new(new_node)
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
