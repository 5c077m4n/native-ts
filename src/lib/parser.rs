use ast::Node;
use lexer::js_token::JsToken;
use logos::{self, Lexer};
use std::io::{Error, ErrorKind, Result};

#[allow(dead_code)]
pub async fn lexer_to_ast(ast_iter: &mut Lexer<'_, JsToken>) -> Result<Box<Node>> {
	let root = Node::new();

	while let Some(token) = ast_iter.next() {
		match token {
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

#[cfg(test)]
mod parser_tests {
	use super::*;
	use logos::Logos;

	#[tokio::test]
	#[should_panic]
	async fn sanity() {
		let lex = &mut JsToken::lexer("console.log(12e3);");
		let _ = lexer_to_ast(lex).await.unwrap();

		assert!(false);
	}
}
