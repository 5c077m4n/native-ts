use super::tokens::Token;
use logos::{self, Lexer};

pub async fn lexer_to_ast<'s>(ast_iter: &mut Lexer<'s, Token>) {
	while let Some(token) = ast_iter.next() {
		match token {
			Token::Error => {
				eprintln!(
					"There wan an error in parsing {} in {:?}",
					ast_iter.slice(),
					ast_iter.span()
				);
				return;
			}
			_ => (),
		}
	}
}

#[cfg(test)]
mod parser_tests {
	use super::*;

	#[test]
	fn sanity() {}
}
