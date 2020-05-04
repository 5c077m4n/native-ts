use super::tokens::Token;
use logos::{self, Lexer};

#[allow(dead_code)]
pub async fn lexer_to_ast(ast_iter: &mut Lexer<'_, Token>) {
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
	#[allow(unused_imports)]
	use super::*;

	#[test]
	fn sanity() {}
}
