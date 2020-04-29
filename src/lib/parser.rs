use super::tokens::Token;
use logos::Lexer;

pub async fn parser<'s>(ast_iter: Lexer<'s, Token>) {
	for node in ast_iter {
		println!("{:#?}", node);
	}
}

#[cfg(test)]
mod parser_tests {
	use super::*;

	#[test]
	fn sanity() {}
}
