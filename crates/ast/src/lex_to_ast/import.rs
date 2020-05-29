use super::Node;
use lexer::StaticImportToken;
use logos::Lexer;
use std::io::{Error, ErrorKind, Result};

pub async fn import_tokens_to_ast(
	ast_iter: &mut Lexer<'_, StaticImportToken>,
) -> Result<Box<Node>> {
	let root = Node::boxed();

	while let Some(token) = ast_iter.next() {
		match token {
			StaticImportToken::Import => {
				let next_token = ast_iter.next();

				if let Some(StaticImportToken::Text) = next_token {
					let _import_name: String = ast_iter.slice().parse::<String>().unwrap();
				} else if let Some(StaticImportToken::BracketCurlyOpen) = next_token {
					let _import_names: Vec<&str> = ast_iter
						.slice()
						.parse::<String>()
						.unwrap()
						.split(",")
						.collect();
				} else {
					return Err(Error::new(
						ErrorKind::InvalidInput,
						format!(
							"There was an invalid input after the import keyword `{}` @ {:?}.",
							ast_iter.slice(),
							ast_iter.span()
						),
					));
				}
			}
			StaticImportToken::Error => {
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
