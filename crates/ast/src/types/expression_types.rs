use super::node::Node;

type BoxedNode = Box<Node>;
type OptionBoxedNode = Option<Box<Node>>;

pub enum ExpressionType {
	Block(OptionBoxedNode),
	If(BoxedNode, BoxedNode, OptionBoxedNode),
}
