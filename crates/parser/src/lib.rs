use ast::{import_tokens_to_ast, js_tokens_to_ast};
use lexer::{JsToken, StaticImportToken};
use logos::Logos;
use std::io::Result;
mod tests;

pub async fn parse(script: &str) -> Result<()> {
	let mut import_lex = StaticImportToken::lexer(script);
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
