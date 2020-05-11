use ast::Node;
use lexer::Token;
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
				let new_node = Node {
					raw_value: String::new(),
					node_type: ast_iter.slice().to_owned(),
					file_path: "/".to_owned(),
					column: ast_iter.span().start,
					line: ast_iter.span().end,
					children: Vec::new(),
				};
				root.add(new_node);
			}
			Token::From => {
				let new_node = Node {
					raw_value: String::new(),
					node_type: ast_iter.slice().to_owned(),
					file_path: "/".to_owned(),
					column: ast_iter.span().start,
					line: ast_iter.span().end,
					children: Vec::new(),
				};
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
