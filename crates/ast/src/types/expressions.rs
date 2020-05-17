use super::{BoxedNode, BoxedOptionalNode, BoxedVecNode, DeclarationType};

pub enum ExpressionType {
	StatementList(BoxedVecNode),
	Block(BoxedVecNode),
	If(BoxedNode, BoxedNode, BoxedOptionalNode),
	Declaration(DeclarationType, BoxedNode, BoxedOptionalNode),
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
	ImportDefault(BoxedNode, BoxedNode),
	ImportNamed(BoxedVecNode, BoxedNode),
}

impl Default for ExpressionType {
	fn default() -> Self {
		ExpressionType::StatementList(Box::new(Vec::new()))
	}
}
