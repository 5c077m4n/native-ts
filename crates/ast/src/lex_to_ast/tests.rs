#![cfg(test)]

use super::*;
use lexer::{JsToken, StaticImportToken};
use logos::Logos;

mod js_ast_tests {
	use super::*;

	#[tokio::test]
	#[should_panic(expected = "Unknown token `import` @ 0..6.")]
	async fn sanity() {
		let mut lex = JsToken::lexer("import { fn1 } from 'no/path/file.ts';");
		let _ = js_tokens_to_ast(&mut lex).await.unwrap();

		unreachable!();
	}
}

mod import_ast_tests {
	use super::*;

	#[tokio::test]
	#[should_panic(expected = "Unknown token `console.log` @ 0..11.")]
	async fn sanity() {
		let mut lex = StaticImportToken::lexer("console.log(123);");
		let _ = import_tokens_to_ast(&mut lex).await.unwrap();

		unreachable!();
	}
}
