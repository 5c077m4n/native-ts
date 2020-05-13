use super::node::Node;
use lexer::import_token::ImportToken;
use logos::Lexer;
use std::io::{Error, ErrorKind, Result};

pub async fn import_tokens_to_ast(ast_iter: &mut Lexer<'_, ImportToken>) -> Result<Box<Node>> {
	let root = Node::new();

	while let Some(token) = ast_iter.next() {
		match token {
			ImportToken::Import => (),
			ImportToken::From => (),
			ImportToken::Text => (),
			ImportToken::Error => {
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
		let mut lex = ImportToken::lexer("console.log(123);");
		let _ = import_tokens_to_ast(&mut lex).await.unwrap();

		unreachable!();
	}
}
