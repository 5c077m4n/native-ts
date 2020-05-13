use super::{BoxedNode, BoxedOptionalNode, BoxedVecNode};

pub enum ExpressionType {
	StatementList(BoxedVecNode),
	Block(BoxedVecNode),
	If(BoxedNode, BoxedNode, BoxedOptionalNode),
	Throw(BoxedNode),
	Try(
		BoxedNode,
		BoxedOptionalNode,
		BoxedOptionalNode,
		BoxedOptionalNode,
	),
	LoopFor(
		BoxedOptionalNode,
		BoxedOptionalNode,
		BoxedOptionalNode,
		BoxedOptionalNode,
	),
	LoopWhile(BoxedNode, BoxedOptionalNode),
	This,
	TypeOf(BoxedNode),
	InstanceOf(BoxedNode),
	UnaryOp(BoxedNode, BoxedNode),
	BinaryOp(BoxedNode, BoxedNode, BoxedNode),
}
