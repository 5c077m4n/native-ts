use ast::{import_lex_to_ast::import_tokens_to_ast, js_lex_to_ast::js_tokens_to_ast};
use lexer::{import_token::ImportToken, js_token::JsToken};
use logos::Logos;
use std::io::Result;

#[allow(dead_code)]
pub async fn parse(script: &str) -> Result<()> {
	let mut import_lex = ImportToken::lexer(script);
	let _import_ast = import_tokens_to_ast(&mut import_lex).await?;

	for token in import_lex.spanned() {
		println!("{:?}", token);
	}

	let mut js_lex = JsToken::lexer(script);
	let _js_ast = js_tokens_to_ast(&mut js_lex).await?;

	for token in js_lex.spanned() {
		println!("{:?}", token);
	}

	Ok(())
}

#[cfg(test)]
mod parser_tests {
	use super::*;

	#[tokio::test]
	#[should_panic(expected = "There was an error in parsing the input `😂` @ 0..4.")]
	async fn sanity() {
		parse("😂").await.unwrap();
	}
}
