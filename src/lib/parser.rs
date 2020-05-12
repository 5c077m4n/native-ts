use ast::Node;
use lexer::token::Token;
use logos::{self, Lexer};
use std::io::{Error, ErrorKind, Result};

#[allow(dead_code)]
pub async fn lexer_to_ast<'n>(ast_iter: &mut Lexer<'_, Token>) -> Result<Node> {
	let mut root: Node = Default::default();

	while let Some(token) = ast_iter.next() {
		match token {
			Token::Error => {
				return Err(Error::new(
					ErrorKind::InvalidInput,
					format!(
						"There was an error in parsing the input `{}` @ {:?}.",
						ast_iter.slice(),
						ast_iter.span()
					),
				));
			}
			Token::Import => {
				let mut new_node: Node = Default::default();
				new_node.file_path = "/".to_owned();
				root.add(new_node);
			}
			Token::From => {
				let mut new_node: Node = Default::default();
				new_node.file_path = "/".to_owned();
				root.add(new_node);
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
		let lex = &mut Token::lexer("console.log(12e3);");
		let _ = lexer_to_ast(lex).await.unwrap();

		assert!(false);
	}
}
