use super::{BoxedNode, BoxedOptionalNode};

pub enum DeclarationTypes {
	Var(BoxedNode, BoxedOptionalNode),
	Let(BoxedNode, BoxedOptionalNode),
	Const(BoxedNode, BoxedNode),
}
