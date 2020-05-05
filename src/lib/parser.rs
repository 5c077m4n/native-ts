use super::{ast::Node, tokens::Token};
use logos::{self, Lexer};

#[allow(dead_code)]
pub async fn lexer_to_ast<'n>(ast_iter: &mut Lexer<'_, Token>) -> Result<Node<'n>, &'n str> {
	let root = Node::new();

	while let Some(token) = ast_iter.next() {
		match token {
			Token::Error => {
				eprintln!(
					"There wan an error parsing {} in {:?}",
					ast_iter.slice(),
					ast_iter.span()
				);
				return Err("There was an error in parsing the input.");
			}
			Token::Import => (),
			Token::From => (),
			_ => panic!("Unknown token."),
		}
	}

	Ok(root)
}

#[cfg(test)]
mod parser_tests {
	use super::*;
	use logos::Logos;

	#[tokio::test]
	async fn sanity() {
		let lex = &mut Token::lexer("12e3");
		let lex = lexer_to_ast(lex).await;
		assert_eq!(lex, Err("There was an error in parsing the input."));
	}
}
