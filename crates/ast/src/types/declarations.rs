use super::{BoxedNode, BoxedOptionalNode};

pub enum DeclarationType {
	Var(BoxedNode, BoxedOptionalNode),
	Let(BoxedNode, BoxedOptionalNode),
	Const(BoxedNode, BoxedNode),
}
