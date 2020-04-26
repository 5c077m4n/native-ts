struct Location {
	pub line: u32,
	pub column: u32,
}
pub struct Node {
	pub node_type: String,
	pub range: (usize, usize),
	pub location: Location,
	pub children: Vec<Node>,
}
