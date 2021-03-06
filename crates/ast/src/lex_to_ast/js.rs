use super::Node;
use lexer::JsToken;
use logos::Lexer;
use std::io::{Error, ErrorKind, Result};

pub async fn js_tokens_to_ast(ast_iter: &mut Lexer<'_, JsToken>) -> Result<Box<Node>> {
	let root = Node::boxed();

	while let Some(token) = ast_iter.next() {
		match token {
			JsToken::Const => (),
			JsToken::Error => {
				return Err(Error::new(
					ErrorKind::InvalidInput,
					format!(
						"There was an error in parsing the input `{}` @ {:?}.",
						ast_iter.slice(),
						ast_iter.span()
					),
				));
			}
			_ => {
				return Err(Error::new(
					ErrorKind::InvalidInput,
					format!(
						"Unknown token `{}` @ {:?}.",
						ast_iter.slice(),
						ast_iter.span()
					),
				))
			}
		}
	}

	Ok(root)
}
