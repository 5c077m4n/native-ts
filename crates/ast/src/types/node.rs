#[derive(Debug, Default, PartialEq)]
pub struct Node {
	pub raw_value: String,
	pub node_type: String,
	pub file_path: String,
	pub column: usize,
	pub line: usize,
	pub children: Vec<Node>,
}

pub type BoxedNode = Box<Node>;
pub type BoxedOptionalNode = Box<Option<Node>>;
pub type BoxedVecNode = Box<Vec<Node>>;

impl Node {
	pub fn add(&mut self, child: Node) -> &Self {
		self.children.push(child);
		self
	}

	pub fn boxed() -> BoxedNode {
		let new_node: Self = Default::default();
		Box::new(new_node)
	}

	pub fn boxed_option() -> BoxedOptionalNode {
		let new_node: Self = Default::default();
		Box::new(Some(new_node))
	}

	pub fn boxed_vec() -> BoxedVecNode {
		let new_node_list: Vec<Self> = Vec::new();
		Box::new(new_node_list)
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
