use super::Node;
use lexer::ImportToken;
use logos::Lexer;
use std::io::{Error, ErrorKind, Result};

pub async fn import_tokens_to_ast(ast_iter: &mut Lexer<'_, ImportToken>) -> Result<Box<Node>> {
	let root = Node::boxed();

	while let Some(token) = ast_iter.next() {
		match token {
			ImportToken::Import => {
				if let Some(ImportToken::Text) = ast_iter.next() {
					let _import_name: String = ast_iter.slice().parse::<String>().unwrap();
				}
				if let Some(ImportToken::BracketCurlyOpen) = ast_iter.next() {}
			}
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
	#[should_panic(expected = "Unknown token `console.log` @ 0..11.")]
	async fn sanity() {
		let mut lex = ImportToken::lexer("console.log(123);");
		let _ = import_tokens_to_ast(&mut lex).await.unwrap();

		unreachable!();
	}
}