use super::ExpressionType;
use std::path::PathBuf;

#[derive(Default)]
pub struct Node {
	pub raw_value: Option<String>,
	pub node_type: ExpressionType,
	pub file_path: PathBuf,
	pub column: usize,
	pub line: usize,
}

pub type BoxedNode = Box<Node>;
pub type BoxedOptionalNode = Box<Option<Node>>;
pub type BoxedVecNode = Box<Vec<Node>>;

impl Node {
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
		let node = Node::boxed();
		let child_node = Node::boxed();

		match node.node_type {
			ExpressionType::StatementList(mut boxed_node) => {
				assert_eq!((*boxed_node).len(), 0);

				(*boxed_node).push(*child_node);
				assert_eq!((*boxed_node).len(), 1);
			}
			_ => unreachable!(),
		}
	}
}
